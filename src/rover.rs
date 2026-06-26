use rppal::gpio::{Gpio, OutputPin};

use crate::motor::MotorPair;

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
    left_motor_pair: MotorPair,
    right_motor_pair: MotorPair,
    direction: Direction,
}

impl Rover {
    /// Creates a new `Rover` from a left and right motor pair.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let gpio = Gpio::new()?;

        let mut stby = gpio.get(STBY)?.into_output();

        let pwma = gpio.get(PWMA)?.into_output();
        let ain1 = gpio.get(AIN1)?.into_output();
        let ain2 = gpio.get(AIN2)?.into_output();

        let bin1 = gpio.get(BIN1)?.into_output();
        let bin2 = gpio.get(BIN2)?.into_output();
        let pwmb = gpio.get(PWMB)?.into_output();

        let right_motor = MotorPair::new(ain1, ain2, pwma);
        let left_motor = MotorPair::new(bin1, bin2, pwmb);

        stby.set_high();
        Ok(Self {
            stby,
            left_motor_pair: left_motor,
            right_motor_pair: right_motor,
            direction: Direction::None,
        })
    }

    /// Moves the rover forward.
    pub fn forward(&mut self) {
        self.direction = Direction::Forward;
        self.left_motor_pair.forward();
        self.right_motor_pair.forward();
    }

    /// Moves the rover backward.
    pub fn backward(&mut self) {
        self.direction = Direction::Backward;
        self.left_motor_pair.backward();
        self.right_motor_pair.backward();
    }

    /// Turns rover right.
    /// 
    /// If the rover is moving forward/backwards when this is called:
    /// The rover will turn to the right
    /// 
    /// If the rover is still when this is called:
    /// It will spin in place (clockwise)
    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::Forward => {
                self.left_motor_pair.forward();
                self.right_motor_pair.stop();
            }
            Direction::Backward => {
                self.left_motor_pair.backward();
                self.right_motor_pair.stop();
            }
            Direction::None => {
                self.left_motor_pair.forward();
                self.right_motor_pair.backward();
            }
        }
    }

    /// Turns rover left.
    /// 
    /// If the rover is moving forward/backwards when this is called:
    /// The rover will turn to the left
    /// 
    /// If the rover is still when this is called:
    /// It will spin in place (counterclockwise)
    pub fn turn_left(&mut self) {
        match self.direction {
            Direction::Forward => {
                self.left_motor_pair.stop();
                self.right_motor_pair.forward();
            }
            Direction::Backward => {
                self.left_motor_pair.stop();
                self.right_motor_pair.backward();
            }
            Direction::None => {
                self.left_motor_pair.backward();
                self.right_motor_pair.forward();
            }
        }
    }

    /// Stops all motors.
    pub fn stop(&mut self) {
        self.direction = Direction::None;
        self.left_motor_pair.stop();
        self.right_motor_pair.stop();
    }
}

impl Drop for Rover {
    /// Stops the rover when it goes out of scope.
    fn drop(&mut self) {
        self.stop();
        self.stby.set_low();
    }
}

///Tests are being run on single thread to avoid issues with creating mutliple objects using the same pins
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
