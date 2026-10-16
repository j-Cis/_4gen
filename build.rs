// build.rs

use std::time::SystemTime;
use temporal_fmt::Temporal;

fn main() {
    let now = SystemTime::now();
    let date_at_publish = Temporal::format_system_time(
        now,
        "WYYY-WW-D:hh:mm:ss:tt",
    );
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", date_at_publish);
}