use rppal::gpio::{Gpio, OutputPin};

use crate::motor::{Direction, Motor};

const PWMA: u8 = 18;
const AIN2: u8 = 27;
const AIN1: u8 = 17;

const STBY: u8 = 22;

const BIN1: u8 = 23;
const BIN2: u8 = 24;
const PWMB: u8 = 13;

/// Controls the rover by coordinating the left and right motors.
///
/// Uses tank steering.
pub struct Rover {
    stby: OutputPin,
    left_motor: Motor,
    right_motor: Motor,
    direction: Direction,
    speed: f64,
}

impl Rover {
    /// Creates a new `Rover` from a left and right motor pair.
    /// The two left motors and two right motors are treated as one since this is going to drive like a tank
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let gpio = Gpio::new()?;

        let mut stby = gpio.get(STBY)?.into_output();

        let pwma = gpio.get(PWMA)?.into_output();
        let ain1 = gpio.get(AIN1)?.into_output();
        let ain2 = gpio.get(AIN2)?.into_output();

        let bin1 = gpio.get(BIN1)?.into_output();
        let bin2 = gpio.get(BIN2)?.into_output();
        let pwmb = gpio.get(PWMB)?.into_output();

        let left_motor = Motor::new(bin1, bin2, pwmb);
        let right_motor = Motor::new(ain1, ain2, pwma);

        stby.set_high();
        Ok(Self {
            stby,
            left_motor,
            right_motor,
            direction: Direction::None,
            speed: 1.0,
        })
    }

    pub fn set_speed(&mut self, speed: f64) {
        match self.left_motor.set_speed(speed) {
            Ok(()) => match self.right_motor.set_speed(speed) {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("Failed to update right motor speed: {}", error);
                    self.stop();
                    return;
                }
            },
            Err(error) => {
                eprintln!("Failed to update left motor speed: {}", error);
                self.stop();
                return;
            }
        }
        self.speed = speed;
    }

    fn update_speed_left(&mut self, speed: f64) {
        match self.left_motor.set_speed(speed) {
            Ok(()) => {}
            Err(error) => {
                eprintln!("Failed to update left motor speed: {}", error);
                self.stop();
                return;
            }
        }
    }

    fn update_speed_right(&mut self, speed: f64) {
        match self.right_motor.set_speed(speed) {
            Ok(()) => {}
            Err(error) => {
                eprintln!("Failed to update right motor speed: {}", error);
                self.stop();
                return;
            }
        }
    }
    /// Moves the rover forward.
    pub fn forward(&mut self) {
        self.direction = Direction::Forward;
        self.set_speed(self.speed);

        match self.left_motor.forward() {
            Ok(()) => {}
            Err(error) => {
                eprintln!("Failed to move left motor forward: {}", error);
                self.stop();
                return;
            }
        };

        match self.right_motor.forward() {
            Ok(()) => {}
            Err(error) => {
                eprintln!("Failed to move right motor forward: {}", error);
                self.stop();
                return;
            }
        };
    }

    /// Moves the rover backward.
    pub fn backward(&mut self) {
        self.direction = Direction::Backward;
        self.set_speed(self.speed);
        match self.left_motor.backward() {
            Ok(()) => {}
            Err(error) => {
                eprintln!("Failed to move left motor backward: {}", error);
                self.stop();
                return;
            }
        };

        match self.right_motor.backward() {
            Ok(()) => {}
            Err(error) => {
                eprintln!("Failed to move right motor backward: {}", error);
                self.stop();
                return;
            }
        };
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
                self.update_speed_right( self.speed * 0.5);
                self.update_speed_left( self.speed);
            }
            Direction::Backward => {
                self.update_speed_right( self.speed * 0.5);
                self.update_speed_left( self.speed);
            }
            Direction::None => {
                self.set_speed(self.speed);
                match self.left_motor.forward() {
                    Ok(()) => {}
                    Err(error) => {
                        eprintln!("Failed to move left motor forward: {}", error);
                        self.stop();
                        return;
                    }
                };
                match self.right_motor.backward() {
                    Ok(()) => {}
                    Err(error) => {
                        eprintln!("Failed to move right motor backward: {}", error);
                        self.stop();
                        return;
                    }
                };
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
                self.update_speed_left(self.speed * 0.5);
                self.update_speed_right(self.speed);
            }
            Direction::Backward => {
                self.update_speed_left(self.speed * 0.5);
                self.update_speed_right(self.speed);
            }
            Direction::None => {
                self.set_speed(self.speed);
                match self.left_motor.backward() {
                    Ok(()) => {}
                    Err(error) => {
                        eprintln!("Failed to move left motor backward: {}", error);
                        self.stop();
                        return;
                    }
                };

                match self.right_motor.forward() {
                    Ok(()) => {}
                    Err(error) => {
                        eprintln!("Failed to move right motor forward: {}", error);
                        self.stop();
                        return;
                    }
                };
            }
        }
    }

    /// Stops all motors.
    pub fn stop(&mut self) {
        self.direction = Direction::None;
        self.left_motor.stop();
        self.right_motor.stop();
        self.set_speed(self.speed);
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
