use serde_json::Value;
use std::io::{Error, ErrorKind};
use std::process::Command;

fn run_command(command: &str) -> Result<String, Error> {
    let args: Vec<&str> = command.split(" ").collect();
    let output = Command::new(args[0]).args(&args[1..]).output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.to_string())
        }
        Err(error) => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Command: '{}'\nError: '{}'", command, error),
        )),
    }
}

fn find_first(devices: &Vec<Value>, target: &str) -> Option<Value> {
    for device in devices {
        if device["name"] == target {
            return Some(device.clone());
        }

        // Recursive search
        if let Some(children) = device["children"].as_array() {
            if let Some(found) = find_first(children, target) {
                return Some(found);
            }
        }
    }

    None
}

pub fn run_lsblk(device: &str) -> Result<Value, Error> {
    let command = "lsblk -J -o name,size,type,mountpoint";
    match run_command(command) {
        Ok(output) => {
            let devices: Value = serde_json::from_str(&output).unwrap();
            let devices = devices["blockdevices"].as_array().unwrap();
            match find_first(devices, device) {
                Some(found) => Ok(found),
                None => Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Device '{}' not found", device),
                )),
            }
        }
        Err(error) => Err(error),
    }
}
