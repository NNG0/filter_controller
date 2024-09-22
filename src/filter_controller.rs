use rppal::gpio::OutputPin;
use std::thread::sleep;
use std::time::Duration;

const RELAY_DELAY_MS: u64 = 50;

pub struct FilterController {
    relay1: OutputPin,
    relay2: OutputPin,
    running: bool,
}

impl FilterController {
    pub fn new(relay1: OutputPin, relay2: OutputPin) -> FilterController {
        FilterController {
            relay1,
            relay2,
            running: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start_filter_process(&mut self) {
        if !self.is_running() {
            self.relay1.set_low();
            sleep(Duration::from_millis(RELAY_DELAY_MS));
            self.relay2.set_low();
            println!("Relays turned on");
            sleep(Duration::from_millis(RELAY_DELAY_MS));
            self.running = true;
        }
    }

    pub fn stop_filter_process(&mut self) {
        if self.is_running() {
            self.relay1.set_high();
            sleep(Duration::from_millis(RELAY_DELAY_MS));
            self.relay2.set_high();
            println!("Relays turned off");
            sleep(Duration::from_millis(RELAY_DELAY_MS));
            self.running = false;
        }
    }
}
