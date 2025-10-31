use std::io::{self, Write};
use std::process::Command;

fn main() {
    let mut device: Option<String> = None;
    let mut iso_file: Option<String> = None;

    loop {
        println!("\n1. Select Device");
        println!("2. Select ISO File");
        println!("3. Run / Write to USB");
        println!("4. Exit");

        if let Some(d) = &device {
            println!("   Current device: {}", d);
        }
        if let Some(i) = &iso_file {
            println!("   Current ISO: {}", i);
        }

        print!("\nYour choice: ");
        io::stdout().flush().unwrap();

        let mut choice_str = String::new();
        io::stdin()
            .read_line(&mut choice_str)
            .expect("Failed to read line");

        let choice: i32 = match choice_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if choice == 1 {
            device = Some(select_device());
            println!("Selected device: {}", device.as_ref().unwrap());
        } else if choice == 2 {
            print!("Enter path to ISO file: ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            iso_file = Some(line.trim().to_string());
            println!("Selected ISO file: {}", iso_file.as_ref().unwrap());
        } else if choice == 3 {
            if let (Some(d), Some(i)) = (&device, &iso_file) {
                const OPERATING_SYSTEM: &str = std::env::consts::OS;
                write_to_usb(OPERATING_SYSTEM, d, i);
            } else {
                println!("Please select a device and an ISO file first.");
            }
        } else if choice == 4 {
            break;
        } else {
            println!("Invalid choice, please try again.");
        }
    }
}

fn select_device() -> String {
    const OPERATING_SYSTEM: &str = std::env::consts::OS;
    println!("\nListing devices...");

    if OPERATING_SYSTEM == "windows" {
        let output = Command::new("wmic")
            .args(&["diskdrive", "get", "DeviceID,Model,Size"])
            .output()
            .expect("Failed to list devices. Is wmic available in your PATH?");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else if OPERATING_SYSTEM == "linux" {
        let output = Command::new("lsblk")
            .args(&["-d", "-n", "-o", "NAME,SIZE,MODEL"])
            .output()
            .expect("Failed to list devices. Is lsblk available in your PATH?");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else if OPERATING_SYSTEM == "macos" {
        let output = Command::new("diskutil")
            .args(&["list", "external"])
            .output()
            .expect("Failed to list devices. Is diskutil available in your PATH?");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("Unsupported OS for device listing.");
    }

    print!(
        "\nEnter the device identifier (e.g. /dev/sdb on Linux, \\\\.\\PHYSICALDRIVE1 on Windows):"
    );
    io::stdout().flush().unwrap();

    let mut selected_device = String::new();
    io::stdin()
        .read_line(&mut selected_device)
        .expect("Failed to read line");
    selected_device.trim().to_string()
}

fn write_to_usb(operating_system: &str, device: &str, iso_path: &str) {
    println!("\nAbout to write '{}' to '{}'.", iso_path, device);
    println!(
        "!!! WARNING: THIS IS A DESTRUCTIVE OPERATION. ALL DATA ON THE DEVICE WILL BE LOST !!!"
    );
    print!("Are you sure you want to continue? (yes/no): ");
    io::stdout().flush().unwrap();

    let mut confirmation = String::new();
    io::stdin()
        .read_line(&mut confirmation)
        .expect("Failed to read line");

    if confirmation.trim().to_lowercase() != "yes" {
        println!("Aborting.");
        return;
    }

    println!("\nStarting write process...");

    let status;
    if operating_system == "windows" {
        println!(
            "On Windows, this requires a 'dd' utility in your PATH and administrator privileges."
        );
        status = Command::new("dd")
            .arg(format!("if={}", iso_path))
            .arg(format!("of={}", device))
            .arg("bs=4M")
            .arg("--progress")
            .status();
    } else if operating_system == "linux" {
        println!("This will require sudo privileges.");
        status = Command::new("sudo")
            .arg("dd")
            .arg(format!("if={}", iso_path))
            .arg(format!("of={}", device))
            .arg("bs=4M")
            .arg("status=progress")
            .arg("oflag=sync")
            .status();
    } else if operating_system == "macos" {
        println!("This will require sudo privileges.");
        let raw_device = device.replace("/dev/disk", "/dev/rdisk");
        println!("Using raw device: {}", raw_device);
        status = Command::new("sudo")
            .arg("dd")
            .arg(format!("if={}", iso_path))
            .arg(format!("of={}", raw_device))
            .arg("bs=4m") // macOS dd uses 'm' for megabytes
            .status();
    } else {
        println!("Unsupported operating system for writing.");
        return;
    }

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("\nSuccessfully wrote to USB device.");
            } else {
                println!(
                    "\nFailed to write to USB device. Process exited with: {}",
                    exit_status
                );
            }
        }
        Err(e) => {
            println!("\nFailed to execute write command. Error: {}", e);
            if operating_system != "windows" {
                println!("Did you enter your password for sudo?");
            }
        }
    }
}
