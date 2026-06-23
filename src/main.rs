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

use rppal::gpio::Gpio;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

const PWMA: u8 = 18;
const AIN2: u8 = 27;
const AIN1: u8 = 17;

const STBY: u8 = 22;

const BIN1: u8 = 23;
const BIN2: u8 = 24;
const PWMB: u8 = 13;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting motor test...");

    let gpio = Gpio::new()?;

    let mut pwma = gpio.get(PWMA)?.into_output();
    let mut ain1 = gpio.get(AIN1)?.into_output();
    let mut ain2 = gpio.get(AIN2)?.into_output();

    let mut stby = gpio.get(STBY)?.into_output();

    let mut bin1 = gpio.get(BIN1)?.into_output();
    let mut bin2 = gpio.get(BIN2)?.into_output();
    let mut pwmb = gpio.get(PWMB)?.into_output();

    println!("Waking motor driver...");
    stby.set_high();

    println!("Motor A forward for 1 second...");
    ain1.set_high();
    ain2.set_low();
    pwma.set_high();

    sleep(Duration::from_secs(1));

    println!("Stopping Motor A...");
    pwma.set_low();
    ain1.set_low();
    ain2.set_low();

    sleep(Duration::from_secs(1));

    println!("Motor B forward for 1 second...");
    bin1.set_high();
    bin2.set_low();
    pwmb.set_high();

    sleep(Duration::from_secs(1));

    println!("Stopping Motor B...");
    pwmb.set_low();
    bin1.set_low();
    bin2.set_low();

    println!("Putting driver back in standby...");
    stby.set_low();

    println!("Motor test complete.");

    Ok(())
}