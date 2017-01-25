//! `test-patience` is a utility to synchronize the startup of applications that are part of an integration test and the test itself.
//!
//! The test has to create an instance of the `Server` struct, which starts a TCP server and returns a port number.
//! That port number needs to be sent to the application that is needed to execute the test.
//! This could be done using an environment variable, an argument or a configuration file.
//! After the start of the application has been initiated, the `wait` method needs to be called.
//! It blocks the currently running thread until either the starting application has signaled its successful start or the `timeout` period has expired.
//!
//! When the application is ready, it has to create an instance of the `Client` struct and call the `notify` method with the correct port number.
//! After that the thread of the test continues executing.
#![warn(missing_docs)]

use std::net::{TcpListener, TcpStream};
use std::time::{Instant, Duration};
use std::io::{Result, Error, ErrorKind};
use std::io::prelude::*;
use std::thread;

/// Entry point for the application that needs to be synchronized
pub struct Client;

impl Client {
    /// Notify the server that the client has started successfully
    pub fn notify(port: u16) -> Result<()> {
        let mut stream = TcpStream::connect(("127.0.0.1", port))?;
        stream.write_all(b"done")?;
        Ok(())
    }
}

/// Entry point for the test, waiting for the application to start
pub struct Server {
    listener: TcpListener,
}

impl Server {
    /// Start new TCP server, waiting for the application's startup notification
    pub fn new() -> Result<Server> {
        Ok(Server {
            listener: TcpListener::bind(("127.0.0.1", 0))?
        })
    }

    /// Get the port number of the TCP Server
    ///
    /// This port number has to sent to the application.
    pub fn port(&self) -> Result<u16> {
        Ok(self.listener.local_addr()?.port())
    }

    /// Block the currently running thread until either the starting application has signaled its successful start or the `timeout` period has expired
    ///
    /// Returns the duration for which was waited or an error in case of a timeout or invalid startup notification.
    pub fn wait(self, timeout: Duration) -> Result<Duration> {
        self.listener.set_nonblocking(true)?;

        let start = Instant::now();
        while start.elapsed() < timeout {
            match self.listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = Vec::new();
                    stream.read_to_end(&mut buf)?;
                    if buf == b"done" {
                        return Ok(start.elapsed())
                    } else {
                        return Err(Error::new(ErrorKind::Other, "wrong startup notification received"))
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => return Err(e)
            }
            thread::sleep(Duration::from_millis(1));
        }
        Err(Error::new(ErrorKind::TimedOut, "did not receive startup notification"))
    }
}
