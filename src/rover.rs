use rppal::gpio::{Gpio, OutputPin};

use crate::motor::Motor;

const PWMA: u8 = 18;
const AIN2: u8 = 27;
const AIN1: u8 = 17;

const STBY: u8 = 22;

const BIN1: u8 = 23;
const BIN2: u8 = 24;
const PWMB: u8 = 13;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Forward,
    Backward,
    None,
}
/// Controls the rover by coordinating the left and right motors.
///
/// Uses tank steering.
pub struct Rover {
    stby: OutputPin,
    left_motor: Motor,
    right_motor: Motor,
    direction: Direction,
}

impl Rover {
    /// Creates a new `Rover` from a left and right motor.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let gpio = Gpio::new()?;

        let mut stby = gpio.get(STBY)?.into_output();

        let pwma = gpio.get(PWMA)?.into_output();
        let ain1 = gpio.get(AIN1)?.into_output();
        let ain2 = gpio.get(AIN2)?.into_output();

        let bin1 = gpio.get(BIN1)?.into_output();
        let bin2 = gpio.get(BIN2)?.into_output();
        let pwmb = gpio.get(PWMB)?.into_output();

        let right_motor = Motor::new(ain1, ain2, pwma);
        let left_motor = Motor::new(bin1, bin2, pwmb);

        stby.set_high();
        Ok(Self {
            stby,
            left_motor,
            right_motor,
            direction: Direction::None,
        })
    }

    /// Moves the rover forward.
    pub fn forward(&mut self) {
        self.direction = Direction::Forward;
        self.left_motor.forward();
        self.right_motor.forward();
    }

    /// Moves the rover backward.
    pub fn backward(&mut self) {
        self.direction = Direction::Backward;
        self.left_motor.backward();
        self.right_motor.backward();
    }

    /// Turns right by moving only the left motor.
    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::Forward => {
                self.left_motor.forward();
                self.right_motor.stop();
            }
            Direction::Backward => {
                self.left_motor.backward();
                self.right_motor.stop();
            }
            Direction::None => {
                self.left_motor.forward();
                self.right_motor.backward();
            }
        }
    }

    /// Turns left by moving only the right motor.
    pub fn turn_left(&mut self) {
        match self.direction {
            Direction::Forward => {
                self.left_motor.stop();
                self.right_motor.forward();
            }
            Direction::Backward => {
                self.left_motor.stop();
                self.right_motor.backward();
            }
            Direction::None => {
                self.left_motor.backward();
                self.right_motor.forward();
            }
        }
    }

    /// Stops both motors.
    pub fn stop(&mut self) {
        self.direction = Direction::None;
        self.left_motor.stop();
        self.right_motor.stop();
    }
}

impl Drop for Rover {
    /// Stops the rover when it goes out of scope.
    fn drop(&mut self) {
        self.stop();
        self.stby.set_low();
    }
}
//use this to run tests. will fail otherwise
//cargo test -- --test-threads=1

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore = "requires Raspberry Pi GPIO"]
    fn should_move_forward() {
        let mut test_rover = match Rover::new() {
            Ok(test_rover) => test_rover,
            Err(error) => {
                eprintln!("Failed to create rover: {}", error);
                assert!(false);
                return;
            }
        };
        test_rover.forward();
        assert_eq!(test_rover.direction, Direction::Forward);
        drop(test_rover);
    }

    #[test]
    #[ignore = "requires Raspberry Pi GPIO"]
    fn should_move_backward() {
        let mut test_rover = match Rover::new() {
            Ok(test_rover) => test_rover,
            Err(error) => {
                eprintln!("Failed to create rover: {}", error);
                assert!(false);
                return;
            }
        };
        test_rover.backward();
        assert_eq!(test_rover.direction, Direction::Backward);
        drop(test_rover);
    }
}
