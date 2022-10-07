pub mod database;
pub mod netspots;

use database::Database;
use netspots::NetspotManager;
use rocket::debug;

pub struct NetspotControl {
    pub netspots: NetspotManager,
    pub database: Database,
}

impl NetspotControl {
    pub fn new() -> Result<NetspotControl, String> {
        let database = Database::new()?;
        let netspots = NetspotManager::new(database.get_configurations()?)?;
        debug!("NetspotControl started.");
        netspots.start_all();
        Ok(NetspotControl { database, netspots })
    }

    pub fn shutdown(&self) {
        self.netspots.stop_all();
        debug!("NetspotControl shut down gracefully.")
    }
}
