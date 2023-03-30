use crate::state::NetspotControlState;
use crate::structures::status::{Status, Statuses};
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_okapi::openapi;

/// # Restart netspot service
///
/// Restart netspot configuration by ID
#[openapi(tag = "Status")]
#[get("/netspot/<id>/restart")]
pub async fn restart_by_id(state: &State<NetspotControlState>, id: i32) -> Option<Json<Status>> {
    match state.netspots.restart_by_id(id).await {
        Ok(status) => Some(Json(status)),
        Err(_) => None,
    }
}

/// # Start netspot service
///
/// Start netspot configuration by ID
#[openapi(tag = "Status")]
#[get("/netspot/<id>/start")]
pub async fn start_by_id(state: &State<NetspotControlState>, id: i32) -> Option<Json<Status>> {
    match state.netspots.start_by_id(id).await {
        Ok(status) => Some(Json(status)),
        Err(_) => None,
    }
}

/// # Netspot service status
///
/// Status for the netspot configuration by ID
#[openapi(tag = "Status")]
#[get("/netspot/<id>/status")]
pub async fn status_by_id(state: &State<NetspotControlState>, id: i32) -> Option<Json<Status>> {
    match state.netspots.status_by_id(id).await {
        Ok(status) => Some(Json(status)),
        Err(_) => None,
    }
}

/// # Stop netspot service
///
/// Stop netspot configuration by ID
#[openapi(tag = "Status")]
#[get("/netspot/<id>/stop")]
pub async fn stop_by_id(state: &State<NetspotControlState>, id: i32) -> Option<Json<Status>> {
    match state.netspots.stop_by_id(id).await {
        Ok(status) => Some(Json(status)),
        Err(_) => None,
    }
}

/// # Status of netspot services
///
/// List netspot configurations and their status
#[openapi(tag = "Status")]
#[get("/netspots")]
pub async fn status_all(state: &State<NetspotControlState>) -> Json<Statuses> {
    Json(state.netspots.status_all().await)
}

/// # Restart all netspot services
///
/// Restart all netspot configurations
#[openapi(tag = "Status")]
#[get("/netspots/restart")]
pub async fn restart_all(state: &State<NetspotControlState>) -> Json<Statuses> {
    state.netspots.restart_all().await;
    Json(state.netspots.status_all().await)
}

/// # Start all netspot services
///
/// Start all netspot configurations
#[openapi(tag = "Status")]
#[get("/netspots/start")]
pub async fn start_all(state: &State<NetspotControlState>) -> Json<Statuses> {
    state.netspots.start_all().await;
    Json(state.netspots.status_all().await)
}

/// # Stop all netspot services
///
/// Stop all netspot configurations
#[openapi(tag = "Status")]
#[get("/netspots/stop")]
pub async fn stop_all(state: &State<NetspotControlState>) -> Json<Statuses> {
    state.netspots.stop_all().await;
    Json(state.netspots.status_all().await)
}

#[cfg(test)]
mod tests {
    use crate::structures::status::{ProcessStatus, Status, Statuses};
    use crate::tests_common::{statuses_to_hash_map, TestSetup};
    use rocket::http;

    // This test does the following:
    //
    // 1. Adds second running configuration
    // 2. Add disabled configuration
    // 3. List configurations (should have IDs #1, #2, and #3)
    // 4. Restart #1
    // 5. Start #2
    // 6. Stop #1
    // 7. Restart all
    // 8. Stop all
    // 9. Start all
    // 10. Check status for #3
    #[tokio::test]
    async fn test_valid_requests() {
        let setup = TestSetup::new().await;
        let client = &setup.client;

        // 1. Adds second configuration
        let response = client
            .post("/v1/netspot")
            .body(
                r#"{
	"configuration": {
		"name": "Test",
		"device": "any",
		"promiscuous": true,
		"enabled": true
	},
	"stats": {
		"perf": {
			"enabled": true
		}
	}
}"#,
            )
            .dispatch()
            .await;
        assert_eq!(response.status(), http::Status::Created);

        // 2. Add disabled configuration
        let response = client
            .post("/v1/netspot")
            .body(
                r#"{
	"configuration": {
		"name": "Disabled",
		"enabled": false
	}
}"#,
            )
            .dispatch()
            .await;
        assert_eq!(response.status(), http::Status::Created);

        // 3. List configurations (should have two IDs #1 and #2)
        let response = client.get("/v1/netspots").dispatch().await;
        assert_eq!(response.status(), http::Status::Ok);
        let statuses =
            statuses_to_hash_map(response.into_json::<Statuses>().await.expect("Valid JSON"));
        assert_eq!(statuses.len(), 3);
        assert_eq!(statuses.get(&1).unwrap().status, ProcessStatus::Running);
        assert_eq!(statuses.get(&2).unwrap().status, ProcessStatus::Stopped);
        assert_eq!(statuses.get(&3).unwrap().status, ProcessStatus::Disabled);

        // 4. Restart #1
        let response = client.get("/v1/netspot/1/restart").dispatch().await;
        assert_eq!(response.status(), http::Status::Ok);
        let status = response.into_json::<Status>().await.expect("Valid JSON");
        assert_eq!(status.status, ProcessStatus::Running);

        // 5. Start #2
        let response = client.get("/v1/netspot/2/start").dispatch().await;
        assert_eq!(response.status(), http::Status::Ok);
        let status = response.into_json::<Status>().await.expect("Valid JSON");
        assert_eq!(status.status, ProcessStatus::Running);

        // 6. Stop #1
        let response = client.get("/v1/netspot/1/stop").dispatch().await;
        assert_eq!(response.status(), http::Status::Ok);
        let status = response.into_json::<Status>().await.expect("Valid JSON");
        assert_eq!(status.status, ProcessStatus::Stopped);

        // 7. Restart all
        let response = client.get("/v1/netspots/restart").dispatch().await;
        assert_eq!(response.status(), http::Status::Ok);
        let statuses =
            statuses_to_hash_map(response.into_json::<Statuses>().await.expect("Valid JSON"));
        assert_eq!(statuses.get(&1).unwrap().status, ProcessStatus::Running);
        assert_eq!(statuses.get(&2).unwrap().status, ProcessStatus::Running);
        assert_eq!(statuses.get(&3).unwrap().status, ProcessStatus::Disabled);

        // 8. Stop all
        let response = client.get("/v1/netspots/stop").dispatch().await;
        assert_eq!(response.status(), http::Status::Ok);
        let statuses =
            statuses_to_hash_map(response.into_json::<Statuses>().await.expect("Valid JSON"));
        assert_eq!(statuses.get(&1).unwrap().status, ProcessStatus::Stopped);
        assert_eq!(statuses.get(&2).unwrap().status, ProcessStatus::Stopped);
        assert_eq!(statuses.get(&3).unwrap().status, ProcessStatus::Disabled);

        // 9. Start all
        let response = client.get("/v1/netspots/start").dispatch().await;
        assert_eq!(response.status(), http::Status::Ok);
        let statuses =
            statuses_to_hash_map(response.into_json::<Statuses>().await.expect("Valid JSON"));
        assert_eq!(statuses.get(&1).unwrap().status, ProcessStatus::Running);
        assert_eq!(statuses.get(&2).unwrap().status, ProcessStatus::Running);
        assert_eq!(statuses.get(&3).unwrap().status, ProcessStatus::Disabled);

        // 10. Check status for #3
        let response = client.get("/v1/netspot/3/status").dispatch().await;
        assert_eq!(response.status(), http::Status::Ok);
        let status = response.into_json::<Status>().await.expect("Valid JSON");
        assert_eq!(status.name, "Disabled");
        assert_eq!(status.status, ProcessStatus::Disabled);

        setup.cleanup().await;
    }

    #[tokio::test]
    async fn test_invalid_requests() {
        let setup = TestSetup::new().await;
        let client = &setup.client;

        let endpoints = [
            "/v1/netspot/2/restart",
            "/v1/netspot/2/start",
            "/v1/netspot/2/status",
            "/v1/netspot/2/stop",
            "/v1/netspot/foo/restart",
            "/v1/netspot/foo/start",
            "/v1/netspot/foo/status",
            "/v1/netspot/foo/stop",
        ];

        for endpoint in endpoints {
            let response = client.get(endpoint).dispatch().await;
            assert_eq!(response.status(), http::Status::NotFound);
        }

        setup.cleanup().await;
    }
}
