mod net;

use crate::api_v1::testing::TestAlarmMessage;
use crate::state::netspots::net::SocketUse;
use crate::structures::configuration::{NetspotConfig, NetspotConfigMap};
use crate::structures::statistics::{AlarmMessage, Message, MessageType};
use crate::structures::status::{ProcessStatus, Status, Statuses};
use crate::tasks::RunChecker;

use nix::sys::signal;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use rocket::warn;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, io};
use tokio::process::{Child, Command};
use tokio::sync::{broadcast, oneshot, RwLock};
use tokio::time;

// Configuration id is mapped to process handler
type Netspots = HashMap<i32, NetspotProcess>;

pub enum NetspotManagerError {
    NotFound,
}

pub struct NetspotManager {
    data_path: PathBuf,
    message_tx: Mutex<broadcast::Sender<Message>>,
    netspots_lock: RwLock<Netspots>,
}

impl NetspotManager {
    pub async fn new(
        data_path: &Path,
        configurations: NetspotConfigMap,
        message_tx: broadcast::Sender<Message>,
        run_checker: RunChecker,
    ) -> Result<NetspotManager, String> {
        net::start_listener_task(
            data_path,
            SocketUse::Alarm,
            message_tx.clone(),
            run_checker.clone(),
        )?;
        net::start_listener_task(data_path, SocketUse::Data, message_tx.clone(), run_checker)?;
        let manager = NetspotManager {
            data_path: PathBuf::from(data_path),
            message_tx: Mutex::new(message_tx),
            netspots_lock: RwLock::new(Netspots::new()),
        };
        manager.update_all(configurations).await?;
        Ok(manager)
    }

    pub fn send_test_alarm(&self, test_alarm: TestAlarmMessage) -> bool {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;
        let full_alarm_message = AlarmMessage {
            time,
            name: test_alarm.name,
            series: "TEST ALARM".to_string(),
            stat: test_alarm.stat,
            status: test_alarm.status,
            value: test_alarm.value,
            probability: test_alarm.probability,
            code: 1,
            msg_type: MessageType::Alarm,
        };
        let message_tx = self.message_tx.lock().unwrap();
        message_tx
            .send(Message::Alarm(Box::new(full_alarm_message)))
            .is_ok()
    }

    pub async fn restart_all(&self) {
        self.stop_all().await;
        self.start_all().await;
    }

    pub async fn restart_by_id(&self, id: i32) -> Result<Status, NetspotManagerError> {
        self.stop_by_id(id).await?;
        self.start_by_id(id).await?;
        self.status_by_id(id).await
    }

    pub async fn start_all(&self) {
        let mut netspots = self.netspots_lock.write().await;
        for (id, process) in netspots.iter_mut() {
            if let Err(err) = process.start() {
                warn!("Could not start process {}: {}", id, err.to_string());
            }
        }
    }

    pub async fn start_by_id(&self, id: i32) -> Result<Status, NetspotManagerError> {
        // Using scope to remove write lock before reading status result
        {
            let mut netspots = self.netspots_lock.write().await;
            if let Some(process) = netspots.get_mut(&id) {
                if let Err(err) = process.start() {
                    warn!("Could not start process {}: {}", id, err.to_string());
                }
            }
        }
        self.status_by_id(id).await
    }

    pub async fn status_all(&self) -> Statuses {
        let netspots = self.netspots_lock.read().await;
        let mut statuses = Statuses::new();
        for process in netspots.values() {
            statuses.push(process.status());
        }
        statuses
    }

    pub async fn status_by_id(&self, id: i32) -> Result<Status, NetspotManagerError> {
        let netspots = self.netspots_lock.read().await;
        if let Some(process) = netspots.get(&id) {
            return Ok(process.status());
        }
        Err(NetspotManagerError::NotFound)
    }

    pub async fn stop_all(&self) {
        let mut netspots = self.netspots_lock.write().await;
        for (id, process) in netspots.iter_mut() {
            if let Err(err) = process.stop().await {
                warn!("Error while stopping process {}: {}", id, err.to_string());
            }
        }
    }

    pub async fn stop_by_id(&self, id: i32) -> Result<Status, NetspotManagerError> {
        // Using scope to remove write lock before reading status result
        {
            let mut netspots = self.netspots_lock.write().await;
            if let Some(process) = netspots.get_mut(&id) {
                if let Err(err) = process.stop().await {
                    warn!("Error while stopping process {}: {}", id, err.to_string());
                }
            }
        }
        self.status_by_id(id).await
    }

    pub async fn update_all(&self, configurations: NetspotConfigMap) -> Result<(), String> {
        // Lock for writing
        let mut netspots = self.netspots_lock.write().await;

        // Remove processes that are no longer in the database
        netspots.retain(|id, _| configurations.contains_key(id));

        // Create and update entries
        for (id, config) in configurations {
            match netspots.entry(id) {
                Entry::Occupied(entry) => {
                    entry.into_mut().set_config(config);
                }
                Entry::Vacant(entry) => {
                    entry.insert(NetspotProcess::from(&self.data_path, id, config));
                }
            };
        }

        Ok(())
    }
}

pub struct NetspotProcess {
    config: NetspotConfig,
    data_path: String,
    id: i32,
    process: Option<Child>,
    toml_file_path: String,
}

impl NetspotProcess {
    fn from(data_path: &Path, id: i32, config: NetspotConfig) -> NetspotProcess {
        let mut toml_file_path = PathBuf::from(data_path);
        toml_file_path.push(format!("netspot_{id}.toml"));
        NetspotProcess {
            config,
            data_path: String::from(data_path.to_str().expect("valid str")),
            id,
            process: None,
            toml_file_path: String::from(toml_file_path.to_str().expect("valid str")),
        }
    }

    fn toml_file_path(&self) -> &str {
        &self.toml_file_path
    }

    fn process_status(&self) -> ProcessStatus {
        match self.process {
            None => {
                if !self.config.configuration.enabled {
                    ProcessStatus::Disabled
                } else {
                    ProcessStatus::Stopped
                }
            }
            Some(_) => ProcessStatus::Running,
        }
    }

    fn set_config(&mut self, config: NetspotConfig) {
        self.config = config;
    }

    fn start(&mut self) -> Result<(), io::Error> {
        if self.process_status() == ProcessStatus::Running
            || self.process_status() == ProcessStatus::Disabled
        {
            return Ok(());
        }

        fs::write(&self.toml_file_path, self.config.make_toml(&self.data_path))?;

        match Command::new("netspot")
            .args(["run", "-c", &self.toml_file_path])
            .spawn()
        {
            Ok(process) => {
                self.process = Some(process);
                println!("Netspot configuration {} started.", self.id);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn status(&self) -> Status {
        Status {
            id: self.id,
            name: self.config.configuration.name.clone(),
            status: self.process_status(),
        }
    }

    async fn stop(&mut self) -> Result<(), io::Error> {
        if self.process_status() != ProcessStatus::Running {
            return Ok(());
        }
        if let Some(mut process) = self.process.take() {
            // Try to terminate netspot with SIGINT
            if let Some(id) = process.id() {
                if let Err(err) = signal::kill(Pid::from_raw(id as i32), Signal::SIGINT) {
                    eprintln!(
                        "Unexpected error: Could not send SIGINT for netspot process: {}",
                        err
                    );
                }
            }

            // We allow 5 seconds for the netspot to shutdown correctly
            let (tx, rx) = oneshot::channel::<()>();
            let timeout = time::timeout(Duration::from_secs(5), rx);
            tokio::select! {
                _ = process.wait() => {
                    drop(tx);
                    println!("Netspot configuration {} stopped.", self.id);
                }
                _ = timeout => {
                    eprintln!("Netspot configuration {} did not stop correctly. \
                               Terminating the netspot process.", self.id);
                    let _ = process.kill().await ;
                }
            }
        }
        assert!(self.process.is_none());
        fs::remove_file(self.toml_file_path())?;
        Ok(())
    }
}
