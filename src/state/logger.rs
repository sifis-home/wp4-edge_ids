use crate::structures::statistics::Message;
use crate::tasks::RunChecker;
use termion::{color, style};
use tokio::sync::broadcast;

pub async fn message_printer(
    mut message_rx: broadcast::Receiver<Message>,
    mut run_checker: RunChecker,
) {
    println!("Message printer started.");
    while run_checker.keep_running() {
        tokio::select! {
            Ok(message) = message_rx.recv() => print_message(message),
            _ = run_checker.shutdown_recv() => {},
        }
    }
    println!("Message printer stopped.")
}

fn print_message(message: Message) {
    if let Ok(json) = message.to_json() {
        match message {
            Message::Alarm(_) => {
                println!(
                    "{}Alarm: {}{}",
                    color::Fg(color::Yellow),
                    json,
                    style::Reset
                );
            }
            Message::Data(_) => {
                println!("Data: {json}");
            }
        }
    }
}
