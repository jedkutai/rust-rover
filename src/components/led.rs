// make Led on GPIO17 blink every .5 secconds
use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;

use crate::constants::pins;

pub struct Led {
    red_pin: OutputPin,
    green_pin: OutputPin,
    blue_pin: OutputPin,
}

impl Led {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let gpio = Gpio::new()?;

        let red_pin = gpio.get(pins::LED_RED)?.into_output();
        let green_pin = gpio.get(pins::LED_GREEN)?.into_output();
        let blue_pin = gpio.get(pins::LED_BLUE)?.into_output();
        Ok(Self {
            red_pin,
            green_pin,
            blue_pin,
        })
    }
}
