use rppal::gpio::{Gpio, InputPin, OutputPin};
use std::error::Error;

use crate::sensor::Sensor;

const FRONT_TRIG: u8 = 5;
const FRONT_ECHO: u8 = 6;

const LEFT_TRIG: u8 = 12;
const LEFT_ECHO: u8 = 16;

const RIGHT_TRIG: u8 = 20;
const RIGHT_ECHO: u8 = 21;

const REAR_TRIG: u8 = 25;
const REAR_ECHO: u8 = 26;

pub struct Cluster {
    front_sensor: Sensor,
    right_sensor: Sensor,
    left_sensor: Sensor,
    rear_sensor: Sensor,
}

impl Cluster {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let gpio = Gpio::new()?;

        let front_trig = gpio.get(FRONT_TRIG)?.into_output();
        let front_echo = gpio.get(FRONT_ECHO)?.into_input();

        let left_trig = gpio.get(LEFT_TRIG)?.into_output();
        let left_echo = gpio.get(LEFT_ECHO)?.into_input();

        let right_trig = gpio.get(RIGHT_TRIG)?.into_output();
        let right_echo = gpio.get(RIGHT_ECHO)?.into_input();

        let rear_trig = gpio.get(REAR_TRIG)?.into_output();
        let rear_echo = gpio.get(REAR_ECHO)?.into_input();

        let front_sensor = Sensor::new(front_trig, front_echo);
        let left_sensor = Sensor::new(left_trig, left_echo);
        let right_sensor = Sensor::new(right_trig, right_echo);
        let rear_sensor = Sensor::new(rear_trig, rear_echo);

        Ok(Self {
            front_sensor,
            right_sensor,
            left_sensor,
            rear_sensor,
        })
    }
}
