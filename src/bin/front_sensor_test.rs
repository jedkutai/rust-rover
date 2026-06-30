use rppal::gpio::Gpio;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

const FRONT_TRIG: u8 = 5; // physical pin 29
const FRONT_ECHO: u8 = 6; // physical pin 31

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting HC-SR04 front sensor test...");
    println!("TRIG = GPIO5 / physical pin 29");
    println!("ECHO = GPIO6 / physical pin 31 through voltage divider");

    let gpio = Gpio::new()?;

    let mut trig = gpio.get(FRONT_TRIG)?.into_output();
    let echo = gpio.get(FRONT_ECHO)?.into_input();

    loop {
        match read_distance_cm(&mut trig, &echo) {
            Ok(distance) => {
                println!("Distance: {:.2} cm", distance);
            }
            Err(error) => {
                println!("Sensor error: {error}");
            }
        }

        sleep(Duration::from_millis(500));
    }
}

fn read_distance_cm(
    trig: &mut rppal::gpio::OutputPin,
    echo: &rppal::gpio::InputPin,
) -> Result<f64, Box<dyn Error>> {
    // Make sure trigger starts low
    trig.set_low();
    sleep(Duration::from_micros(2));

    // Send 10 microsecond pulse to start measurement
    trig.set_high();
    sleep(Duration::from_micros(10));
    trig.set_low();

    // Wait for ECHO to go HIGH
    let wait_start = Instant::now();
    while echo.is_low() {
        if wait_start.elapsed() > Duration::from_millis(50) {
            return Err("Timed out waiting for ECHO to go HIGH".into());
        }
    }

    // Measure how long ECHO stays HIGH
    let echo_start = Instant::now();
    while echo.is_high() {
        if echo_start.elapsed() > Duration::from_millis(50) {
            return Err("Timed out waiting for ECHO to go LOW".into());
        }
    }

    let echo_time = echo_start.elapsed();

    // Speed of sound ≈ 34300 cm/s.
    // Divide by 2 because sound travels to object and back.
    let distance_cm = echo_time.as_secs_f64() * 34300.0 / 2.0;

    Ok(distance_cm)
}