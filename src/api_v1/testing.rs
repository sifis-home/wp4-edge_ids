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
