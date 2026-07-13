use rppal::gpio::{InputPin, OutputPin};
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::constants::others::SPEED_OF_SOUND;
use crate::constants::times;

pub struct Sensor {
    trig_pin: OutputPin,
    echo_pin: InputPin,
}

impl Sensor {
    /// Create sensor
    pub fn new(trig_pin: OutputPin, echo_pin: InputPin) -> Self {
        Self { trig_pin, echo_pin }
    }

    /// Reads the distance between the sensor and obstacle
    pub fn read_distance_cm(&mut self) -> Result<f64, Box<dyn Error>> {
        // Make sure trigger starts low
        self.trig_pin.set_low();
        sleep(Duration::from_micros(times::SENSOR_SET_LOW_TIME)); 

        // Send pulse to start measurement
        self.trig_pin.set_high();
        sleep(Duration::from_micros(times::SENSOR_SET_HIGH_TIME));
        self.trig_pin.set_low();

        // Wait for ECHO to go HIGH
        let wait_start = Instant::now();
        while self.echo_pin.is_low() {
            if wait_start.elapsed() > Duration::from_millis(times::SENSOR_TIMEOUT) {
                return Err("Timed out waiting for ECHO to go HIGH".into());
            }
        }

        // Measure how long ECHO stays HIGH
        let echo_start = Instant::now();
        while self.echo_pin.is_high() {
            if echo_start.elapsed() > Duration::from_millis(times::SENSOR_TIMEOUT) {
                return Err("Timed out waiting for ECHO to go LOW".into());
            }
        }

        let echo_time = echo_start.elapsed();

        let distance_cm = echo_time.as_secs_f64() * SPEED_OF_SOUND / 2.0;

        Ok(distance_cm)
    }
}
