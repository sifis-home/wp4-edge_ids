pub mod configuration;
pub mod network;
pub mod status;

use rocket_okapi::openapi_get_routes;

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![
        status::statuses,
        configuration::netspot_add,
        configuration::netspot_get,
        configuration::netspot_put,
        configuration::netspot_delete,
        network::interfaces
    ]
}
