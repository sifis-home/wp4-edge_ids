use crate::structures::statistics::{AlarmMessage, DataMessage, Message};
use crate::tasks::RunChecker;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::broadcast;

// Socket use decides location for the Unix socket file
#[derive(Copy, Clone)]
pub enum SocketUse {
    Alarm,
    Data,
}

pub fn start_listener_task(
    socket_use: SocketUse,
    message_tx: broadcast::Sender<Message>,
    run_checker: RunChecker,
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

    // Start listener task
    tokio::spawn(listener_task(
        listener,
        socket_use,
        message_tx,
        name,
        run_checker,
    ));

    Ok(())
}

async fn listener_task(
    listener: UnixListener,
    socket_use: SocketUse,
    message_tx: broadcast::Sender<Message>,
    name: &'static str,
    mut run_checker: RunChecker,
) {
    println!("{name} socket listener started.");
    while run_checker.keep_running() {
        tokio::select! {
            result = listener.accept() => {
                match result {
                    Ok((stream, _)) => { tokio::spawn(handle_connection(stream, socket_use, message_tx.clone(), name, run_checker.clone())); }
                    Err(err) => {
                        eprintln!("Listener error: {}", err);
                        break;
                    }
                }
            }
            _ = run_checker.shutdown_recv() => {},
        }
    }
    println!("{name} socket listener stopped.");
}

async fn handle_connection(
    stream: UnixStream,
    socket_use: SocketUse,
    message_tx: broadcast::Sender<Message>,
    name: &'static str,
    mut run_checker: RunChecker,
) {
    let fd = stream.as_raw_fd();
    println!("{} connection in file descriptor {} connected.", name, fd);
    let mut reader = BufReader::new(stream);
    let mut buffer = Vec::new();
    while run_checker.keep_running() {
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
            _ = run_checker.shutdown_recv() => {},
        }
    }
    println!(
        "{} connection in file descriptor {} disconnected.",
        name, fd
    );
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
