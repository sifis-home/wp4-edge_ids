use crate::structures::statistics::{AlarmMessages, DataMessages};
use crate::NetspotControlState;
use rocket::serde::json::Json;
use rocket::{get, http, State};
use rocket_okapi::openapi;

/// # Read alarms from netspot statistics
///
/// Reads recorded alarms from nestpot statistics.
///
/// We can use parameters to limit which results are returned.
/// Without parameters, only 100 last items are returned.
#[openapi(tag = "Statistics")]
#[get("/netspots/alarms?<time>&<last>")]
pub async fn ger_alarms(
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
/// Reads recorded nestpot statistics.
///
/// We can use parameters to limit which results are returned.
/// Without parameters, only 100 last items are returned.
#[openapi(tag = "Statistics")]
#[get("/netspots/data?<time>&<last>")]
pub async fn ger_data(
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
