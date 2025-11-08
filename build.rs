// build.rs
use chrono::{Local, Datelike};
fn main() {
    let now = Local::now();
    let date = format!("{:04}{:02}{:02}", now.year(), now.month(), now.day());
    let year = now.year();

    println!("cargo:rustc-env=BUILD_DATE={}", date);
    println!("cargo:rustc-env=BUILD_YEAR={}", year);
}


