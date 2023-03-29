use assert_cmd::cargo::cargo_bin;
use nix::sys::signal;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use reqwest::StatusCode;
use std::error::Error;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;
use tempfile::TempDir;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use tokio::process::{ChildStderr, ChildStdout};
use tokio::time::timeout;

#[tokio::test]
async fn test_server_binary() -> Result<(), Box<dyn Error>> {
    // Running with valgrind?
    if let Ok(value) = std::env::var("LD_PRELOAD") {
        if value.contains("/valgrind/") || value.contains("/vgpreload") {
            println!("The test was skipped because we cannot send SIGTERM for");
            println!("the server when it is run within the Valgrind checking tool.");
            return Ok(());
        }
    }

    // Making temporary directory for testing
    let tmp_dir = TempDir::new()?;

    // Adding empty files, which netspot listener should delete
    let mut fake_socket_file = PathBuf::from(tmp_dir.path());
    fake_socket_file.push("netspot_alarm.socket");
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(fake_socket_file)?;
    let mut fake_socket_file = PathBuf::from(tmp_dir.path());
    fake_socket_file.push("netspot_data.socket");
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(fake_socket_file)?;

    // Set server listen adders and port
    std::env::set_var("ROCKET_ADDRESS", "127.0.0.1");
    std::env::set_var("ROCKET_PORT", "28000");

    // Enable message logger
    std::env::set_var("SHOW_NETSPOT_MESSAGES", "1");

    // Start the server
    let server_bin_path = cargo_bin("netspot_control");
    let mut server = tokio::process::Command::new(&server_bin_path)
        .args(["-r", tmp_dir.path().to_str().ok_or("Invalid tmp path")?])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let stdout = server.stdout.take().unwrap();
    let stderr = server.stderr.take().unwrap();
    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();
    if timeout(
        Duration::from_secs(10),
        wait_for_message(
            &mut stdout_reader,
            &mut stderr_reader,
            "Rocket has launched from",
            false,
        ),
    )
    .await
    .is_err()
    {
        return Err("Wait for the launch complete test timed out".into());
    }

    // Testing that message printer is working
    if timeout(
        Duration::from_secs(10),
        wait_for_message(&mut stdout_reader, &mut stderr_reader, "Data: ", true),
    )
    .await
    .is_err()
    {
        return Err("Wait for the stat message test timed out".into());
    };

    // Testing the test API and alarm message output
    send_test_alarm().await?;
    if timeout(
        Duration::from_secs(10),
        wait_for_message(&mut stdout_reader, &mut stderr_reader, "Alarm: ", false),
    )
    .await
    .is_err()
    {
        return Err("Alarm message test timed out".into());
    };

    // Testing the graceful shutdown
    let server_pid = Pid::from_raw(server.id().unwrap() as i32);
    if timeout(
        Duration::from_secs(10),
        test_graceful_shutdown(&mut stdout_reader, &mut stderr_reader, server_pid),
    )
    .await
    .is_err()
    {
        return Err("Graceful shutdown test timed out".into());
    };

    Ok(())
}

async fn wait_for_message(
    stdout_reader: &mut Lines<BufReader<ChildStdout>>,
    stderr_reader: &mut Lines<BufReader<ChildStderr>>,
    contains_text: &str,
    stderr_is_ok: bool,
) -> Result<(), Box<dyn Error>> {
    loop {
        tokio::select! {
            result = stdout_reader.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        println!("stdout: {line}");
                        if line.contains(contains_text) {
                            return Ok(());
                        }
                    },
                    Err(err) => return Err(err.into()),
                    _ => (),
                }
            }
            result = stderr_reader.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        eprintln!("stderr: {line}");
                        if !stderr_is_ok {
                            return Err("Unexpected stderr output".into())
                        }
                    }
                    Err(err) => return Err(err.into()),
                    _ => (),
                }
            }
        }
    }
}

async fn send_test_alarm() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:28000/v1/netspots/test/alarm")
        .header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        )
        .body("{}") // Default alarm is generated with empty JSON
        .send()
        .await?;
    let status = response.status();
    if status != StatusCode::CREATED {
        return Err(format!(
            "Unexpected response from the server: {} {}",
            status.as_str(),
            status.canonical_reason().unwrap_or("")
        )
        .into());
    }
    Ok(())
}

async fn test_graceful_shutdown(
    stdout_reader: &mut Lines<BufReader<ChildStdout>>,
    stderr_reader: &mut Lines<BufReader<ChildStderr>>,
    server_pid: Pid,
) -> Result<(), Box<dyn Error>> {
    // Sending SIGINT once per second
    let mut sigint_interval = tokio::time::interval(Duration::from_secs(1));

    // We wait for both stdout and stderr streams to close
    let mut stdout_closed = false;
    let mut stderr_closed = false;

    loop {
        tokio::select! {
            result = stdout_reader.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        println!("stdout: {line}");
                    },
                    Ok(None) => {
                        stdout_closed = true;
                        if stderr_closed { break };
                    },
                    Err(err) => return Err(err.into()),
                }
            }
            result = stderr_reader.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        eprintln!("stderr: {line}");
                    }
                    Ok(None) => {
                        stderr_closed = true;
                        if stdout_closed { break };
                    },
                    Err(err) => return Err(err.into()),
                }
            }
            _ = sigint_interval.tick() => {
                println!("Sending SIGINT to {server_pid}");
                signal::kill(server_pid, Some(Signal::SIGINT)).expect("Could not send SIGTERM");
            }
        }
    }
    Ok(())
}
