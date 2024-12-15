use serde_json::Value;
use std::process::Command;

fn run_command(command: &str) -> String {
    let args: Vec<&str> = command.split(" ").collect();
    let output = Command::new(args[0]).args(&args[1..]).output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.to_string()
        }
        Err(error) => {
            println!("Command failed: {command}");
            eprintln!("error: {}", error);
            "".to_string()
        }
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

pub fn run_lsblk(device: &str) -> Option<Value> {
    let command = "lsblk -J -o name,size,type,mountpoint";
    let output = run_command(command);
    if output.is_empty() {
        return None;
    }

    let devices: serde_json::Value = serde_json::from_str(&output).unwrap();
    let devices = devices["blockdevices"].as_array().unwrap();
    find_first(devices, device)
}
