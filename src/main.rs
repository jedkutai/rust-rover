// use rppal::gpio::Gpio;
// use std::error::Error;

// mod led;
// use led::Led;

// const LED_PIN: u8 = 17;

// fn main() -> Result<(), Box<dyn Error>> {
//     println!("Starting LED blink test...");

//     let gpio = Gpio::new()?;
//     let blue_pin = gpio.get(LED_PIN)?.into_output();
//     let mut blue_led = Led::new(blue_pin, 500);
//     loop {
//         blue_led.blink();
//     }
// }
mod motor;
mod rover;

// use rppal::gpio::Gpio;
// use rppal::pwm;
// use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

// use crate::motor::Motor;
use crate::rover::Rover;

// const PWMA: u8 = 18;
// const AIN2: u8 = 27;
// const AIN1: u8 = 17;

// const STBY: u8 = 22;

// const BIN1: u8 = 23;
// const BIN2: u8 = 24;
// const PWMB: u8 = 13;

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
    sleep(Duration::from_secs(1));

    println!("Turn right for 1 second...");
    rover.turn_right();
    sleep(Duration::from_secs(1));

    println!("Turn left for 1 second...");
    rover.turn_left();
    sleep(Duration::from_secs(1));

    rover.stop();

    println!("Move backward for 1 second...");
    rover.backward();
    sleep(Duration::from_secs(1));

    println!("Turn right for 1 second...");
    rover.turn_right();
    sleep(Duration::from_secs(1));

    println!("Turn left for 1 second...");
    rover.turn_left();
    sleep(Duration::from_secs(1));

    rover.stop();

    //spin
    println!("Turn right for 1 second...");
    rover.turn_right();
    sleep(Duration::from_secs(1));

    println!("Turn left for 1 second...");
    rover.turn_left();
    sleep(Duration::from_secs(1));

    rover.stop();

    println!("Putting driver back in standby...");
    drop(rover);

    println!("Motor test complete.");

    
}
