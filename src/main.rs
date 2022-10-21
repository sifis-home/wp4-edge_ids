mod api_v1;
mod state;
mod structures;

use crate::state::NetspotControlState;
use dotenv::dotenv;
use rocket::fs::{relative, FileServer};
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use tokio::sync::mpsc;

#[rocket::main]
async fn main() {
    // Read .env file when available
    if dotenv().is_ok() {
        println!("Loaded environment variables from .env file");
    }

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

    // We prepare the multi-producer, single-consumer channel, which is given for NetspotControl
    // worker tasks. At the end of the program, we wait for the channel to shut down to ensure all
    // worker tasks are done with their work. Then we create a state object with the shutdown
    // sender object.
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel::<()>(1);
    let state = match NetspotControlState::new(shutdown_complete_tx) {
        Ok(state) => state,
        Err(err) => {
            eprintln!("Netspot Control had an error: {}", err);
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

    // Wait for NetspotControl worker tasks to stop
    let _ = shutdown_complete_rx.recv().await;
    println!("NetspotControl shutdown completed.")
}
