
mod motor;
mod rover;

use std::thread::sleep;
use std::time::Duration;
use crate::rover::Rover;

const TEST_LENGTH: u64 = 3;
fn main()  {
    println!("Starting motor test...");

    let mut rover = match Rover::new() {
        Ok(rover) => rover,
        Err(error) => {
            eprintln!("Failed to create rover: {}", error);
            return;
        }
    };
    
    rover.set_speed(0.5);

    println!("Move forward for {TEST_LENGTH} second...");
    rover.forward();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Move right for {TEST_LENGTH} second...");
    rover.turn_right();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Move left for {TEST_LENGTH} second...");
    rover.turn_left();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Move backward for {TEST_LENGTH} second...");
    rover.backward();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Move right for {TEST_LENGTH} second...");
    rover.turn_right();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Move left for {TEST_LENGTH} second...");
    rover.turn_left();
    sleep(Duration::from_secs(TEST_LENGTH));

    rover.stop();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Spin right for {TEST_LENGTH} second...");
    rover.turn_right();
    sleep(Duration::from_secs(TEST_LENGTH));

    println!("Spin left for {TEST_LENGTH} second...");
    rover.turn_left();
    sleep(Duration::from_secs(TEST_LENGTH));
    // println!("Turn right for {TEST_LENGTH} second...");
    // rover.turn_right();
    // sleep(Duration::from_secs(TEST_LENGTH));

    // println!("Turn left for {TEST_LENGTH} second...");
    // rover.turn_left();
    // sleep(Duration::from_secs(TEST_LENGTH));

    // rover.stop();

    // println!("Move backward for {TEST_LENGTH} second...");
    // rover.backward();
    // sleep(Duration::from_secs(TEST_LENGTH));

    // println!("Turn right for {TEST_LENGTH} second...");
    // rover.turn_right();
    // sleep(Duration::from_secs(TEST_LENGTH));

    // println!("Turn left for {TEST_LENGTH} second...");
    // rover.turn_left();
    // sleep(Duration::from_secs(TEST_LENGTH));

    // rover.stop();

    // //spin
    // println!("Turn right for {TEST_LENGTH} second...");
    // rover.turn_right();
    // sleep(Duration::from_secs(TEST_LENGTH));

    // println!("Turn left for {TEST_LENGTH} second...");
    // rover.turn_left();
    // sleep(Duration::from_secs(TEST_LENGTH));


    println!("Putting driver back in standby...");
    rover.stop();


    println!("Motor test complete.");

    
}
