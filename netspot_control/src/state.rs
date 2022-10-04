pub mod database;

use database::Database;
use rocket::debug;

pub struct NetspotControl {
    pub database: Database,
}

impl NetspotControl {
    pub fn new() -> Result<NetspotControl, String> {
        let database = Database::new()?;
        debug!("NetspotControl started.");
        Ok(NetspotControl { database })
    }

    pub fn shutdown(&self) {
        debug!("NetspotControl shut down gracefully.")
    }
}
