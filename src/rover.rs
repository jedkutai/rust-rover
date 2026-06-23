use crate::motor::Motor;

/// Controls the rover by coordinating the left and right motors.
///
/// Uses tank steering.
pub struct Rover {
    left_motor: Motor,
    right_motor: Motor,
}

impl Rover {
    /// Creates a new `Rover` from a left and right motor.
    pub fn new(left_motor: Motor, right_motor: Motor) -> Self {
        Self {
            left_motor,
            right_motor,
        }
    }

    /// Moves the rover forward.
    pub fn forward(&mut self) {
        self.left_motor.forward();
        self.right_motor.forward();
    }

    /// Moves the rover backward.
    pub fn backward(&mut self) {
        self.left_motor.backward();
        self.right_motor.backward();
    }

    /// Turns right by moving only the left motor.
    pub fn turn_right(&mut self) {
        self.left_motor.forward();
        self.right_motor.stop();
    }

    /// Turns left by moving only the right motor.
    pub fn turn_left(&mut self) {
        self.left_motor.stop();
        self.right_motor.forward();
    }

    /// Spins the rover clockwise in place.
    pub fn spin_clockwise(&mut self) {
        self.left_motor.forward();
        self.right_motor.backward();
    }

    /// Spins the rover counterclockwise in place.
    pub fn spin_counterclockwise(&mut self) {
        self.left_motor.backward();
        self.right_motor.forward();
    }

    /// Stops both motors.
    pub fn stop(&mut self) {
        self.left_motor.stop();
        self.right_motor.stop();
    }
}

impl Drop for Rover {
    /// Stops the rover when it goes out of scope.
    fn drop(&mut self) {
        self.stop();
    }
}
