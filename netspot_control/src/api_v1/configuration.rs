use crate::configurations::netspot::NetspotConfig;
use crate::NetspotControl;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_okapi::openapi;

/// # Get netspot configuration
///
/// Get netspot configuration by ID
#[openapi(tag = "Configuration")]
#[get("/netspot/<id>")]
pub async fn netspot_get_id(
    state: &State<NetspotControl>,
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
