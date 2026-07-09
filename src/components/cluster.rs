use rppal::gpio::Gpio;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use crate::components::sensor::Sensor;
use crate::constants::{pins, times};
use crate::enums::proximity::Proximity;

pub struct Cluster {
    front_center_sensor: Sensor,
    front_right_sensor: Sensor,
    front_left_sensor: Sensor,
    rear_sensor: Sensor,

    front_proximity: Proximity,
    rear_proximity: Proximity,
}

impl Cluster {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let gpio = Gpio::new()?;

        let front_trig = gpio.get(pins::FRONT_SENSOR_TRIG)?.into_output();
        let front_echo = gpio.get(pins::FRONT_SENSOR_ECHO)?.into_input();

        let left_trig = gpio.get(pins::FRONT_LEFT_SENSOR_TRIG)?.into_output();
        let left_echo = gpio.get(pins::FRONT_LEFT_SENSOR_ECHO)?.into_input();

        let right_trig = gpio.get(pins::FRONT_RIGHT_SENSOR_TRIG)?.into_output();
        let right_echo = gpio.get(pins::FRONT_RIGHT_SENSOR_ECHO)?.into_input();

        let rear_trig = gpio.get(pins::REAR_SENSOR_TRIG)?.into_output();
        let rear_echo = gpio.get(pins::REAR_SENSOR_ECHO)?.into_input();

        let front_center_sensor = Sensor::new(front_trig, front_echo);
        let front_left_sensor = Sensor::new(left_trig, left_echo);
        let front_right_sensor = Sensor::new(right_trig, right_echo);
        let rear_sensor = Sensor::new(rear_trig, rear_echo);

        Ok(Self {
            front_center_sensor,
            front_right_sensor,
            front_left_sensor,
            rear_sensor,

            front_proximity: Proximity::Far,
            // right_proximity: Proximity::Far,
            // left_proximity: Proximity::Far,
            rear_proximity: Proximity::Far,
        })
    }

    pub fn poll(&mut self) {
        let mut front_distance = 0.0;

        match self.front_center_sensor.read_distance_cm() {
            Ok(distance) => {
                front_distance = distance;
            }
            Err(error) => {
                eprintln!("Failed to read front distance: {}", error);
            }
        };
        sleep(Duration::from_millis(times::POLL_PAUSE));

        match self.front_right_sensor.read_distance_cm() {
            Ok(distance) => {
                front_distance = f64::min(distance, front_distance);
            }
            Err(error) => {
                eprintln!("Failed to read right distance: {}", error);
            }
        };

        sleep(Duration::from_millis(times::POLL_PAUSE));

        match self.front_left_sensor.read_distance_cm() {
            Ok(distance) => {
                front_distance = f64::min(distance, front_distance);
            }
            Err(error) => {
                eprintln!("Failed to read left distance: {}", error);
            }
        };
        match self.rear_sensor.read_distance_cm() {
            Ok(distance) => {
                self.rear_proximity = self.distance_to_proximity(distance);
            }
            Err(error) => {
                self.rear_proximity = Proximity::Far;
                eprintln!("Failed to read rear distance: {}", error);
            }
        };

        self.front_proximity = self.distance_to_proximity(front_distance);
    }

    fn distance_to_proximity(&mut self, distance: f64) -> Proximity {
        let near: f64 = 30.0;
        let medium: f64 = 45.0;

        if distance <= near {
            Proximity::Near
        } else if distance <= medium {
            Proximity::Medium
        } else {
            Proximity::Far
        }
    }

    pub fn get_front_proximity(&self) -> Proximity {
        self.front_proximity
    }
    
    pub fn get_rear_proximity(&self) -> Proximity {
        self.rear_proximity
    }
}
