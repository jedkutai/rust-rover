mod components;
mod constants;
mod enums;

use crate::enums::direction::Direction;
use crate::enums::proximity::Proximity;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::error::Error;
use std::time::Duration;

use crate::components::rover::Rover;
struct RawModeGuard;

fn main() {
    println!("Starting rover...");

    let mut rover = match Rover::new() {
        Ok(rover) => rover,
        Err(error) => {
            eprintln!("Failed to create rover: {}", error);
            return;
        }
    };
    rover.turn_on_detection();

    rover.print_controls();

    let _raw_mode = match RawModeGuard::new() {
        Ok(_raw_mode) => _raw_mode,
        Err(error) => {
            eprintln!("Failed to start raw mode: {}", error);
            return;
        }
    };

    match _raw_mode.drive(&mut rover) {
        Ok(()) => {}
        Err(error) => {
            eprintln!("Failed to drive rover: {}", error);
            return;
        }
    }
}

impl RawModeGuard {
    fn new() -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;
        Ok(Self)
    }

    fn detect(&self, rover: &mut Rover) {
        if !rover.detection_on() {
            return;
        }

        rover.poll();

        match rover.get_direction() {
            Direction::Forward => {
                if rover.get_front_proximity() == Proximity::Near {
                    rover.obstacle_detected();
                    rover.stop();
                }
            }
            Direction::Backward => {
                if rover.get_rear_proximity() == Proximity::Near {
                    rover.obstacle_detected();
                    rover.stop();
                }
            }
            Direction::None => {}
        }
    }

    fn drive(&self, rover: &mut Rover) -> Result<(), Box<dyn Error>> {
        loop {
            self.detect(rover);

            if event::poll(Duration::from_millis(100))? {
                let event = event::read()?;

                let Event::Key(key_event) = event else {
                    continue;
                };

                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                match key_event.code {
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        rover.forward();
                        println!("\rForward | Speed: {:.0}%     ", rover.get_speed() * 100.0);
                    }

                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        rover.backward();
                        println!("\rBackward | Speed: {:.0}%    ", rover.get_speed() * 100.0);
                    }

                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        rover.turn_left();
                        println!("\rTurn left | Speed: {:.0}%   ", rover.get_speed() * 100.0);
                    }

                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        rover.turn_right();
                        println!("\rTurn right | Speed: {:.0}%  ", rover.get_speed() * 100.0);
                    }

                    KeyCode::Char('m') | KeyCode::Char('M') => {
                        rover.increase_speed();
                        println!("\rSpeed increased: {:.0}%     ", rover.get_speed() * 100.0);
                    }

                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        rover.decrease_speed();
                        println!("\rSpeed decreased: {:.0}%     ", rover.get_speed() * 100.0);
                    }

                    KeyCode::Char('h') | KeyCode::Char('H') => {
                        rover.print_controls();
                        println!("\rSpeed decreased: {:.0}%     ", rover.get_speed() * 100.0);
                    }

                    KeyCode::Char(' ') => {
                        rover.stop();
                        println!("\rStopped                     ");
                    }

                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        rover.turn_on_detection();
                        println!("\rDetection: On     ");
                    }

                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        rover.turn_off_detection();
                        println!("\rDetection: Off    ");
                    }

                    KeyCode::Char('x') | KeyCode::Char('X') | KeyCode::Esc => {
                        rover.stop();
                        println!("\rExiting...                  ");
                        break;
                    }

                    _ => {}
                }
            }
        }

        Ok(())
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}
