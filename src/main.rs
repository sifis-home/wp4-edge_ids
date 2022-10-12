mod api_v1;
mod state;
mod structures;

use crate::state::NetspotControlFairing;
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

    // Launch server
    let launch_result = rocket::build()
        // Attach Netspot Control Fairing
        .attach(NetspotControlFairing {})
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
    if let Err(err) = launch_result {
        eprintln!("Rocket had an error: {}", err);
    };
}
