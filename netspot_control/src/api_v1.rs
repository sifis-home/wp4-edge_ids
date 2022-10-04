pub mod configuration;
pub mod network;

use rocket_okapi::openapi_get_routes;

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![configuration::netspot_get_id, network::interfaces]
}
