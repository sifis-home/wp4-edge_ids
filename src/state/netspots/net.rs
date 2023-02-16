use crate::structures::statistics::{AlarmMessage, DataMessage, Message};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::{broadcast, mpsc};

// Socket use decides location for the Unix socket file
#[derive(Copy, Clone)]
pub enum SocketUse {
    Alarm,
    Data,
}

pub fn start_listener_task(
    socket_use: SocketUse,
    message_tx: broadcast::Sender<Message>,
    mut shutdown_request_rx: broadcast::Receiver<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
) -> Result<(), String> {
    let socket_path = Path::new(match socket_use {
        SocketUse::Alarm => "/tmp/netspot_alarm.socket",
        SocketUse::Data => "/tmp/netspot_data.socket",
    });

    // Listening fails if socket file already exists.
    // Therefore, we try to remove any existing socket file.
    if socket_path.exists() {
        if let Err(err) = std::fs::remove_file(socket_path) {
            return Err(err.to_string());
        }
    }

    // Create a new listener socket
    let listener = match UnixListener::bind(socket_path) {
        Ok(listener) => listener,
        Err(err) => return Err(err.to_string()),
    };

    let name = match socket_use {
        SocketUse::Alarm => "Alarm",
        SocketUse::Data => "Data",
    };

    // Start listener
    tokio::spawn(async move {
        println!("{} socket listener started.", name);
        tokio::select! {
            _ = listener_loop(listener, socket_use, message_tx,
                shutdown_request_rx.resubscribe(), shutdown_complete_tx.clone(), name) => {}
            _ = shutdown_request_rx.recv() => {}
        }
        println!("{} socket listener stopped.", name);
        drop(shutdown_complete_tx);
    });

    Ok(())
}

async fn listener_loop(
    listener: UnixListener,
    socket_use: SocketUse,
    message_tx: broadcast::Sender<Message>,
    shutdown_request_rx: broadcast::Receiver<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
    name: &'static str,
) {
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                // Clone channels for the new task
                let shutdown_request_rx = shutdown_request_rx.resubscribe();
                let shutdown_complete_tx = shutdown_complete_tx.clone();
                let message_tx = message_tx.clone();
                // Start a new task for the connection
                tokio::spawn(async move {
                    handle_connection(
                        stream,
                        socket_use,
                        message_tx,
                        shutdown_request_rx,
                        shutdown_complete_tx,
                        name,
                    )
                    .await;
                });
            }
            Err(err) => {
                eprintln!("Listener error: {}", err);
                break;
            }
        }
    }
}

async fn handle_connection(
    stream: UnixStream,
    socket_use: SocketUse,
    message_tx: broadcast::Sender<Message>,
    mut shutdown_request_rx: broadcast::Receiver<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
    name: &'static str,
) {
    let fd = stream.as_raw_fd();
    println!("{} connection in file descriptor {} connected.", name, fd);
    let mut reader = BufReader::new(stream);
    let mut buffer = Vec::new();
    loop {
        tokio::select! {
            result = reader.read_until(b'}', &mut buffer) => {
                match result {
                    Ok(0) => break, // Disconnected
                    Ok(_) => {
                        parse_and_send(&socket_use, &buffer, &message_tx);
                        buffer.clear();
                    },
                    Err(err) => {
                        eprintln!("Unexpected error: {}.", err);
                        break;
                    }
                }
            }
            _ = shutdown_request_rx.recv() => {
                break;
            }
        }
    }
    println!(
        "{} connection in file descriptor {} disconnected.",
        name, fd
    );
    drop(shutdown_complete_tx);
}

fn parse_and_send(
    socket_use: &SocketUse,
    json_bytes: &[u8],
    message_tx: &broadcast::Sender<Message>,
) {
    // Try to convert u8 vector to str
    if let Ok(json) = std::str::from_utf8(json_bytes) {
        match socket_use {
            SocketUse::Alarm => {
                if let Ok(message) = serde_json::from_str::<AlarmMessage>(json) {
                    let _ = message_tx.send(Message::Alarm(Box::new(message)));
                }
            }
            SocketUse::Data => {
                if let Ok(message) = serde_json::from_str::<DataMessage>(json) {
                    let _ = message_tx.send(Message::Data(Box::new(message)));
                }
            }
        }
    } else {
        eprintln!("Warning: Received invalid UTF-8 from netspot");
    }
}
