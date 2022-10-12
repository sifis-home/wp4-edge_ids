pub mod database;
pub mod netspots;

use database::Database;
use netspots::NetspotManager;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Build, Orbit, Rocket};

// Netspot Control State
//--------------------------------------------------------------------------------------------------

pub struct NetspotControlState {
    pub netspots: NetspotManager,
    pub database: Database,
}

impl NetspotControlState {
    pub fn new() -> Result<NetspotControlState, String> {
        let database = Database::new()?;
        let netspots = NetspotManager::new(database.get_configurations()?)?;
        println!("NetspotControl started.");
        netspots.start_all();
        Ok(NetspotControlState { database, netspots })
    }

    pub fn shutdown(&self) {
        self.netspots.stop_all();
        println!("NetspotControl shutdown.")
    }
}

// Netspot Control Fairing for adding State and cleaning at shutdown
//--------------------------------------------------------------------------------------------------

pub struct NetspotControlFairing {}

#[rocket::async_trait]
impl Fairing for NetspotControlFairing {
    fn info(&self) -> Info {
        Info {
            name: "Netspot Control",
            kind: Kind::Ignite | Kind::Shutdown,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        // Create configuration and state manager
        let state = match NetspotControlState::new() {
            Ok(state) => state,
            Err(err) => {
                eprintln!("Netspot Control had an error: {}", err);
                return Err(rocket);
            }
        };
        Ok(rocket.manage(state))
    }

    async fn on_shutdown(&self, rocket: &Rocket<Orbit>) {
        // Shutdown state manager as cleanly as possible
        if let Some(state) = rocket.state::<NetspotControlState>() {
            state.shutdown();
        }
    }
}
