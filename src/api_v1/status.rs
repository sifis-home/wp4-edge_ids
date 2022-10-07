use crate::structures::status::Statuses;
use crate::NetspotControl;
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_okapi::openapi;

/// # Status of netspot services
///
/// List netspot configurations and their status
#[openapi(tag = "Status")]
#[get("/netspots")]
pub async fn statuses(state: &State<NetspotControl>) -> Json<Statuses> {
    Json(state.netspots.statuses())
}
