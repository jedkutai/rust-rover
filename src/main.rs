mod motor;
mod rover;
mod sensor;
mod cluster;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::error::Error;
use std::time::Duration;

use crate::rover::Rover;
struct RawModeGuard;



fn main() {
    println!("Starting motor test...");

    let mut rover = match Rover::new() {
        Ok(rover) => rover,
        Err(error) => {
            eprintln!("Failed to create rover: {}", error);
            return;
        }
    };

    rover.print_controls();

    let _raw_mode = match RawModeGuard::new() {
        Ok(_raw_mode) => _raw_mode,
        Err(error) => {
            eprintln!("Failed to start raw mode: {}", error);
            return;
        }
    };

    

    match _raw_mode.drive(&mut rover) {
        Ok(()) => {},
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

    fn drive(&self, rover: &mut Rover) -> Result<(), Box<dyn Error>> {
        loop {
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

                    KeyCode::Char('x') | KeyCode::Char('X') | KeyCode::Esc => {
                        rover.stop();
                        println!("\rExiting...                  ");
                        break;
                    }

                    _ => {}
                }
            }

            // break;
        }

        Ok(())
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}