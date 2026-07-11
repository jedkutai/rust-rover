use crate::components::cluster::Cluster;
use crate::components::led::Led;
use crate::components::motor::Motor;
use crate::constants::pins;
use crate::enums::direction::Direction;
use crate::enums::proximity::Proximity;

use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;

/// Controls the rover by coordinating the left and right motors.
///
/// Uses tank steering.
pub struct Rover {
    stby: OutputPin,
    left_motor: Motor,
    right_motor: Motor,
    cluster: Cluster,
    direction: Direction,
    detection_on: bool,
    led: Led,
    speed: f64,
}

impl Rover {
    /// Creates a new `Rover` from a left and right motor pair.
    /// The two left motors and two right motors are treated as one since this is going to drive like a tank
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let gpio = Gpio::new()?;

        let mut stby = gpio.get(pins::MOTOR_STBY)?.into_output();

        let pwma = gpio.get(pins::MOTOR_PWMA)?.into_output();
        let ain1 = gpio.get(pins::MOTOR_AIN1)?.into_output();
        let ain2 = gpio.get(pins::MOTOR_AIN2)?.into_output();

        let bin1 = gpio.get(pins::MOTOR_BIN1)?.into_output();
        let bin2 = gpio.get(pins::MOTOR_BIN2)?.into_output();
        let pwmb = gpio.get(pins::MOTOR_PWMB)?.into_output();

        let left_motor = Motor::new(bin1, bin2, pwmb);
        let right_motor = Motor::new(ain1, ain2, pwma);

        let cluster = match Cluster::new() {
            Ok(cluster) => cluster,
            Err(error) => {
                eprintln!("Failed to create cluster: {}", error);
                return Err(error);
            }
        };
        let led = match Led::new() {
            Ok(led) => led,
            Err(error) => {
                eprintln!("Failed to create LED: {}", error);
                return Err(error);
            }
        };

        stby.set_high();
        Ok(Self {
            stby,
            left_motor,
            right_motor,
            cluster,
            direction: Direction::None,
            detection_on: true,
            led,
            speed: 1.0,
        })
    }

    fn set_speed(&mut self, speed: f64) {
        let new_speed = speed.clamp(0.2, 1.0);
        match self.left_motor.set_speed(new_speed) {
            Ok(()) => match self.right_motor.set_speed(new_speed) {
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
        self.speed = new_speed;
    }

    /// Moves the rover forward.
    ///
    /// Doesn't move if obstacle is detected.
    pub fn forward(&mut self) {
        if self.cluster.get_front_proximity() == Proximity::Near {
            return;
        }

        self.reset_detection_led();
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
    ///
    /// Doesn't move if obstacle is detected.
    pub fn backward(&mut self) {
        if self.cluster.get_rear_proximity() == Proximity::Near {
            return;
        }

        self.reset_detection_led();
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
    /// It will spin in place (clockwise)
    pub fn turn_right(&mut self) {
        self.reset_detection_led();
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

    /// Turns rover left.
    ///
    /// It will spin in place (counterclockwise)
    pub fn turn_left(&mut self) {
        self.reset_detection_led();
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

    /// decrease the speed of the rover
    pub fn increase_speed(&mut self) {
        self.set_speed(self.speed + 0.1);
    }

    /// decrease the speed of the rover
    pub fn decrease_speed(&mut self) {
        self.set_speed(self.speed - 0.1);
    }

    /// Returns the speed of the rover
    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    /// Returns the direction of the rover
    pub fn get_direction(&mut self) -> Direction {
        self.direction
    }
    /// Stops all motors.
    pub fn stop(&mut self) {
        self.direction = Direction::None;
        self.left_motor.stop();
        self.right_motor.stop();
        self.set_speed(self.speed);
    }

    /// Returns true if detection is on
    pub fn detection_on(&self) -> bool {
        self.detection_on
    }

    /// Turns on detection
    pub fn turn_on_detection(&mut self) {
        self.detection_on = true;
        self.led.detection_mode_on();
    }

    /// Turns off detection
    pub fn turn_off_detection(&mut self) {
        self.detection_on = false;
        self.led.detection_mode_off();
    }

    /// Triggers LED Detected
    /// 
    /// Only triggers if Detection mode is on
    pub fn obstacle_detected(&mut self) {
        if !self.detection_on {
            return;
        }
        self.led.obstacle_detected();
    }

    /// Sets LED to correct state
    pub fn reset_detection_led(&mut self) {
        match self.detection_on {
            true => self.led.detection_mode_on(),
            false => self.led.detection_mode_off(),
        }
    }

    /// Calls cluster poll function
    /// 
    /// It goes through all the sensors to check proximity
    pub fn poll(&mut self) {
        self.cluster.poll();
    }

    /// Returns the front proximity
    pub fn get_front_proximity(&self) -> Proximity {
        self.cluster.get_front_proximity()
    }
    
    /// Returns the rear proximity
    pub fn get_rear_proximity(&self) -> Proximity {
        self.cluster.get_rear_proximity()
    }

    /// Print out the controls for the rover
    pub fn print_controls(&self) {
        println!("\nRover Controls:");
        println!("  W      -> forward");
        println!("  S      -> backward");
        println!("  A      -> turn left");
        println!("  D      -> turn right");
        println!("  M      -> increase speed");
        println!("  N      -> decrease speed");
        println!("  T      -> Turn on detection");
        println!("  Y      -> Turn off detection");
        println!("  Space  -> stop");
        println!("  X/Esc  -> exit");
        println!("  H      -> Reprint Controls");
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
