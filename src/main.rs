use rppal::gpio::Gpio;
use std::error::Error;

mod led;
use led::Led;

const LED_PIN: u8 = 17;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting LED blink test...");

    let gpio = Gpio::new()?;
    let blue_pin = gpio.get(LED_PIN)?.into_output();
    let mut blue_led = Led::new(blue_pin, 500);
    loop {
        blue_led.blink();
    }
}