use crate::state::NetspotControlState;
use crate::structures::webhooks::{Webhook, Webhooks};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};
use rocket_okapi::openapi;

/// # Create a new webhook
///
/// Let a user post a new webhook configuration
#[openapi(tag = "Webhooks")]
#[post("/netspots/webhook", data = "<new_hook>")]
pub async fn webhook_add(
    _state: &State<NetspotControlState>,
    new_hook: Json<Webhook>,
) -> Result<Status, Status> {
    println!("webhook_add: {:#?}", new_hook);
    Err(Status::BadRequest)
}

/// # Get webhook configuration
///
/// Get webhook configuration by ID
#[openapi(tag = "Webhooks")]
#[get("/netspots/webhook/<id>")]
pub async fn webhook_get(
    _state: &State<NetspotControlState>,
    id: Result<i32, &str>,
) -> Result<Option<Json<Webhook>>, Status> {
    println!("webhook_get: {:?}", id);
    Ok(None)
}

/// # Update webhook configuration
///
/// Update webhook configuration by ID
#[openapi(tag = "Webhooks")]
#[put("/netspots/webhook/<id>", data = "<hook>")]
pub async fn webhook_put(
    _state: &State<NetspotControlState>,
    id: Result<i32, &str>,
    hook: Json<Webhook>,
) -> Result<(), Status> {
    println!("webhook_put: {:?} {:#?}", id, hook);
    Err(Status::BadRequest)
}

/// # Delete webhook configuration
///
/// Delete webhook configuration by ID
#[openapi(tag = "Webhooks")]
#[delete("/netspots/webhook/<id>")]
pub async fn webhook_delete(
    _state: &State<NetspotControlState>,
    id: Result<i32, &str>,
) -> Result<(), Status> {
    println!("webhook_delete: {:?}", id);
    Err(Status::BadRequest)
}

/// # List installed webhooks
///
/// Lists installed webhooks by their id and names.
/// Use ID to query detailed configuration when needed.
#[openapi(tag = "Webhooks")]
#[get("/netspot/webhooks")]
pub async fn webhooks_list(_state: &State<NetspotControlState>) -> Json<Webhooks> {
    println!("webhooks_list requested");
    Json(Webhooks::default())
}
