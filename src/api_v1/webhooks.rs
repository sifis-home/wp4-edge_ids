use crate::state::database::DatabaseError;
use crate::state::NetspotControlState;
use crate::structures::webhooks::{Webhook, WebhookList};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};
use rocket_okapi::openapi;

fn update_webhooks(state: &State<NetspotControlState>) {
    match state.database.get_webhooks() {
        Ok(webhooks) => {
            state.webhooks.update(webhooks);
        }
        Err(err) => {
            println!("Unexpected: Could not get webhooks {err}");
        }
    }
}

/// # Create a new webhook
///
/// Let a user post a new webhook configuration
#[openapi(tag = "Webhooks")]
#[post("/netspots/webhook", data = "<new_hook>")]
pub async fn webhook_add(
    state: &State<NetspotControlState>,
    new_hook: Json<Webhook>,
) -> Result<Status, Status> {
    if state.database.add_webhook(&new_hook).is_ok() {
        update_webhooks(state);
        return Ok(Status::Created);
    }
    Err(Status::BadRequest)
}

/// # Get webhook configuration
///
/// Get webhook configuration by ID
#[openapi(tag = "Webhooks")]
#[get("/netspots/webhook/<id>")]
pub async fn webhook_get(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
) -> Result<Option<Json<Webhook>>, Status> {
    match id {
        Ok(id) => match state.database.get_webhook(id) {
            Some(hook) => Ok(Some(Json(hook))),
            None => Ok(None),
        },
        Err(_) => Err(Status::BadRequest),
    }
}

/// # Update webhook configuration
///
/// Update webhook configuration by ID
#[openapi(tag = "Webhooks")]
#[put("/netspots/webhook/<id>", data = "<hook>")]
pub async fn webhook_put(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
    hook: Json<Webhook>,
) -> Result<(), Status> {
    if let Ok(id) = id {
        return match state.database.set_webhook(id, &hook) {
            Ok(_) => {
                update_webhooks(state);
                Ok(())
            }
            Err(DatabaseError::NotFound) => Err(Status::NotFound),
            Err(_) => Err(Status::InternalServerError),
        };
    }
    Err(Status::BadRequest)
}

/// # Delete webhook configuration
///
/// Delete webhook configuration by ID
#[openapi(tag = "Webhooks")]
#[delete("/netspots/webhook/<id>")]
pub async fn webhook_delete(
    state: &State<NetspotControlState>,
    id: Result<i32, &str>,
) -> Result<(), Status> {
    if let Ok(id) = id {
        return match state.database.delete_webhook(id) {
            Ok(_) => {
                update_webhooks(state);
                Ok(())
            }
            Err(DatabaseError::NotFound) => Err(Status::NotFound),
            Err(_) => Err(Status::InternalServerError),
        };
    }
    Err(Status::BadRequest)
}

/// # List installed webhooks
///
/// Lists installed webhooks by their id and names.
/// Use ID to query detailed configuration when needed.
#[openapi(tag = "Webhooks")]
#[get("/netspots/webhooks")]
pub async fn webhooks_list(
    state: &State<NetspotControlState>,
) -> Result<Json<WebhookList>, Status> {
    match state.database.list_webhooks() {
        Ok(webhooks) => Ok(Json(webhooks)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[cfg(test)]
mod tests {
    use crate::api_v1::tests_common::TestSetup;
    use crate::structures::webhooks::{
        Webhook, WebhookHeaders, WebhookList, WebhookRequestMethod, WebhookStatsType,
    };
    use rocket::http::Status;

    // This test does the following:
    //
    // 1. POST   /v1/netspots/webhook   : Adding a new webhook
    // 2. GET    /v1/netspots/webhooks  : List available webhooks (should have one we added)
    // 3. GET    /v1/netspots/webhook/1 : Should return the webhook we added
    // 4. PUT    /v1/netspots/webhook/1 : Updating webhook
    // 5. GET    /v1/netspots/webhook/1 : Should return the updated webhook
    // 6. DELETE /v1/netspots/webhook/1 : Should delete webhook
    // 7. GET    /v1/netspots/webhooks  : List available webhooks (should be empty)
    #[tokio::test]
    async fn test_webhooks_crud() {
        let setup = TestSetup::new().await;
        let client = &setup.client;
        let webhook_uri = "/v1/netspots/webhook";
        let webhooks_uri = "/v1/netspots/webhooks";
        let webhook_1_uri = "/v1/netspots/webhook/1";

        // 1. POST   /v1/netspots/webhook   : Adding a new webhook
        let mut headers = WebhookHeaders::new();
        headers.insert("foo".to_string(), "bar".to_string());
        let mut webhook = Webhook {
            name: "Test".to_string(),
            address: "http://127.0.0.1:9020/alarms".to_string(),
            method: WebhookRequestMethod::Post,
            headers,
            stats_type: WebhookStatsType::Alarms,
        };
        let response = client
            .post(webhook_uri)
            .body(serde_json::to_string(&webhook).unwrap())
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Created);

        // 2. GET    /v1/netspots/webhooks  : List available webhooks (should have one we added)
        let response = client.get(webhooks_uri).dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let webhooks = response.into_json::<WebhookList>().await.unwrap();
        assert_eq!(webhooks.len(), 1);
        assert_eq!(webhooks[0].name, "Test");

        // 3. GET    /v1/netspots/webhook/1 : Should return the webhook we added
        let response = client.get(webhook_1_uri).dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let webhook_recv = response.into_json::<Webhook>().await.unwrap();
        assert_eq!(webhook, webhook_recv);

        // 4. PUT    /v1/netspots/webhook/1 : Updating webhook
        webhook.address = "http://127.0.0.1:9020/both".to_string();
        webhook.headers.clear();
        webhook.method = WebhookRequestMethod::Put;
        webhook.name = "Test webhook".to_string();
        webhook.stats_type = WebhookStatsType::Both;
        let response = client
            .put(webhook_1_uri)
            .body(serde_json::to_string(&webhook).unwrap())
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);

        // 5. GET    /v1/netspots/webhook/1 : Should return the updated webhook
        let response = client.get(webhook_1_uri).dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let webhook_recv = response.into_json::<Webhook>().await.unwrap();
        assert_eq!(webhook, webhook_recv);

        // 6. DELETE /v1/netspots/webhook/1 : Should delete webhook
        let response = client.delete(webhook_1_uri).dispatch().await;
        assert_eq!(response.status(), Status::Ok);

        // 7. GET    /v1/netspots/webhooks  : List available webhooks (should be empty)
        let response = client.get(webhooks_uri).dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let webhooks = response.into_json::<WebhookList>().await.unwrap();
        assert!(webhooks.is_empty());

        setup.cleanup().await;
    }

    // This test does the following:
    //
    // 1.  POST   /v1/netspots/webhook     : With broken JSON, expecting 400 Bad Request
    // 2.  POST   /v1/netspots/webhook     : With incomplete JSON, expecting 422 Unprocessable Entity
    // 3.  GET    /v1/netspots/webhook/1   : Expecting 404 Not Found
    // 4.  GET    /v1/netspots/webhook/foo : Expecting 400 Bad Request
    // 5.  PUT    /v1/netspots/webhook/1   : Expecting 404 Not Found
    // 6.  PUT    /v1/netspots/webhook/foo : Expecting 400 Bad Request
    // 7.  DELETE /v1/netspots/webhook/1   : Expecting 404 Not Found
    // 8.  DELETE /v1/netspots/webhook/foo : Expecting 400 Bad Request
    // 9.  POST   /v1/netspots/webhook     : With valid JSON, expecting 201 Created
    // 10. PUT    /v1/netspots/webhook/1   : With broken JSON, expecting 400 Bad Request
    // 11. PUT    /v1/netspots/webhook/1   : With incomplete JSON, expecting 422 Unprocessable Entity
    #[tokio::test]
    async fn test_invalid_requests() {
        let setup = TestSetup::new().await;
        let client = &setup.client;
        let webhook_uri = "/v1/netspots/webhook";
        let webhook_1_uri = "/v1/netspots/webhook/1";
        let webhook_foo_uri = "/v1/netspots/webhook/foo";
        let broken_json = r#"{"name":broken_json}"#;
        let incomplete_json = r#"{"name":"incomplete_json"}"#;
        let valid_json = r#"{"name":"valid_json","address":"http://localhost:9000/"}"#;

        // 1.  POST   /v1/netspots/webhook     : With broken JSON, expecting 400 Bad Request
        let response = client.post(webhook_uri).body(broken_json).dispatch().await;
        assert_eq!(response.status(), Status::BadRequest);

        // 2.  POST   /v1/netspots/webhook     : With incomplete JSON, expecting 422 Unprocessable Entity
        let response = client
            .post(webhook_uri)
            .body(incomplete_json)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::UnprocessableEntity);

        // 3.  GET    /v1/netspots/webhook/1   : Expecting 404 Not Found
        let response = client.get(webhook_1_uri).dispatch().await;
        assert_eq!(response.status(), Status::NotFound);

        // 4.  GET    /v1/netspots/webhook/foo : Expecting 400 Bad Request
        let response = client.get(webhook_foo_uri).dispatch().await;
        assert_eq!(response.status(), Status::BadRequest);

        // 5.  PUT    /v1/netspots/webhook/1   : Expecting 404 Not Found
        let response = client.put(webhook_1_uri).body(valid_json).dispatch().await;
        assert_eq!(response.status(), Status::NotFound);

        // 6.  PUT    /v1/netspots/webhook/foo : Expecting 400 Bad Request
        let response = client
            .put(webhook_foo_uri)
            .body(valid_json)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::BadRequest);

        // 7.  DELETE /v1/netspots/webhook/1   : Expecting 404 Not Found
        let response = client.delete(webhook_1_uri).dispatch().await;
        assert_eq!(response.status(), Status::NotFound);

        // 8.  DELETE /v1/netspots/webhook/foo : Expecting 400 Bad Request
        let response = client.delete(webhook_foo_uri).dispatch().await;
        assert_eq!(response.status(), Status::BadRequest);

        // 9.  POST   /v1/netspots/webhook     : With valid JSON, expecting 201 Created
        let response = client.post(webhook_uri).body(valid_json).dispatch().await;
        assert_eq!(response.status(), Status::Created);

        // 10. PUT    /v1/netspots/webhook/1   : With broken JSON, expecting 400 Bad Request
        let response = client.put(webhook_1_uri).body(broken_json).dispatch().await;
        assert_eq!(response.status(), Status::BadRequest);

        // 11. PUT    /v1/netspots/webhook/1   : With incomplete JSON, expecting 422 Unprocessable Entity
        let response = client
            .put(webhook_1_uri)
            .body(incomplete_json)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::UnprocessableEntity);

        setup.cleanup().await;
    }
}
