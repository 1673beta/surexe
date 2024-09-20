use std::process::Command;

use anyhow::{Ok, Result};

pub fn execute(cmd: &str) -> Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}