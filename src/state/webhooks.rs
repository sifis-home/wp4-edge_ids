use crate::structures::statistics::Message;
use crate::structures::webhooks::{Webhook, WebhookRequestMethod, WebhookStatsType, Webhooks};
use crate::tasks::RunChecker;
use reqwest::header;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

pub struct WebhookManager {
    webhooks: Arc<RwLock<Webhooks>>,
}

impl WebhookManager {
    pub fn new(
        webhooks: Webhooks,
        messages_rx: broadcast::Receiver<Message>,
        run_checker: RunChecker,
    ) -> WebhookManager {
        let webhooks = Arc::new(RwLock::new(webhooks));
        tokio::spawn(webhook_sender_task(
            webhooks.clone(),
            messages_rx,
            run_checker,
        ));
        WebhookManager { webhooks }
    }

    pub fn update(&self, webhooks: Webhooks) {
        if let Ok(mut webhooks_write_guard) = self.webhooks.write() {
            *webhooks_write_guard = webhooks;
        }
    }
}

// Worker task
//--------------------------------------------------------------------------------------------------

async fn webhook_sender_task(
    webhooks: Arc<RwLock<Webhooks>>,
    mut message_rx: broadcast::Receiver<Message>,
    mut run_checker: RunChecker,
) {
    println!("Webhook sender started.");
    while run_checker.keep_running() {
        tokio::select! {
            Ok(message) = message_rx.recv() => message_handler(message, &webhooks, &run_checker),
            _ = run_checker.shutdown_recv() => {},
        }
    }
    println!("Webhook sender stopped.");
}

fn message_handler(message: Message, webhooks: &Arc<RwLock<Webhooks>>, run_checker: &RunChecker) {
    if let Ok(json) = message.to_json() {
        let shared_message = Arc::new(json);
        for (id, webhook) in &*webhooks.read().unwrap() {
            let shared_webhook = Arc::new(webhook.clone());
            match (&webhook.stats_type, &message) {
                (WebhookStatsType::Both, _)
                | (WebhookStatsType::Alarms, Message::Alarm(_))
                | (WebhookStatsType::Data, Message::Data(_)) => {
                    tokio::spawn(send_message(
                        *id,
                        shared_webhook.clone(),
                        shared_message.clone(),
                        run_checker.clone(),
                    ));
                }
                (_, _) => {} // Should not be send for this webhook
            }
        }
    }
}

async fn send_message(
    id: i32,
    webhook: Arc<Webhook>,
    message: Arc<String>,
    _: RunChecker, // Ensuring that message sending is done before application quits
) {
    // Making headers for the request
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    for (key, value) in &webhook.headers {
        let header_name = header::HeaderName::from_bytes(key.as_bytes());
        let header_value = header::HeaderValue::from_bytes(value.as_bytes());
        match (header_name, header_value) {
            (Ok(header_name), Ok(header_value)) => {
                headers.insert(header_name, header_value);
            }
            (_, _) => {
                eprintln!(
                    "Warning: Invalid header in webhook({}) {}: name = {}, value = {}",
                    id, webhook.name, key, value
                );
            }
        }
    }

    // Sending the request
    let client = reqwest::Client::new();
    let result = match webhook.method {
        WebhookRequestMethod::Get => client.get(&webhook.address),
        WebhookRequestMethod::Post => client.post(&webhook.address),
        WebhookRequestMethod::Put => client.put(&webhook.address),
    }
    .headers(headers)
    .body((*message).clone())
    .send()
    .await;

    // Checking the response
    match result {
        Ok(response) => {
            if response.status() != 200 {
                eprintln!(
                    "Warning: Webhook({}) {} host responded: {:#?}",
                    id, webhook.name, response
                );
            }
        }
        Err(err) => {
            eprintln!(
                "Warning: Could not send message to webhook({}) {}: {}",
                id, webhook.name, err
            );
        }
    }
}
