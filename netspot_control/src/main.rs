mod api_v1;
mod configurations;
mod state;

use crate::state::NetspotControl;
use rocket::debug;
use rocket::fs::{relative, FileServer};
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[rocket::main]
async fn main() {
    // Prepare configuration for API documentation.
    let rapidoc_config = RapiDocConfig {
        title: Some("Netspot Control Service | API Documentation".to_string()),
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("General", "../openapi.json")],
            ..Default::default()
        },
        hide_show: HideShowConfig {
            allow_spec_url_load: false,
            allow_spec_file_load: false,
            ..Default::default()
        },
        ..Default::default()
    };
    let swagger_ui_config = SwaggerUIConfig {
        url: "../openapi.json".to_owned(),
        ..Default::default()
    };

    // Create configuration and state manager
    let state = match NetspotControl::new() {
        Ok(state) => state,
        Err(err) => {
            eprintln!("NetspotControl: {}", err);
            return;
        }
    };

    // Launch server
    let launch_result = rocket::build()
        // Managed state through NetspotControl
        .manage(state)
        // Mount static files to root
        .mount("/", FileServer::from(relative!("static")))
        // Mount APIv1
        .mount("/v1/", api_v1::routes())
        // API documentation from the design
        // Using the openapi.json from the static/design folder
        .mount("/design/rapidoc/", make_rapidoc(&rapidoc_config))
        .mount("/design/swagger-ui/", make_swagger_ui(&swagger_ui_config))
        // API documentation from the implementation
        .mount("/v1/rapidoc/", make_rapidoc(&rapidoc_config))
        .mount("/v1/swagger-ui/", make_swagger_ui(&swagger_ui_config))
        .launch()
        .await;

    // Checking launch result
    match launch_result {
        Ok(rocket) => {
            // Shutdown configuration and state manager
            if let Some(configuration_and_state_manager) = rocket.state::<NetspotControl>() {
                configuration_and_state_manager.shutdown();
            }
            debug!("Server shut down gracefully.")
        }
        Err(err) => debug!("Rocket had an error: {}", err),
    };
}
