pub mod configuration;
pub mod network;
pub mod status;

use rocket_okapi::openapi_get_routes;

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![
        status::status_all,
        status::start_all,
        status::stop_all,
        status::restart_all,
        status::status_by_id,
        status::start_by_id,
        status::stop_by_id,
        status::restart_by_id,
        configuration::netspot_add,
        configuration::netspot_get,
        configuration::netspot_put,
        configuration::netspot_delete,
        network::interfaces
    ]
}
