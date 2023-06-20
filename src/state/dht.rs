use crate::structures::dht::{DhtMessage, RequestPostTopicUUID};
use crate::structures::statistics::Message;
use crate::tasks::RunChecker;
use tokio::sync::broadcast;

pub async fn dht_message_sender(
    api_url: String,
    ip_addresses: Vec<String>,
    mut message_rx: broadcast::Receiver<Message>,
    mut run_checker: RunChecker,
) {
    println!("DHT message sender started.");
    println!("  Using API URL: {api_url}");
    while run_checker.keep_running() {
        tokio::select! {
            Ok(message) = message_rx.recv() => handle_message(&api_url, &ip_addresses, message).await,
            _ = run_checker.shutdown_recv() => {},
        }
    }
    println!("DHT message sender stopped.")
}

async fn handle_message(api_url: &str, ip_addresses: &[String], message: Message) {
    match message {
        Message::Alarm(message) => {
            let dht_message = DhtMessage {
                request_post_topic_uuid: RequestPostTopicUUID::new(ip_addresses, *message),
            };
            let json_message = serde_json::to_string(&dht_message);
            if let Ok(message) = json_message {
                tokio::spawn(send_message(api_url.to_string(), message));
            }
        }
        Message::Data(_) => (), // Skipping these for now
    }
}

async fn send_message(api_url: String, message: String) {
    let client = reqwest::Client::new();
    if let Err(err) = client.post(api_url).body(message).send().await {
        eprintln!("{err}");
    }
}
