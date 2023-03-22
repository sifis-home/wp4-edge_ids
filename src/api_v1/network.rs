use rocket::get;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

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

#[cfg(test)]
mod tests {
    use crate::api_v1::tests_common::TestSetup;
    use rocket::http::Status;

    #[tokio::test]
    async fn test_interfaces() {
        let setup = TestSetup::new().await;
        let client = &setup.client;

        let response = client.get("/v1/network/interfaces").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let interfaces = response
            .into_json::<Vec<String>>()
            .await
            .expect("Valid JSON");

        // Interfaces list should always have `any` and `lo`
        assert!(interfaces.contains(&String::from("any")));
        assert!(interfaces.contains(&String::from("lo")));

        setup.cleanup().await;
    }
}
