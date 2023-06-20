use crate::state::NetspotControlState;
use std::path::{Path, PathBuf};

use clap::Parser;
use dotenvy::dotenv;
use rocket::fs::{relative, FileServer};
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use tokio::task::JoinHandle;

mod api_v1;
mod state;
mod structures;
mod tasks;

#[cfg(test)]
pub mod tests_common;

/// Command line options
#[derive(Debug, Parser)]
#[command(name = "Netspot Control")]
#[command(
    about,
    long_about = "The netspot is a simple anomaly-based network IDS. 

This server allows configuring multiple netspot instances for scanning
the network. In addition, the server collects statistics and makes them 
available under status endpoints. Clients can also configure webhooks
to receive messages in near real-time.

Server configuration can be done using Rocket environment variables.
See https://rocket.rs/v0.5-rc/guide/configuration/

In addition, the following options are available as command-line
arguments.",
    version
)]
struct Cli {
    /// Directory, where the runtime files are stored
    #[arg(short, long)]
    runtime_path: Option<PathBuf>,

    /// Database file path
    #[arg(short, long)]
    db_path: Option<PathBuf>,

    /// Automatic shutdown after <SECONDS>
    #[arg(long = "shutdown-after")]
    seconds: Option<u64>,

    /// Send alarms to the SIFIS-Home DHT REST interface
    ///
    /// The API URL is most likely http://localhost:3000/pub
    #[arg(long, value_name = "API URL")]
    dht: Option<String>,
}

/// Entry Point for the Server Program
#[rocket::main]
async fn main() {
    // Parsing command line arguments
    let cli = Cli::parse();

    println!("NetspotControl started.");

    // Read .env file when available
    if dotenv().is_ok() {
        println!("Loaded environment variables from .env file");
    }

    // Creating State object for the server
    let state = if cli.db_path.is_none() && cli.runtime_path.is_none() {
        NetspotControlState::new(cli.dht).await
    } else {
        let runtime_path = cli.runtime_path.unwrap_or(PathBuf::from("/tmp"));
        let db_path = cli.db_path.unwrap_or(Path::join(&runtime_path, "test.db"));
        NetspotControlState::new_customized(cli.dht, &runtime_path, &db_path).await
    };
    let state = match state {
        Ok(state) => state,
        Err(err) => {
            eprintln!("NetspotControlState had an error: {}", err);
            return;
        }
    };

    // Launch server
    let mut shutdown_handle: Option<JoinHandle<()>> = None;
    let launch_result = match cli.seconds {
        None => {
            // Keep running until SIGINT or SIGTERM
            build_rocket(state).launch().await
        }
        Some(seconds) => {
            // Automatic shutdown after given seconds
            let rocket = match build_rocket(state).ignite().await {
                Ok(rocket) => rocket,
                Err(err) => {
                    eprintln!("Could not ignite Rocket server: {}", err);
                    return;
                }
            };

            let shutdown = rocket.shutdown();
            shutdown_handle = Some(tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(seconds)).await;
                shutdown.notify();
            }));

            rocket.launch().await
        }
    };

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

    // Ensure that shutdown notify thread is finished
    if let Some(handle) = shutdown_handle {
        handle.await.unwrap();
    }

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
