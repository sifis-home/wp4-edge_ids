mod api_v1;
mod state;
mod structures;
mod tasks;

use crate::state::NetspotControlState;
use dotenvy::dotenv;
use rocket::fs::{relative, FileServer};
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

/// Entry Point for the Server Program
#[rocket::main]
async fn main() {
    println!("NetspotControl started.");

    // Read .env file when available
    if dotenv().is_ok() {
        println!("Loaded environment variables from .env file");
    }

    // Creating State object for the server
    let state = match NetspotControlState::new() {
        Ok(state) => state,
        Err(err) => {
            eprintln!("NetspotControlState had an error: {}", err);
            return;
        }
    };

    // Launch server
    let launch_result = build_rocket(state).launch().await;

    // Print error if launching server failed
    match launch_result {
        Ok(rocket) => {
            // Shutdown state manager as cleanly as possible
            if let Some(state) = rocket.state::<NetspotControlState>() {
                state.shutdown().await;
            }
        }
        Err(err) => {
            eprintln!("Rocket had an error: {}", err);
        }
    };
    println!("NetspotControl Stopped.");
}

/// Builds the Netspot Control Rocket
///
/// This function creates a Rocket object that is ready to launch. Rocket is created from the main
/// function, but also unit tests use this function to check endpoints using local instances.
fn build_rocket(state: NetspotControlState) -> rocket::Rocket<rocket::Build> {
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

    // Build rocket
    rocket::build()
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
}
