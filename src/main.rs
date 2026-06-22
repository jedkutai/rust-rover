// fn main() {
//     println!("Hello, world!");
// }
use rppal::gpio::Gpio;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

const LED_PIN: u8 = 17;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting LED blink test...");

    let gpio = Gpio::new()?;
    let mut led = gpio.get(LED_PIN)?.into_output();

    loop {
        led.set_high();
        println!("LED on");
        sleep(Duration::from_millis(500));

        led.set_low();
        println!("LED off");
        sleep(Duration::from_millis(500));
    }
}