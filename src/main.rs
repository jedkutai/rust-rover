mod components;
mod constants;
mod enums;
mod raw_mode;

use crate::components::rover::Rover;
use crate::raw_mode::RawModeGuard;

fn main() {
    println!("Starting rover...");

    let mut rover = match Rover::new() {
        Ok(rover) => rover,
        Err(error) => {
            eprintln!("Failed to create rover: {}", error);
            return;
        }
    };
    // rover.turn_on_detection();

    rover.print_controls();

    let _raw_mode = match RawModeGuard::new() {
        Ok(_raw_mode) => _raw_mode,
        Err(error) => {
            eprintln!("Failed to start raw mode: {}", error);
            return;
        }
    };

    match _raw_mode.drive(&mut rover) {
        Ok(()) => {}
        Err(error) => {
            eprintln!("Failed to drive rover: {}", error);
            return;
        }
    }
}

