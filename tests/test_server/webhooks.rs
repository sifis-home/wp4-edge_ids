use actix_web::web::Data;
use actix_web::{get, post, put, web::Bytes, App, HttpRequest, HttpResponse, HttpServer};
use std::error::Error;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};
use tokio::time::timeout;

struct Channels {
    get_tx: Mutex<mpsc::Sender<()>>,
    post_tx: Mutex<mpsc::Sender<()>>,
    put_tx: Mutex<mpsc::Sender<()>>,
}

#[get("/webhook")]
async fn webhook_get(request: HttpRequest, body: Bytes, channels: Data<Channels>) -> HttpResponse {
    if let Some(test_header) = request.headers().get("test-header") {
        if test_header == "Testing GET Webhook" {
            println!("[GET OK] {}", std::str::from_utf8(&body).unwrap());
            channels.get_tx.lock().await.send(()).await.unwrap();
        }
    }
    HttpResponse::Ok().finish()
}

#[post("/webhook")]
async fn webhook_post(request: HttpRequest, body: Bytes, channels: Data<Channels>) -> HttpResponse {
    if let Some(test_header) = request.headers().get("test-header") {
        if test_header == "Testing POST Webhook" {
            println!("[POST OK] {}", std::str::from_utf8(&body).unwrap());
            channels.post_tx.lock().await.send(()).await.unwrap();
        }
    }
    HttpResponse::Ok().finish()
}

#[put("/webhook")]
async fn webhook_put(request: HttpRequest, body: Bytes, channels: Data<Channels>) -> HttpResponse {
    if let Some(test_header) = request.headers().get("test-header") {
        if test_header == "Testing PUT Webhook" {
            println!("[PUT OK] {}", std::str::from_utf8(&body).unwrap());
            channels.put_tx.lock().await.send(()).await.unwrap();
        }
    }
    HttpResponse::Ok().finish()
}

pub async fn test() -> Result<(), Box<dyn Error>> {
    // Creating channels to receive notify of received messages
    let (get_tx, mut get_rx) = mpsc::channel(1);
    let (post_tx, mut post_rx) = mpsc::channel(1);
    let (put_tx, mut put_rx) = mpsc::channel(1);

    // Start webhook server
    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Channels {
                get_tx: Mutex::new(get_tx.clone()),
                post_tx: Mutex::new(post_tx.clone()),
                put_tx: Mutex::new(put_tx.clone()),
            }))
            .service(webhook_get)
            .service(webhook_post)
            .service(webhook_put)
    })
    .bind(("127.0.0.1", 28001))?
    .run();
    let server_handle = http_server.handle();
    tokio::spawn(http_server);

    // Collecting results
    let mut results = Vec::new();

    // Register testing webhooks
    results.push(register_webhook("GET").await);
    results.push(register_webhook("POST").await);
    results.push(register_webhook("PUT").await);

    // We should receive messages from all channels
    let wait_for_messages = async {
        get_rx.recv().await;
        post_rx.recv().await;
        put_rx.recv().await;
    };
    results.push(
        timeout(Duration::from_secs(10), wait_for_messages)
            .await
            .map_err(|_| "Webhook test timed out".into()),
    );

    // Remove webhooks for cleaner shutdown
    for id in 1..=3 {
        results.push(delete_webhook(id).await);
    }

    server_handle.stop(true).await;

    // Go trough results and report first error if any
    for result in results {
        result?;
    }

    Ok(())
}

async fn register_webhook(method: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:28000/v1/netspots/webhook")
        .header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        )
        .body(format!(
            r#"{{
	"name": "Testing GET Webhook",
	"address": "http://127.0.0.1:28001/webhook",
	"method": "{method}",
	"headers": {{
		"test-header": "Testing {method} Webhook"
	}},
	"type": "both"
}}"#
        ))
        .send()
        .await?;
    let status = response.status();
    if status != reqwest::StatusCode::CREATED {
        return Err(format!(
            "Unexpected response from the server: {} {}",
            status.as_str(),
            status.canonical_reason().unwrap_or("")
        )
        .into());
    }
    Ok(())
}

async fn delete_webhook(id: i32) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .delete(format!("http://127.0.0.1:28000/v1/netspots/webhook/{id}"))
        .send()
        .await?;
    let status = response.status();
    if status != reqwest::StatusCode::OK {
        return Err(format!(
            "Unexpected response from the server: {} {}",
            status.as_str(),
            status.canonical_reason().unwrap_or("")
        )
        .into());
    }
    Ok(())
}
