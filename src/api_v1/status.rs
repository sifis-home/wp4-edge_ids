use crate::state::NetspotControlState;
use crate::structures::status::{Status, Statuses};
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_okapi::openapi;

/// # Restart netspot service
///
/// Restart netspot configuration by ID
#[openapi(tag = "Status")]
#[get("/netspot/<id>/restart")]
pub async fn restart_by_id(state: &State<NetspotControlState>, id: i32) -> Option<Json<Status>> {
    match state.netspots.restart_by_id(id) {
        Ok(status) => Some(Json(status)),
        Err(_) => None,
    }
}

/// # Start netspot service
///
/// Start netspot configuration by ID
#[openapi(tag = "Status")]
#[get("/netspot/<id>/start")]
pub async fn start_by_id(state: &State<NetspotControlState>, id: i32) -> Option<Json<Status>> {
    match state.netspots.start_by_id(id) {
        Ok(status) => Some(Json(status)),
        Err(_) => None,
    }
}

/// # Netspot service status
///
/// Status for the netspot configuration by ID
#[openapi(tag = "Status")]
#[get("/netspot/<id>/status")]
pub async fn status_by_id(state: &State<NetspotControlState>, id: i32) -> Option<Json<Status>> {
    match state.netspots.status_by_id(id) {
        Ok(status) => Some(Json(status)),
        Err(_) => None,
    }
}

/// # Stop netspot service
///
/// Stop netspot configuration by ID
#[openapi(tag = "Status")]
#[get("/netspot/<id>/stop")]
pub async fn stop_by_id(state: &State<NetspotControlState>, id: i32) -> Option<Json<Status>> {
    match state.netspots.stop_by_id(id) {
        Ok(status) => Some(Json(status)),
        Err(_) => None,
    }
}

/// # Status of netspot services
///
/// List netspot configurations and their status
#[openapi(tag = "Status")]
#[get("/netspots")]
pub async fn status_all(state: &State<NetspotControlState>) -> Json<Statuses> {
    Json(state.netspots.status_all())
}

/// # Restart all netspot services
///
/// Restart all netspot configurations
#[openapi(tag = "Status")]
#[get("/netspots/restart")]
pub async fn restart_all(state: &State<NetspotControlState>) -> Json<Statuses> {
    state.netspots.restart_all();
    Json(state.netspots.status_all())
}

/// # Start all netspot services
///
/// Start all netspot configurations
#[openapi(tag = "Status")]
#[get("/netspots/start")]
pub async fn start_all(state: &State<NetspotControlState>) -> Json<Statuses> {
    state.netspots.start_all();
    Json(state.netspots.status_all())
}

/// # Stop all netspot services
///
/// Stop all netspot configurations
#[openapi(tag = "Status")]
#[get("/netspots/stop")]
pub async fn stop_all(state: &State<NetspotControlState>) -> Json<Statuses> {
    state.netspots.stop_all();
    Json(state.netspots.status_all())
}
