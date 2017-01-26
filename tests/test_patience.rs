extern crate test_patience;

use std::time::Duration;
use std::thread;
use std::io;

fn mock_client(port: u16, sleep: Duration) {
    thread::sleep(sleep);
    let _ = test_patience::Client::notify(port); // ignore errors when testing timeout
}

#[test]
fn wait_for_client_0() {
    let server = test_patience::Server::new().expect("failed to create test-patience server");
    let port = server.port().expect("failed to get test-patience server port");

    thread::spawn(move || {
        mock_client(port, Duration::from_secs(0));
    });

    let result = server.wait(Duration::from_secs(5));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_secs(), 0);
}

#[test]
fn wait_for_client_1() {
    let server = test_patience::Server::new().expect("failed to create test-patience server");
    let port = server.port().expect("failed to get test-patience server port");

    thread::spawn(move || {
        mock_client(port, Duration::from_secs(1));
    });

    let result = server.wait(Duration::from_secs(5));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_secs(), 1);
}

#[test]
fn wait_for_client_2() {
    let server = test_patience::Server::new().expect("failed to create test-patience server");
    let port = server.port().expect("failed to get test-patience server port");

    thread::spawn(move || {
        mock_client(port, Duration::from_secs(2));
    });

    let result = server.wait(Duration::from_secs(5));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_secs(), 2);
}

#[test]
fn wait_for_client_buffered() {
    let server = test_patience::Server::new().expect("failed to create test-patience server");
    let port = server.port().expect("failed to get test-patience server port");

    thread::spawn(move || {
        mock_client(port, Duration::from_secs(0));
    });

    thread::sleep(Duration::from_secs(1));

    let result = server.wait(Duration::from_secs(5));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_secs(), 0);
}

#[test]
fn wait_for_client_timeout() {
    let server = test_patience::Server::new().expect("failed to create test-patience server");
    let port = server.port().expect("failed to get test-patience server port");

    thread::spawn(move || {
        mock_client(port, Duration::from_secs(2));
    });

    let result = server.wait(Duration::from_secs(1));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::TimedOut);
}
