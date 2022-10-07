use crate::structures::configuration::{NetspotConfig, NetspotConfigMap};
use crate::structures::status::{ProcessStatus, Status, Statuses};
use rocket::warn;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;
use std::{fs, io};

// Configuration id is mapped to process handler
type Netspots = HashMap<i32, NetspotProcess>;

pub struct NetspotManager {
    netspots_lock: RwLock<Netspots>,
}

impl NetspotManager {
    pub fn new(configurations: NetspotConfigMap) -> Result<NetspotManager, String> {
        let manager = NetspotManager {
            netspots_lock: RwLock::new(Netspots::new()),
        };
        manager.update_all(configurations)?;
        Ok(manager)
    }

    pub fn start_all(&self) {
        let mut netspots = self.netspots_lock.write().unwrap();
        for (id, process) in netspots.iter_mut() {
            if let Err(err) = process.start() {
                warn!("Could not start process {}: {}", id, err.to_string());
            }
        }
    }

    pub fn statuses(&self) -> Statuses {
        // Lock for reading
        let netspots = self.netspots_lock.read().unwrap();

        let mut statuses = Statuses::new();
        for (id, process) in &*netspots {
            statuses.push(Status {
                id: id.clone(),
                name: process.name().to_string(),
                status: process.status(),
            });
        }
        statuses
    }

    pub fn stop_all(&self) {
        let mut netspots = self.netspots_lock.write().unwrap();
        for (id, process) in netspots.iter_mut() {
            if let Err(err) = process.stop() {
                warn!("Error while stopping process {}: {}", id, err.to_string());
            }
        }
    }

    pub fn update_all(&self, configurations: NetspotConfigMap) -> Result<(), String> {
        // Lock for writing
        let mut netspots = self.netspots_lock.write().unwrap();

        // Remove processes that are no longer in the database
        netspots.retain(|id, _| configurations.contains_key(id));

        // Create and update entries
        for (id, config) in configurations {
            match netspots.entry(id) {
                Entry::Occupied(entry) => {
                    entry.into_mut().set_config(config);
                }
                Entry::Vacant(entry) => {
                    entry.insert(NetspotProcess::from(id, config));
                }
            };
        }

        Ok(())
    }
}

pub struct NetspotProcess {
    id: i32,
    config: NetspotConfig,
}

impl NetspotProcess {
    fn from(id: i32, config: NetspotConfig) -> NetspotProcess {
        NetspotProcess { id, config }
    }

    fn toml_file_path(&self) -> String {
        format!("/tmp/netspot_{}.toml", self.id)
    }

    fn name(&self) -> &str {
        &self.config.configuration.name[..]
    }

    fn set_config(&mut self, config: NetspotConfig) {
        self.config = config;
    }

    fn start(&mut self) -> Result<(), io::Error> {
        if self.status() == ProcessStatus::Running || self.status() == ProcessStatus::Disabled {
            return Ok(());
        }
        fs::write(self.toml_file_path(), self.config.make_toml())?;

        warn!("TODO: {} line {}: Start process", file!(), line!());
        Ok(())
    }

    fn status(&self) -> ProcessStatus {
        if !self.config.configuration.enabled {
            return ProcessStatus::Disabled;
        }
        // TODO: Implement checking for if process is actually running or not
        let path = PathBuf::from(self.toml_file_path());
        return match path.exists() {
            true => ProcessStatus::Running,
            false => ProcessStatus::Stopped,
        };
    }

    fn stop(&mut self) -> Result<(), io::Error> {
        if self.status() != ProcessStatus::Running {
            return Ok(());
        }
        fs::remove_file(self.toml_file_path())?;
        warn!("TODO: {} line {}: Stop process", file!(), line!());
        Ok(())
    }
}

impl Drop for NetspotProcess {
    fn drop(&mut self) {
        if let Err(err) = self.stop() {
            warn!(
                "Error while stopping process {}: {}",
                self.id,
                err.to_string()
            );
        }
    }
}
