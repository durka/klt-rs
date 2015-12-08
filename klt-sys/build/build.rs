use std::process::{Command, Stdio};
#[cfg(unix)]use std::os::unix::io::{AsRawFd, FromRawFd};
use std::fs::File;
use std::fmt::Display;

fn print_meta<A: Display, B: Display>(key: A, val: B) {
    println!("cargo:{}={}", key, val);
}

macro_rules! format_meta {
    ($key:expr, $($val:tt)*) => {
        print_meta($key, format!($($val)*))
    }
}

#[cfg(not(unix))]
fn build_klt() {
    panic!("Unix only");
}

#[cfg(unix)]
fn build_klt() {
    const NAME: &'static str = "klt";
    const DIR: &'static str = "lib/klt";

    print_meta("rerun-if-changed", DIR);

    let log = File::create(format!("build/{}.log", NAME))
                   .unwrap_or_else(|e| panic!("could not create {} build log: {}", NAME, e));
    let status = Command::new("make")
                         .args(&["-C", DIR])
                         .stdout(unsafe { Stdio::from_raw_fd(log.as_raw_fd()) })
                         .status()
                         .unwrap_or_else(|e| panic!("could not run make for {}: {}", NAME, e));
    if !status.success() {
        panic!("failed to build {}: {}", NAME, status);
    }

    format_meta!("rustc-link-lib", "static={}", NAME);
    print_meta("rustc-link-search", DIR);
}

fn main() {
    // build script depends on itself
    print_meta("rerun-if-changed", "build/build.rs");

    // build KLT dependency
    build_klt();
}

