extern crate test_patience;

use std::time::Duration;
use std::thread;
use std::io;

fn mock_client(port: u16, sleep: Duration) {
    thread::sleep(sleep);
    let _ = test_patience::Client::notify(port); // ignore errors when testing timeout
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! assert_close {
        ($left:expr, $right:expr) => ({
            let diff = $left - $right;
            if diff.as_secs() > 0 || diff.subsec_nanos() > 10_000_000 {
                panic!("assertion failed: `(time difference smaller than 10 ms)`, (diff: `{:?}`)", diff)
            }
        })
    }

    #[test]
    fn wait_for_client_0() {
        let server = test_patience::Server::new().expect("failed to create test-patience server");
        let port = server.port().expect("failed to get test-patience server port");

        thread::spawn(move || {
            mock_client(port, Duration::from_secs(0));
        });

        let wait_duration = server.wait(Duration::from_secs(5)).expect("failed to wait");

        assert_close!(wait_duration, Duration::from_secs(0));
    }

    #[test]
    fn wait_for_client_1() {
        let server = test_patience::Server::new().expect("failed to create test-patience server");
        let port = server.port().expect("failed to get test-patience server port");

        thread::spawn(move || {
            mock_client(port, Duration::from_secs(1));
        });

        let wait_duration = server.wait(Duration::from_secs(5)).expect("failed to wait");

        assert_close!(wait_duration, Duration::from_secs(1));
    }

    #[test]
    fn wait_for_client_2() {
        let server = test_patience::Server::new().expect("failed to create test-patience server");
        let port = server.port().expect("failed to get test-patience server port");

        thread::spawn(move || {
            mock_client(port, Duration::from_secs(2));
        });

        let wait_duration = server.wait(Duration::from_secs(5)).expect("failed to wait");

        assert_close!(wait_duration, Duration::from_secs(2));
    }

    #[test]
    fn wait_for_client_buffered() {
        let server = test_patience::Server::new().expect("failed to create test-patience server");
        let port = server.port().expect("failed to get test-patience server port");

        thread::spawn(move || {
            mock_client(port, Duration::from_secs(0));
        });

        thread::sleep(Duration::from_secs(1));

        let wait_duration = server.wait(Duration::from_secs(5)).expect("failed to wait");

        assert_close!(wait_duration, Duration::from_secs(0));
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
}
