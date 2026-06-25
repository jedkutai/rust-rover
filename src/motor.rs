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

#[cfg(test)]
impl Motor {
    pub(crate) fn pin1_is_high(&self) -> bool {
        self.pin1.is_set_high()
    }

    pub(crate) fn pin2_is_high(&self) -> bool {
        self.pin2.is_set_high()
    }

    pub(crate) fn pin_pwm_is_high(&self) -> bool {
        self.pin_pwm.is_set_high()
    }

    // const STBY: u8 = 22;

    pub(crate) fn get_test_motor_a() -> Result<Motor, Box<dyn std::error::Error>> {
        use rppal::gpio::Gpio;
        const PWMA: u8 = 18;
        const AIN2: u8 = 27;
        const AIN1: u8 = 17;

        let gpio = Gpio::new()?;
        let pwma = gpio.get(PWMA)?.into_output();
        let ain1 = gpio.get(AIN1)?.into_output();
        let ain2 = gpio.get(AIN2)?.into_output();

        let motor = Motor::new(ain1, ain2, pwma);
        Ok(motor)
    }

    pub(crate) fn get_test_motor_b() -> Result<Motor, Box<dyn std::error::Error>> {
        use rppal::gpio::Gpio;

        const BIN1: u8 = 23;
        const BIN2: u8 = 24;
        const PWMB: u8 = 13;

        let gpio = Gpio::new()?;
        let bin1 = gpio.get(BIN1)?.into_output();
        let bin2 = gpio.get(BIN2)?.into_output();
        let pwmb = gpio.get(PWMB)?.into_output();

        let motor = Motor::new(bin1, bin2, pwmb);

        Ok(motor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore = "requires Raspberry Pi GPIO"]
    fn should_move_forward() {
        let mut motor_a = match Motor::get_test_motor_a() {
            Ok(motor) => motor,
            Err(error) => {
                eprintln!("Failed to create motor: {}", error);
                assert!(false);
                return;
            }
        };

        let mut motor_b = match Motor::get_test_motor_b() {
            Ok(motor) => motor,
            Err(error) => {
                eprintln!("Failed to create motor: {}", error);
                assert!(false);
                return;
            }
        };

        motor_a.forward();
        assert!(motor_a.pin1_is_high());
        assert!(!motor_a.pin2_is_high());
        assert!(motor_a.pin_pwm_is_high());
        motor_a.stop();

        motor_b.forward();
        assert!(motor_b.pin1_is_high());
        assert!(!motor_b.pin2_is_high());
        assert!(motor_b.pin_pwm_is_high());
        motor_b.stop();
    }

    #[test]
    #[ignore = "requires Raspberry Pi GPIO"]
    fn should_move_backward() {
        let mut motor_a = match Motor::get_test_motor_a() {
            Ok(motor) => motor,
            Err(error) => {
                eprintln!("Failed to create motor: {}", error);
                assert!(false);
                return;
            }
        };

        let mut motor_b = match Motor::get_test_motor_b() {
            Ok(motor) => motor,
            Err(error) => {
                eprintln!("Failed to create motor: {}", error);
                assert!(false);
                return;
            }
        };

        motor_a.backward();
        assert!(!motor_a.pin1_is_high());
        assert!(motor_a.pin2_is_high());
        assert!(motor_a.pin_pwm_is_high());
        motor_a.stop();

        motor_b.backward();
        assert!(!motor_b.pin1_is_high());
        assert!(motor_b.pin2_is_high());
        assert!(motor_b.pin_pwm_is_high());
        motor_b.stop();
    }

    #[test]
    #[ignore = "requires Raspberry Pi GPIO"]
    fn should_stop() {
        let mut motor_a = match Motor::get_test_motor_a() {
            Ok(motor) => motor,
            Err(error) => {
                eprintln!("Failed to create motor: {}", error);
                assert!(false);
                return;
            }
        };

        let mut motor_b = match Motor::get_test_motor_b() {
            Ok(motor) => motor,
            Err(error) => {
                eprintln!("Failed to create motor: {}", error);
                assert!(false);
                return;
            }
        };

        motor_a.stop();
        assert!(!motor_a.pin1_is_high());
        assert!(!motor_a.pin2_is_high());
        assert!(!motor_a.pin_pwm_is_high());

        motor_b.stop();
        assert!(!motor_b.pin1_is_high());
        assert!(!motor_b.pin2_is_high());
        assert!(!motor_b.pin_pwm_is_high());
    }
}
