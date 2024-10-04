//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get the current timestamp in seconds since UNIX epoch.
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Set the TEST_FOO environment variable
    println!("cargo:rerun-if-changed=build.rs"); // Instruct Cargo to rerun this script if build.rs changes
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Set feature flag conditionally based on the current time
    if timestamp % 2 == 0 { // Example condition; you can customize this
        // Enable `pass` feature if the timestamp is even
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }
}

