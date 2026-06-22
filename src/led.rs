// make Led on GPIO17 blink every .5 secconds
use rppal::gpio::OutputPin;
use std::thread::sleep;
use std::time::Duration;

pub struct Led {
    pin: OutputPin,
    rate: u64,
}

impl Led {
    pub fn new(pin: OutputPin, rate: u64) -> Self {
        Self {
            pin,
            rate: if rate > 0 { rate } else { 500 },
        }
    }

    pub fn on(&mut self) {
        self.pin.set_high();
    }

    pub fn off(&mut self) {
        self.pin.set_low();
    }

    pub fn blink(&mut self) {
        self.on();
        sleep(Duration::from_millis(self.rate));

        self.off();
        sleep(Duration::from_millis(self.rate));
    }
}
