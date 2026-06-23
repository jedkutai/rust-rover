use rppal::gpio::OutputPin;

/// Controls one DC motor through a motor driver.
///
/// The motor uses two direction pins and one PWM pin.
pub struct Motor {
    pin1: OutputPin,
    pin2: OutputPin,
    pin_pwm: OutputPin,
}

impl Motor {
    /// Creates a new `Motor`.
    ///
    /// `pin1` and `pin2` control direction.
    /// `pin_pwm` enables or disables the motor.
    pub fn new(pin1: OutputPin, pin2: OutputPin, pin_pwm: OutputPin) -> Self {
        Self {
            pin1,
            pin2,
            pin_pwm,
        }
    }

    /// Drives the motor forward.
    pub fn forward(&mut self) {
        self.pin_pwm.set_low();

        self.pin1.set_high();
        self.pin2.set_low();

        self.pin_pwm.set_high();
    }

    /// Drives the motor backward.
    pub fn backward(&mut self) {
        self.pin_pwm.set_low();

        self.pin1.set_low();
        self.pin2.set_high();

        self.pin_pwm.set_high();
    }

    /// Stops the motor.
    pub fn stop(&mut self) {
        self.pin_pwm.set_low();
        self.pin1.set_low();
        self.pin2.set_low();
    }
}

