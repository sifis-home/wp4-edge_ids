use crate::structures::statistics::{AlarmMessages, DataMessages};
use crate::NetspotControlState;
use rocket::serde::json::Json;
use rocket::{get, http, State};
use rocket_okapi::openapi;

/// # Read alarms from netspot statistics
///
/// Reads recorded alarms from netspot statistics.
///
/// We can use parameters to limit which results are returned.
/// Without parameters, only 100 last items are returned.
#[openapi(tag = "Statistics")]
#[get("/netspots/alarms?<time>&<last>")]
pub async fn get_alarms(
    state: &State<NetspotControlState>,
    time: Option<i64>,
    mut last: Option<i32>,
) -> Result<Json<AlarmMessages>, http::Status> {
    if time.is_none() && last.is_none() {
        last = Some(100);
    }
    match state.database.get_alarms(time, last) {
        Ok(results) => Ok(Json(results)),
        Err(_) => Err(http::Status::InternalServerError),
    }
}

/// # Read netspot statistics
///
/// Reads recorded netspot statistics.
///
/// We can use parameters to limit which results are returned.
/// Without parameters, only 100 last items are returned.
#[openapi(tag = "Statistics")]
#[get("/netspots/data?<time>&<last>")]
pub async fn get_data(
    state: &State<NetspotControlState>,
    time: Option<i64>,
    mut last: Option<i32>,
) -> Result<Json<DataMessages>, http::Status> {
    if time.is_none() && last.is_none() {
        last = Some(100);
    }
    match state.database.get_data(time, last) {
        Ok(results) => Ok(Json(results)),
        Err(_) => Err(http::Status::InternalServerError),
    }
}

#[cfg(test)]
mod tests {
    use crate::api_v1::tests_common::TestSetup;
    use crate::structures::statistics::{AlarmMessages, DataMessages};
    use rocket::http::Status;

    #[tokio::test]
    async fn test_statistics() {
        let setup = TestSetup::new().await;
        let client = &setup.client;

        // Using the test API to insert alarm message
        let response = client
            .post("/v1/netspots/test/alarm")
            .body(
                r#"{
	"name": "Test",
	"stat": "PERF",
	"status": "UP_ALERT",
	"value": 12.3,
	"probability": 0.75
}"#,
            )
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Created);

        // Waiting for netspot to create some data messages
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        // We should now have at least one data messages
        let response = client.get("/v1/netspots/data").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let messages = response
            .into_json::<DataMessages>()
            .await
            .expect("Valid JSON");
        assert!(!messages.is_empty());

        // We should also have the alarm message we inserted before
        let response = client.get("/v1/netspots/alarms").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let messages = response
            .into_json::<AlarmMessages>()
            .await
            .expect("Valid JSON");
        println!("{messages:#?}");
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].name, "Test");

        setup.cleanup().await;
    }
}
