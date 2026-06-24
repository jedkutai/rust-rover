
mod motor;
mod rover;

use std::thread::sleep;
use std::time::Duration;
use crate::rover::Rover;

const TEST_LENGTH: u64 =3;
fn main()  {
    println!("Starting motor test...");

    let mut rover = match Rover::new() {
        Ok(rover) => rover,
        Err(error) => {
            eprintln!("Failed to create rover: {}", error);
            return;
        }
    };
    

    println!("Move forward for 1 second...");
    rover.forward();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Turn right for 1 second...");
    rover.turn_right();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Turn left for 1 second...");
    rover.turn_left();
    sleep(Duration::from_secs(TEST_LENGTH));

    rover.stop();

    println!("Move backward for 1 second...");
    rover.backward();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Turn right for 1 second...");
    rover.turn_right();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Turn left for 1 second...");
    rover.turn_left();
    sleep(Duration::from_secs(TEST_LENGTH));

    rover.stop();

    //spin
    println!("Turn right for 1 second...");
    rover.turn_right();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Turn left for 1 second...");
    rover.turn_left();
    sleep(Duration::from_secs(TEST_LENGTH));

    rover.stop();

    println!("Putting driver back in standby...");
    drop(rover);

    println!("Motor test complete.");

    
}
