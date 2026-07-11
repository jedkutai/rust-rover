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
    /// Create LED
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

    /// Sets all the pins to low
    fn set_pins_low(&mut self) {
        self.red_pin.set_low();
        self.blue_pin.set_low();
        self.green_pin.set_low();
    }
    /// Turns on detection
    /// 
    /// The blue LED shows that detection mode is on
    pub fn detection_mode_on(&mut self) {
        self.set_pins_low();

        self.blue_pin.set_high();
    }

    /// Turns off detection
    /// 
    /// The green LED shows that detection mode is off
    pub fn detection_mode_off(&mut self) {
        self.set_pins_low();

        self.green_pin.set_high();
    }

    /// Indicates that an obstacle is detected
    /// 
    /// Note: Only works when detection mode is on
    /// The red LED shows that an obstacle is detected
    pub fn obstacle_detected(&mut self) {
        self.set_pins_low();

        self.red_pin.set_high();
    }
}

impl Drop for Led {
    fn drop(&mut self) {
        self.set_pins_low();
    }
}