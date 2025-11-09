// build.rs
use chrono::{Local, Datelike, Timelike};
use std::{env, process::Command};
fn main() {

    // 在 `Cargo.toml` 的 `package` 中 必须要添加 `description` 说明软件用途
    // 作者,新项目要修改
    let author = "WenJun";


    let now = Local::now();
    let year = now.year();
    let time = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.offset().to_string()
    );

    // 获取第一次提交的年份
    let output = Command::new("git")
        .args(["log", "--reverse", "--format=%ad", "--date=format:%Y"])
        .output()
        .expect("Failed to run git command");

    let first_year = String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()
        .unwrap_or("2025")  // 如果 git 不可用则默认 2025
        .to_string();
    let first_year: i32 = first_year.parse().unwrap_or(2025);

    // 短Hash
    let shash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap())
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string();
    println!("cargo:rustc-env=BUILD_SHASH={}",shash);
    // 长Hash
    let lhash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap())
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string();
    println!("cargo:rustc-env=BUILD_LHASH={}",lhash);

    // 程序名称
    let program_name = env::var("CARGO_PKG_NAME").unwrap();
    println!("cargo:rustc-env=BUILD_NAME={}",program_name);


    // 程序版本
    let program_version = format!("V:{}-{}",env::var("CARGO_PKG_VERSION").unwrap(),shash);
    println!("cargo:rustc-env=BUILD_VERSION={}",program_version);

    
    println!("cargo:rustc-env=BUILD_AUTHOR={}",author);

    // 编译时间
    println!("cargo:rustc-env=BUILD_TIME={}",time);

    // 短关于
    let program_short_about = env::var("CARGO_PKG_DESCRIPTION").unwrap();
    println!("cargo:rustc-env=BUILD_ABOUT={}",program_short_about);

    //版权
    let program_copyright = if year == first_year {
    format!("Copyright (c) {} {}. All rights reserved.", first_year, author)
    } 
    else {
        format!("Copyright (c) {}-{} {}. All rights reserved.", first_year, year, author)
    };

    //let program_copyright = format!("Copyright (c) {}-{} {}. All rights reserved.", first_year, year, author);
    println!("cargo:rustc-env=BUILD_COPYRIGHT={}",program_copyright);


    // 当 build.rs 本身或其它依赖文件改变时重新运行 build.rs
    println!("cargo:rerun-if-changed=build.rs");

    // 让 build.rs 在 HEAD 变化时重新运行
    println!("cargo:rerun-if-changed=.git/HEAD");
}


