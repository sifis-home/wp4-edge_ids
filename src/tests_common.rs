use crate::build_rocket;
use crate::state::NetspotControlState;
use crate::structures::status::{Status, Statuses};

use rocket::local::asynchronous::Client;
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

pub struct TestSetup {
    pub client: Client,
    pub test_dir: TempDir,
}

impl TestSetup {
    pub async fn new() -> TestSetup {
        // Using alternative constructor to point database to temporary directory
        let test_dir = TempDir::new().expect("temporary directory");
        let mut test_db = PathBuf::from(test_dir.path());
        test_db.push("test.db");

        // Creating state object
        let state = NetspotControlState::new_customized(None, test_dir.path(), &test_db)
            .await
            .expect("Valid state object");

        // Build test Client
        let client = Client::untracked(build_rocket(state))
            .await
            .expect("valid rocket");

        // Return complete test setup
        TestSetup { client, test_dir }
    }

    pub async fn cleanup(&self) {
        if let Some(state) = self.client.rocket().state::<NetspotControlState>() {
            state.shutdown().await;
        }
    }
}

pub fn statuses_to_hash_map(statuses: Statuses) -> HashMap<i32, Status> {
    let mut hash_map = HashMap::new();
    for status in statuses {
        hash_map.insert(status.id, status);
    }
    hash_map
}
