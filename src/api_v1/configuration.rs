use crate::state::database::DatabaseError;
use crate::state::NetspotControlState;
use crate::structures::configuration::NetspotConfig;
use rocket::http::Status;
use rocket::log::private::warn;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};
use rocket_okapi::openapi;

async fn update_all_netspots(state: &State<NetspotControlState>) {
    if let Ok(configurations) = state.database.get_configurations() {
        if state.netspots.update_all(configurations).await.is_err() {
            warn!("Unexpected: updating process configurations failed");
        }
    } else {
        warn!("Unexpected: reading configurations failed");
    }
}

/// # Create a new netspot configuration
///
/// Lets a user post a new configuration
#[openapi(tag = "Configuration")]
#[post("/netspot", data = "<new_config>")]
pub async fn netspot_add(
    state: &State<NetspotControlState>,
    new_config: Json<NetspotConfig>,
) -> Result<Status, Status> {
    if state.database.add_configuration(&new_config).is_ok() {
        update_all_netspots(state).await;
        return Ok(Status::Created);
    }
    Err(Status::BadRequest)
}

/// # Get netspot configuration
///
/// Get netspot configuration by ID
#[openapi(tag = "Configuration")]
#[get("/netspot/<id>")]
pub async fn netspot_get(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
) -> Result<Option<Json<NetspotConfig>>, Status> {
    match id {
        Ok(id) => match state.database.get_configuration(id) {
            Some(config) => Ok(Some(Json(config))),
            None => Ok(None),
        },
        Err(_) => Err(Status::BadRequest),
    }
    // TODO: Check if there is a way to return 200, 400, 404,
    //       and have type info in the generated OpenAPI
}

/// # Update an existing netspot configuration
///
/// Update netspot configuration by ID
#[openapi(tag = "Configuration")]
#[put("/netspot/<id>", data = "<config>")]
pub async fn netspot_put(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
    config: Json<NetspotConfig>,
) -> Result<(), Status> {
    if let Ok(id) = id {
        return match state.database.set_configuration(id, &config) {
            Ok(_) => {
                update_all_netspots(state).await;
                Ok(())
            }
            Err(DatabaseError::NotFound) => Err(Status::NotFound),
            Err(_) => Err(Status::BadRequest),
        };
    }
    Err(Status::BadRequest)
}

/// # Delete netspot configuration
///
/// Delete netspot configuration by ID
#[openapi(tag = "Configuration")]
#[delete("/netspot/<id>")]
pub async fn netspot_delete(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
) -> Result<(), Status> {
    if let Ok(id) = id {
        let _ = state.netspots.stop_by_id(id).await;
        return match state.database.delete_configuration(id) {
            Ok(_) => {
                update_all_netspots(state).await;
                Ok(())
            }
            Err(DatabaseError::NotFound) => Err(Status::NotFound),
            Err(_) => Err(Status::BadRequest),
        };
    }
    Err(Status::BadRequest)
}

#[cfg(test)]
mod tests {
    use crate::api_v1::tests_common::TestSetup;
    use crate::structures::configuration::NetspotConfig;
    use crate::structures::status::Statuses;
    use rocket::http::Status;

    // This test does the following:
    //
    // 1. GET     /netspots    : Checking configuration list expecting only the default
    // 2. POST    /netspot     : Adds test configuration
    // 3. GET     /netspots    : Checking that test configuration was added
    // 4. PUT     /netspot/2   : Updates the test configuration
    // 5. GET     /netspot/2   : Checks that test configuration changed
    // 6. DELETE  /netspot/2   : Deletes test configuration
    // 7. GET     /netspots    : Checks that test configuration was deleted
    #[tokio::test]
    async fn test_configuration_crud() {
        let setup = TestSetup::new().await;
        let client = &setup.client;

        // 1. GET     /netspots    : Checking configuration list expecting only the default
        let response = client.get("/v1/netspots").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let statuses = response.into_json::<Statuses>().await.expect("Valid JSON");
        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].name, "Default configuration");

        // 2. POST    /netspot     : Adds test configuration
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
        assert_eq!(response.status(), Status::Created);

        // 3. GET     /netspots    : Checking that test configuration was added
        let response = client.get("/v1/netspots").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let statuses = response.into_json::<Statuses>().await.expect("Valid JSON");
        assert_eq!(statuses.len(), 2);
        let mut test_id = None;
        for status in statuses {
            if status.name == "Test" {
                test_id = Some(status.id);
            }
        }
        assert_eq!(test_id, Some(2));

        // 4. PUT     /netspot/2   : Updates the test configuration
        let response = client
            .put("/v1/netspot/2")
            .body(
                r#"{
	"configuration": {
		"name": "Test Changed",
		"device": "any",
		"promiscuous": true,
		"enabled": false
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
        assert_eq!(response.status(), Status::Ok);

        // 5. GET     /netspot/2   : Checks that test configuration changed
        let response = client.get("/v1/netspot/2").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let config = response
            .into_json::<NetspotConfig>()
            .await
            .expect("Valid JSON");
        assert_eq!(config.configuration.name, "Test Changed");
        assert!(!config.configuration.enabled);

        // 6. DELETE  /netspot/2   : Deletes test configuration
        let response = client.delete("/v1/netspot/2").dispatch().await;
        assert_eq!(response.status(), Status::Ok);

        // 7. GET     /netspots    : Checks that test configuration was deleted
        let response = client.get("/v1/netspots").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let statuses = response.into_json::<Statuses>().await.expect("Valid JSON");
        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].name, "Default configuration");

        setup.cleanup().await;
    }

    // This test does the following:
    //
    // 1. POST    /netspot     : Invalid JSON
    // 2. GET     /netspot/2   : Invalid ID
    // 3. GET     /netspot/foo : Invalid ID
    // 4. DELETE  /netspot/2   : Invalid ID
    // 5. DELETE  /netspot/foo : Invalid ID
    // 6. PUT     /netspot/2   : Invalid ID
    // 7. PUT     /netspot/foo : Invalid ID
    // 8. PUT     /netspot/1   : Invalid JSON
    #[tokio::test]
    async fn test_configuration_invalid_requests() {
        let setup = TestSetup::new().await;
        let client = &setup.client;
        let valid_json = r#"{"configuration":{"name":"Test","enabled":false}}"#;
        let invalid_json = r#"{"configuration":{"enabled":false}}"#;

        // 1. POST    /netspot     : Invalid JSON
        let response = client
            .post("/v1/netspot")
            .body(invalid_json)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::UnprocessableEntity);

        // 2. GET     /netspot/2   : Invalid ID
        let response = client.get("/v1/netspot/2").dispatch().await;
        assert_eq!(response.status(), Status::NotFound);

        // 3. GET     /netspot/foo : Invalid ID
        let response = client.get("/v1/netspot/foo").dispatch().await;
        assert_eq!(response.status(), Status::BadRequest);

        // 4. DELETE  /netspot/2   : Invalid ID
        let response = client.delete("/v1/netspot/2").dispatch().await;
        assert_eq!(response.status(), Status::NotFound);

        // 5. DELETE  /netspot/foo : Invalid ID
        let response = client.delete("/v1/netspot/foo").dispatch().await;
        assert_eq!(response.status(), Status::BadRequest);

        // 6. PUT     /netspot/2   : Invalid ID
        let response = client
            .put("/v1/netspot/2")
            .body(valid_json)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::NotFound);

        // 7. PUT     /netspot/foo : Invalid ID
        let response = client
            .put("/v1/netspot/foo")
            .body(valid_json)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::BadRequest);

        // 8. PUT     /netspot/1   : Invalid JSON
        let response = client
            .put("/v1/netspot/1")
            .body(invalid_json)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::UnprocessableEntity);

        setup.cleanup().await;
    }
}
