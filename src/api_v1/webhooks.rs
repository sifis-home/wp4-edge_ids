use crate::state::database::DatabaseError;
use crate::state::NetspotControlState;
use crate::structures::webhooks::{Webhook, WebhookList};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};
use rocket_okapi::openapi;

fn update_webhooks(state: &State<NetspotControlState>) {
    match state.database.get_webhooks() {
        Ok(webhooks) => {
            state.webhooks.update(webhooks);
        }
        Err(err) => {
            println!("Unexpected: Could not get webhooks {err}");
        }
    }
}

/// # Create a new webhook
///
/// Let a user post a new webhook configuration
#[openapi(tag = "Webhooks")]
#[post("/netspots/webhook", data = "<new_hook>")]
pub async fn webhook_add(
    state: &State<NetspotControlState>,
    new_hook: Json<Webhook>,
) -> Result<Status, Status> {
    if state.database.add_webhook(&new_hook).is_ok() {
        update_webhooks(state);
        return Ok(Status::Created);
    }
    Err(Status::BadRequest)
}

/// # Get webhook configuration
///
/// Get webhook configuration by ID
#[openapi(tag = "Webhooks")]
#[get("/netspots/webhook/<id>")]
pub async fn webhook_get(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
) -> Result<Option<Json<Webhook>>, Status> {
    match id {
        Ok(id) => match state.database.get_webhook(id) {
            Some(hook) => Ok(Some(Json(hook))),
            None => Ok(None),
        },
        Err(_) => Err(Status::BadRequest),
    }
}

/// # Update webhook configuration
///
/// Update webhook configuration by ID
#[openapi(tag = "Webhooks")]
#[put("/netspots/webhook/<id>", data = "<hook>")]
pub async fn webhook_put(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
    hook: Json<Webhook>,
) -> Result<(), Status> {
    if let Ok(id) = id {
        return match state.database.set_webhook(id, &hook) {
            Ok(_) => {
                update_webhooks(state);
                Ok(())
            }
            Err(DatabaseError::NotFound) => Err(Status::NotFound),
            Err(_) => Err(Status::InternalServerError),
        };
    }
    Err(Status::BadRequest)
}

/// # Delete webhook configuration
///
/// Delete webhook configuration by ID
#[openapi(tag = "Webhooks")]
#[delete("/netspots/webhook/<id>")]
pub async fn webhook_delete(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
) -> Result<(), Status> {
    if let Ok(id) = id {
        return match state.database.delete_webhook(id) {
            Ok(_) => {
                update_webhooks(state);
                Ok(())
            }
            Err(DatabaseError::NotFound) => Err(Status::NotFound),
            Err(_) => Err(Status::InternalServerError),
        };
    }
    Err(Status::BadRequest)
}

/// # List installed webhooks
///
/// Lists installed webhooks by their id and names.
/// Use ID to query detailed configuration when needed.
#[openapi(tag = "Webhooks")]
#[get("/netspot/webhooks")]
pub async fn webhooks_list(
    state: &State<NetspotControlState>,
) -> Result<Json<WebhookList>, Status> {
    match state.database.list_webhooks() {
        Ok(webhooks) => Ok(Json(webhooks)),
        Err(_) => Err(Status::InternalServerError),
    }
}
