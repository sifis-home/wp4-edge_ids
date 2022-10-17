pub mod database;
pub mod netspots;

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

        // Create shutdown request channel
        let (shutdown_request_tx, _) = broadcast::channel(1);

        // Database does not currently need shutdown channel
        let database = Database::new()?;

        // Netspot manager has worker tasks, therefore we give it shutdown channels
        let netspots = NetspotManager::new(
            shutdown_request_tx.subscribe(),
            shutdown_complete_tx,
            database.get_configurations()?,
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
