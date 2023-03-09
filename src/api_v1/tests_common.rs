use crate::build_rocket;
use crate::state::NetspotControlState;
use rocket::local::asynchronous::Client;
use std::path::PathBuf;
use tempfile::TempDir;

pub struct TestSetup {
    pub client: Client,
    pub test_dir: TempDir,
}

pub async fn create_test_setup() -> TestSetup {
    // Using alternative constructor to point database to temporary directory
    let test_dir = TempDir::new().expect("temporary directory");
    let mut test_db = PathBuf::from(test_dir.path());
    test_db.push("test.db");

    // Creating state object
    let state = NetspotControlState::new_customized(
        test_dir.path(),
        test_db.to_str().expect("valid database url"),
    )
    .await
    .expect("Valid state object");

    // Build test Client
    let client = Client::untracked(build_rocket(state))
        .await
        .expect("valid rocket");

    // Return complete test setup
    TestSetup { client, test_dir }
}

pub async fn stop_test_setup(test_setup: TestSetup) {
    if let Some(state) = test_setup.client.rocket().state::<NetspotControlState>() {
        state.shutdown().await;
    }
}
