# test-patience

[![Build Status](https://travis-ci.org/dfaust/test-patience.svg?branch=master)](https://travis-ci.org/dfaust/test-patience)
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/dfaust/test-patience?svg=true)](https://ci.appveyor.com/project/dfaust/test-patience)
[![Crate version](https://img.shields.io/crates/v/test-patience.svg)](https://crates.io/crates/test-patience)
[![Documentation](https://img.shields.io/badge/documentation-docs.rs-df3600.svg)](https://docs.rs/test-patience)

test-patience is a utility to synchronize the startup of applications that are part of an integration test and the test itself.

When writing integration tests, it is often necessary to launch applications that are part of the test.
This can take some time and if the test has to wait until the application is running, valuable time is lost when waiting for a fixed duration.
Also waiting for a fixed duration can still lead to test failures without producing a clear error message.
test-patience waits exactly until the starting application signals that it's ready or a specified timeout period has passed.

The test has to create an instance of the `Server` struct, which starts a TCP server and returns a port number.
That port number needs to be sent to the application that is needed to execute the test.
This could be done using an environment variable, an argument or a configuration file.
After the start of the application has been initiated, the `wait` method needs to be called.
It blocks the currently running thread until either the starting application has signaled its successful start or the `timeout` period has passed.

When the application is ready, it has to create an instance of the `Client` struct and call the `notify` method with the correct port number.
After that the thread of the test continues executing.

In order to disable startup notifications in release builds, use `cfg!(debug_assertions)` (see [conditional compilation](https://doc.rust-lang.org/reference.html#conditional-compilation)).

## Examples

Application

```rust
use std::env;

// initialize application (eg. connect to database server)
let db_connection = get_db_connection();

// notify test in case the environment variable TEST_PATIENCE_PORT is set
if let Some(port) = env::var("TEST_PATIENCE_PORT").ok()
                    .and_then(|s| s.parse::<u16>().ok()) {
    test_patience::Client::notify(port).unwrap();
}
```

Test

```rust
use std::time::Duration;
use std::process;

let server = test_patience::Server::new().unwrap();
let port = server.port().unwrap();

let process = process::Command::new("path/to/application")
    .env("TEST_PATIENCE_PORT", format!("{}", port))
    .spawn()
    .unwrap();

server.wait(Duration::from_secs(5)).unwrap();
```
