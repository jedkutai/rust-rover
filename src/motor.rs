use rppal::gpio::OutputPin;
use std::error::Error;
/// Controls one DC motor through a motor driver.
///
/// The motor uses two direction pins and one PWM pin.

const PWM_FREQUENCY: f64 = 1000.0;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
    None,
}

pub struct Motor {
    pin1: OutputPin,
    pin2: OutputPin,
    pin_pwm: OutputPin,
    direction: Direction,
    speed: f64,
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
            direction: Direction::None,
            speed: 1.0,
        }
    }

    /// set speed of pwm
    pub fn set_speed(&mut self, speed: f64) -> Result<(), Box<dyn Error>> {
        self.speed = speed.clamp(0.0, 1.0);
        if self.direction != Direction::None {
            self.pin_pwm.set_pwm_frequency(PWM_FREQUENCY, self.speed)?;
        }

        Ok(())
    }

    /// Drives the motor forward.
    pub fn forward(&mut self) -> Result<(), Box<dyn Error>> {
        self.pin_pwm.set_low();

        self.pin1.set_high();
        self.pin2.set_low();

        self.pin_pwm.set_pwm_frequency(PWM_FREQUENCY, self.speed)?;

        self.direction = Direction::Forward;
        Ok(())
    }

    /// Drives the motor backward.
    pub fn backward(&mut self) -> Result<(), Box<dyn Error>> {
        self.pin_pwm.set_low();

        self.pin1.set_low();
        self.pin2.set_high();

        self.pin_pwm.set_pwm_frequency(PWM_FREQUENCY, self.speed)?;
        self.direction = Direction::Backward;
        Ok(())
    }

    /// Stops the motor.
    pub fn stop(&mut self) {
        let _ = self.pin_pwm.clear_pwm();

        self.pin_pwm.set_low();
        self.pin1.set_low();
        self.pin2.set_low();
        self.direction = Direction::None;
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

    pub(crate) fn pin_pwm_is_on(&self) -> bool {
        !self.pin_pwm.is_set_low()
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

        match motor_a.forward() {
            Ok(()) => {
                assert!(motor_a.pin1_is_high());
                assert!(!motor_a.pin2_is_high());
                assert!(motor_a.pin_pwm_is_on());
            }
            Err(error) => {
                eprintln!("Motor A failed to move forward: {}", error);
                motor_a.stop();
                assert!(false);
            }
        };

        motor_a.stop();

        match motor_b.forward() {
            Ok(()) => {
                assert!(motor_b.pin1_is_high());
                assert!(!motor_b.pin2_is_high());
                assert!(motor_b.pin_pwm_is_on());
            }
            Err(error) => {
                eprintln!("Motor B failed to move forward: {}", error);
                motor_b.stop();
                assert!(false);
            }
        };

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

        match motor_a.backward() {
            Ok(()) => {
                assert!(!motor_a.pin1_is_high());
                assert!(motor_a.pin2_is_high());
                assert!(motor_a.pin_pwm_is_on());
            }
            Err(error) => {
                eprintln!("Motor A failed to move backward: {}", error);
                assert!(false);
                motor_a.stop();
                return;
            }
        };

        motor_a.stop();

        match motor_b.backward() {
            Ok(()) => {
                assert!(!motor_b.pin1_is_high());
                assert!(motor_b.pin2_is_high());
                assert!(motor_b.pin_pwm_is_on());
            }
            Err(error) => {
                eprintln!("Motor B failed to move backward: {}", error);
                assert!(false);
                motor_b.stop();
                return;
            }
        };
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
        assert!(!motor_a.pin_pwm_is_on());

        motor_b.stop();
        assert!(!motor_b.pin1_is_high());
        assert!(!motor_b.pin2_is_high());
        assert!(!motor_b.pin_pwm_is_on());
    }
}
