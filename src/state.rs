pub mod database;
pub mod logger;
pub mod netspots;
pub mod webhooks;

use crate::state::webhooks::WebhookManager;
use crate::structures::statistics::Message;

use crate::state::logger::message_printer;
use crate::tasks::RunChecker;
use database::Database;
use netspots::NetspotManager;
use std::path::PathBuf;
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
    pub async fn new() -> Result<NetspotControlState, String> {
        // Get database path from environment
        let database_file = match env::var("DB_FILE_PATH") {
            Ok(path) => path,
            Err(_) => {
                return Err("DB_FILE_PATH environment variable must be set".to_string());
            }
        };

        // Ensure that path to database exists
        match PathBuf::from(database_file.as_str()).parent() {
            None => {
                return Err("Invalid DB_FILE_PATH environment variable".to_string());
            }
            Some(path) => {
                if let Err(err) = fs::create_dir_all(path) {
                    return Err(format!("Could not create database path: {}", err));
                }
            }
        }

        // Forward path to alternative constructor
        Self::new_with_db_url(database_file.as_str()).await
    }

    pub async fn new_with_db_url(database_url: &str) -> Result<NetspotControlState, String> {
        // Create channels for broadcasting data and alarm messages
        let (messages_tx, _) = broadcast::channel::<Message>(16);

        // Create channel for letting worker threads to know when to stop
        let (run_tx, _) = watch::channel(true);

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

        // Database has worker task for writing messages to the database.
        let database = Database::new(
            database_url,
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
