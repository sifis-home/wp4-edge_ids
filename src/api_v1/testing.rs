use crate::state::NetspotControlState;
use crate::structures::statistics::{AlertStatus, Stat};
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::okapi::schemars;
use rocket_okapi::openapi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TestAlarmMessage {
    #[serde(default = "test_alarm_default_name")]
    pub name: String,
    #[serde(default = "test_alarm_default_stat")]
    pub stat: Stat,
    #[serde(default = "test_alarm_default_status")]
    pub status: AlertStatus,
    #[serde(default = "test_alarm_default_value")]
    pub value: f64,
    #[serde(default = "test_alarm_default_probability")]
    pub probability: f64,
}

fn test_alarm_default_name() -> String {
    "Test alarm".to_string()
}

fn test_alarm_default_stat() -> Stat {
    Stat::RSyn
}

fn test_alarm_default_status() -> AlertStatus {
    AlertStatus::UpAlert
}

fn test_alarm_default_value() -> f64 {
    1000.0
}

fn test_alarm_default_probability() -> f64 {
    0.75
}

impl Default for TestAlarmMessage {
    fn default() -> Self {
        TestAlarmMessage {
            name: test_alarm_default_name(),
            stat: test_alarm_default_stat(),
            status: test_alarm_default_status(),
            value: test_alarm_default_value(),
            probability: test_alarm_default_probability(),
        }
    }
}

/// # Send test alarm
///
/// This endpoint allows developers to send test alarm messages.
///
/// Test alarm messages have automatically the following parameters set:
///
/// * `time` as nanoseconds since Unix Epoch
///
/// * `series` "TEST ALARM"
///
/// * `code`  1
///
/// * `type` "alarm"
///
/// Other alarm parameters can be defined in request body, but this is optional.
#[openapi(tag = "Testing")]
#[post("/netspots/test/alarm", data = "<message>")]
pub async fn send_test_alarm(
    state: &State<NetspotControlState>,
    message: Option<Json<TestAlarmMessage>>,
) -> Result<Status, Status> {
    let test_alarm = match message {
        None => TestAlarmMessage::default(),
        Some(json) => json.into_inner(),
    };
    match state.netspots.send_test_alarm(test_alarm) {
        true => Ok(Status::Created),
        false => Err(Status::InternalServerError),
    }
}

#[cfg(test)]
mod tests {
    use crate::structures::statistics::{AlarmMessages, AlertStatus, MessageType, Stat};
    use crate::tests_common::TestSetup;
    use rocket::http::Status;

    // This test does the following:
    //
    // 1. Inserts default alarm message
    // 2. Inserts custom alarm message
    // 3. Waits for one second to database writes to complete
    // 4. Checks that alarms are available
    #[tokio::test]
    async fn test_testing() {
        let setup = TestSetup::new().await;
        let client = &setup.client;
        let uri = "/v1/netspots/test/alarm";

        // 1. Inserts default alarm message
        let response = client.post(uri).dispatch().await;
        assert_eq!(response.status(), Status::Created);

        // 2. Inserts custom alarm message
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

        // 3. Waits for one second to database writes to complete
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // 4. Checks that alarms are available
        let response = client.get("/v1/netspots/alarms").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let messages = response
            .into_json::<AlarmMessages>()
            .await
            .expect("Valid JSON");
        assert_eq!(messages.len(), 2);

        assert_eq!(messages[0].name, "Test alarm");
        assert_eq!(messages[0].stat, Stat::RSyn);
        assert_eq!(messages[0].status, AlertStatus::UpAlert);
        assert_eq!(messages[0].value, 1000.0);
        assert_eq!(messages[0].probability, 0.75);
        assert_eq!(messages[0].series, "TEST ALARM");
        assert_eq!(messages[0].code, 1);
        assert_eq!(messages[0].msg_type, MessageType::Alarm);

        assert_eq!(messages[1].name, "Test");
        assert_eq!(messages[1].stat, Stat::Perf);
        assert_eq!(messages[1].status, AlertStatus::UpAlert);
        assert_eq!(messages[1].value, 12.3);
        assert_eq!(messages[1].probability, 0.75);
        assert_eq!(messages[1].series, "TEST ALARM");
        assert_eq!(messages[1].code, 1);
        assert_eq!(messages[1].msg_type, MessageType::Alarm);

        setup.cleanup().await;
    }
}
