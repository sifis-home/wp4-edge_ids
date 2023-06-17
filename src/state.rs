pub mod database;
pub mod dht;
pub mod logger;
pub mod netspots;
pub mod webhooks;

use crate::state::webhooks::WebhookManager;
use crate::structures::statistics::Message;

use crate::state::dht::dht_message_sender;
use crate::state::logger::message_printer;
use crate::tasks::RunChecker;
use database::Database;
use netspots::NetspotManager;
use std::path::Path;
use std::{env, fs};
use tokio::sync::{broadcast, watch};

// Netspot Control State
//--------------------------------------------------------------------------------------------------

pub struct NetspotControlState {
    pub netspots: NetspotManager,
    pub database: Database,
    pub webhooks: WebhookManager,

    /// Signaling worker tasks to stop when shutdown is called
    run_tx: watch::Sender<bool>,
}

impl NetspotControlState {
    pub async fn new(dht: Option<String>) -> Result<NetspotControlState, String> {
        // Get database path from environment
        let database_path = match env::var("DB_FILE_PATH") {
            Ok(path) => path,
            Err(_) => {
                return Err("DB_FILE_PATH environment variable must be set".to_string());
            }
        };

        // Forward data to customized constructor
        Self::new_customized(dht, Path::new("/tmp"), Path::new(&database_path)).await
    }

    pub async fn new_customized(
        dht: Option<String>,
        runtime_path: &Path,
        database_path: &Path,
    ) -> Result<NetspotControlState, String> {
        // Ensure that data path exists
        if let Err(err) = fs::create_dir_all(runtime_path) {
            return Err(format!("Could not create runtime path: {}", err));
        }

        // Ensure that database path exists
        match database_path.parent() {
            None => {
                return Err(format!("Invalid database path: {database_path:?}"));
            }
            Some(path) => {
                if let Err(err) = fs::create_dir_all(path) {
                    return Err(format!("Could not create database path: {}", err));
                }
            }
        }

        // Create channel for letting worker threads to know when to stop
        let (run_tx, _) = watch::channel(true);

        // Create channels for broadcasting data and alarm messages
        let (messages_tx, _) = broadcast::channel::<Message>(16);

        // Check if SHOW_NETSPOT_MESSAGES environment variable is set
        if let Ok(value) = env::var("SHOW_NETSPOT_MESSAGES") {
            if let Ok(value) = value.parse::<i32>() {
                if value != 0 {
                    // Printing received messages to stdout
                    tokio::spawn(message_printer(
                        messages_tx.subscribe(),
                        RunChecker::new(run_tx.subscribe()),
                    ));
                }
            }
        };

        // Sending messages to DHT REST API
        if let Some(api_url) = dht {
            tokio::spawn(dht_message_sender(
                api_url,
                get_ip_addresses()?,
                messages_tx.subscribe(),
                RunChecker::new(run_tx.subscribe()),
            ));
        }

        // Database has worker task for writing messages to the database.
        let database = Database::new(
            database_path.to_str().ok_or("Invalid DB path")?,
            messages_tx.subscribe(),
            RunChecker::new(run_tx.subscribe()),
        )?;

        // Webhook manager has worker task for sending messages.
        let webhooks = WebhookManager::new(
            database.get_webhooks()?,
            messages_tx.subscribe(),
            RunChecker::new(run_tx.subscribe()),
        );

        // Netspot manager has worker tasks for receiving messages from netspot processes
        let netspots = NetspotManager::new(
            runtime_path,
            database.get_configurations()?,
            messages_tx,
            RunChecker::new(run_tx.subscribe()),
        )
        .await?;

        // Start all netspot processes we can
        netspots.start_all().await;

        // Complete
        println!("NetspotControlState started.");
        Ok(NetspotControlState {
            database,
            netspots,
            webhooks,
            run_tx,
        })
    }

    pub async fn shutdown(&self) {
        println!("NetspotControlState shutdown requested...");

        // Request all netspot processes to stop
        self.netspots.stop_all().await;

        // Send signal to stop workers and wait them to stop
        if self.run_tx.send(false).is_ok() {
            self.run_tx.closed().await;
        }

        println!("NetspotControlState shutdown completed.")
    }
}

fn get_ip_addresses() -> Result<Vec<String>, String> {
    let mut addresses = Vec::new();
    let device_list = pcap::Device::list().map_err(|e| e.to_string())?;
    for device in device_list {
        for address in device.addresses {
            let addr = &address.addr;
            if addr.is_loopback() || addr.is_multicast() || addr.is_unspecified() {
                // Skipping over these
                continue;
            }
            if address.addr.is_ipv4() || address.addr.is_ipv6() {
                // These we use
                addresses.push(addr.to_string());
            }
        }
    }
    Ok(addresses)
}
