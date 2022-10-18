pub mod database;
pub mod netspots;

use crate::structures::statistics::Message;
use database::Database;
use netspots::NetspotManager;
use tokio::sync::{broadcast, mpsc};

// Netspot Control State
//--------------------------------------------------------------------------------------------------

pub struct NetspotControlState {
    pub netspots: NetspotManager,
    pub database: Database,

    // Broadcasting shutdown signal to all worker tasks
    shutdown_request_tx: broadcast::Sender<()>,
}

impl NetspotControlState {
    pub fn new(shutdown_complete_tx: mpsc::Sender<()>) -> Result<NetspotControlState, String> {
        println!("NetspotControl started.");

        // Create channels for broadcasting data and alarm messages
        let (messages_tx, _) = broadcast::channel::<Message>(16);

        // Printing received messages on debug build
        #[cfg(debug_assertions)]
        tokio::spawn(message_printer(messages_tx.subscribe()));

        // Create shutdown request channel
        let (shutdown_request_tx, _) = broadcast::channel(1);

        // Database has worker task for writing messages to the database.
        let database = Database::new(
            messages_tx.subscribe(),
            shutdown_request_tx.subscribe(),
            shutdown_complete_tx.clone(),
        )?;

        // Netspot manager has worker tasks for receiving messages from netspot processes
        let netspots = NetspotManager::new(
            database.get_configurations()?,
            messages_tx,
            shutdown_request_tx.subscribe(),
            shutdown_complete_tx,
        )?;

        // Start all netspot processes we can
        netspots.start_all();
        Ok(NetspotControlState {
            database,
            netspots,
            shutdown_request_tx,
        })
    }

    pub async fn shutdown(&self) {
        println!("NetspotControl shutdown requested...");
        // Stop processes and then send shutdown request for worker tasks
        self.netspots.stop_all();
        let _ = self.shutdown_request_tx.send(());
        // The main function waits for tasks to exit using the mpsc channel
    }
}

#[cfg(debug_assertions)]
async fn message_printer(mut message_rx: broadcast::Receiver<Message>) {
    use termion::{color, style};
    while let Ok(message) = message_rx.recv().await {
        if let Ok(json) = message.to_json() {
            match message {
                Message::Alarm(_) => {
                    println!(
                        "{}Alarm: {}{}",
                        color::Fg(color::Yellow),
                        json,
                        style::Reset
                    );
                }
                Message::Data(_) => {
                    println!("Data: {json}");
                }
            }
        }
    }
}
