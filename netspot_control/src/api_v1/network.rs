use rocket::get;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes};

pub type Result<T, E = Debug<pcap::Error>> = std::result::Result<T, E>;

/// # Get host network interfaces
///
/// Returns all available network interfaces on the host system
#[openapi(tag = "Network")]
#[get("/network/interfaces")]
pub async fn interfaces() -> Result<Json<Vec<String>>> {
    let mut devices = Vec::new();
    let device_list = pcap::Device::list()?;
    for device in device_list {
        devices.push(device.name)
    }
    Ok(Json(devices))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![interfaces]
}
