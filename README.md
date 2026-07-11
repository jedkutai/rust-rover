# Rust Rover

A Raspberry Pi 5 rover written in Rust featuring keyboard controlled movement,
PWM speed control, four ultrasonic distance sensors, obstacle detection, and an
RGB status indicator.

## Demo

[![Watch the Rust Rover demo](https://img.youtube.com/vi/8th5zglHIjU/maxresdefault.jpg)](https://www.youtube.com/watch?v=8th5zglHIjU)

▶️ [Watch the full demonstration on YouTube](https://www.youtube.com/watch?v=8th5zglHIjU)

## Features

* Keyboard controlled tank drive
* Adjustable motor speed using PWM
* Four HC-SR04 ultrasonic distance sensors
* Obstacle detection and avoidance
* RGB LED mode indicator
* Headless Raspberry Pi operation
* Written in Rust using the `rppal` crate

## Hardware

* Raspberry Pi 5
* TB6612FNG motor driver
* Four DC motors
* Four HC-SR04 ultrasonic distance sensors
* RGB LED
* Battery pack
* USB power bank
* Breadboards and resistors

## Software

* Rust
* `rppal`
* Raspberry Pi OS
* SSH and headless development

## How It Works

The project is organized around a central `Rover` type that controls the motors,
RGB LED, and ultrasonic sensor cluster.

```text
Rover
├── Motors
├── LED
└── Sensor Cluster
    └── Sensors
```

## Controls

The rover is controlled using the keyboard:

| Key          | Action                     |
| ------------ | -------------------------- |
| `W`          | Drive forward              |
| `S`          | Drive backward             |
| `A`          | Turn left                  |
| `D`          | Turn right                 |
| `M`          | Increase motor speed       |
| `N`          | Decrease motor speed       |
| `T`          | Enable obstacle detection  |
| `Y`          | Disable obstacle detection |
| `Space`      | Stop the rover             |
| `X` or `Esc` | Exit the program           |
| `H`          | Reprint the control list   |

## Wiring

### Motors

[View the motor wiring guide](docs/Stage_1/motor_wiring.txt)

### Sensors

[View the sensor wiring guide](docs/Stage_3/sensor_wiring.txt)

### RGB LED

[View the RGB LED wiring guide](docs/Stage_4/led_wiring.txt)

## Installation

This project is designed to run on a Raspberry Pi 5 using Raspberry Pi OS.

### Prerequisites

Before installing the project, make sure the Raspberry Pi has:

* Rust and Cargo installed
* Git installed
* The rover hardware connected to the correct GPIO pins

Clone the repository:

```bash
git clone https://github.com/jedkutai/rust-rover.git
cd rust-rover
```

Build the project in release mode:

```bash
cargo build --release
```

## Running the Project

Run the rover program with:

```bash
cargo run --release
```

The default binary is configured in `Cargo.toml`, so Cargo automatically runs the main rover application.

Once the program starts, use the keyboard controls listed below to drive the rover.

> **Important:** Run the program directly on the Raspberry Pi while the motor driver, sensors, RGB LED, and external motor power supply are connected correctly. The Raspberry Pi, motor driver, and motor battery must share a common ground.

