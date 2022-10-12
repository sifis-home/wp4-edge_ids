use crate::state::database::DatabaseError;
use crate::state::NetspotControlState;
use crate::structures::configuration::NetspotConfig;
use rocket::http::Status;
use rocket::log::private::warn;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};
use rocket_okapi::openapi;

fn update_all_netspots(state: &State<NetspotControlState>) {
    if let Ok(configurations) = state.database.get_configurations() {
        if state.netspots.update_all(configurations).is_err() {
            warn!("Unexpected: updating process configurations failed");
        }
    } else {
        warn!("Unexpected: reading configurations failed");
    }
}

/// # Create a new netspot configuration
///
/// Lets a user post a new configuration
#[openapi(tag = "Configuration")]
#[post("/netspot", data = "<new_config>")]
pub async fn netspot_add(
    state: &State<NetspotControlState>,
    new_config: Json<NetspotConfig>,
) -> Result<Status, Status> {
    if state.database.add_configuration(&*new_config).is_ok() {
        update_all_netspots(state);
        return Ok(Status::Created);
    }
    Err(Status::BadRequest)
}

/// # Get netspot configuration
///
/// Get netspot configuration by ID
#[openapi(tag = "Configuration")]
#[get("/netspot/<id>")]
pub async fn netspot_get(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
) -> Result<Option<Json<NetspotConfig>>, Status> {
    match id {
        Ok(id) => match state.database.get_configuration(id) {
            Some(config) => Ok(Some(Json(config))),
            None => Ok(None),
        },
        Err(_) => Err(Status::BadRequest),
    }
    // TODO: Check if there is a way to return 200, 400, 404,
    //       and have type info in the generated OpenAPI
}

/// # Update an existing netspot configuration
///
/// Update netspot configuration by ID
#[openapi(tag = "Configuration")]
#[put("/netspot/<id>", data = "<config>")]
pub async fn netspot_put(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
    config: Json<NetspotConfig>,
) -> Result<(), Status> {
    if let Ok(id) = id {
        return match state.database.set_configuration(id, &*config) {
            Ok(_) => {
                update_all_netspots(state);
                Ok(())
            }
            Err(DatabaseError::NotFound) => Err(Status::NotFound),
            Err(_) => Err(Status::BadRequest),
        };
    }
    Err(Status::BadRequest)
}

/// # Delete netspot configuration
///
/// Delete netspot configuration by ID
#[openapi(tag = "Configuration")]
#[delete("/netspot/<id>")]
pub async fn netspot_delete(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
) -> Result<(), Status> {
    if let Ok(id) = id {
        return match state.database.delete_configuration(id) {
            Ok(_) => {
                update_all_netspots(state);
                Ok(())
            }
            Err(DatabaseError::NotFound) => Err(Status::NotFound),
            Err(_) => Err(Status::BadRequest),
        };
    }
    Err(Status::BadRequest)
}
