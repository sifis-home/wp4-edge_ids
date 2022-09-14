use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::{get, routes};

type Result<T, E = Debug<pcap::Error>> = std::result::Result<T, E>;

#[get("/network/interfaces")]
async fn network_interfaces() -> Result<Json<Vec<String>>> {
    let mut devices = Vec::new();
    let device_list = pcap::Device::list()?;
    for device in device_list {
        devices.push(device.name)
    }
    Ok(Json(devices))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/v1/", routes![network_interfaces])
        .launch()
        .await?;
    Ok(())
}
