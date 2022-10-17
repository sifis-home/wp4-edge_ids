use std::os::unix::io::AsRawFd;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::{broadcast, mpsc};

// Socket use decides location for the Unix socket file
pub enum SocketUse {
    Alarm,
    Data,
}

pub fn start_listener_task(
    socket_use: SocketUse,
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

    // Start listener
    let name = match socket_use {
        SocketUse::Alarm => "Alarm",
        SocketUse::Data => "Data",
    };
    let _ = tokio::spawn(async move {
        println!("{} socket listener started.", name);
        tokio::select! {
            _ = listener_loop(listener, shutdown_request_rx.resubscribe(), shutdown_complete_tx.clone(), name) => {}
            _ = shutdown_request_rx.recv() => {}
        }
        println!("{} socket listener stopped.", name);
        drop(shutdown_complete_tx);
    });

    Ok(())
}

async fn listener_loop(
    listener: UnixListener,
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
                // Start a new task for the connection
                tokio::spawn(async move {
                    handle_connection(stream, shutdown_request_rx, shutdown_complete_tx, name)
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
                        // Todo: Forward to exporters via channel
                        println!("{}@{}: {}", name, fd, String::from_utf8_lossy(&buffer));
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
