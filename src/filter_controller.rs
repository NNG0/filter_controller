use rppal::gpio::OutputPin;
use std::thread::sleep;
use std::time::Duration;

pub struct FilterController {
    relay1: OutputPin,
    relay2: OutputPin,
}

impl FilterController {
    pub fn new(relay1: OutputPin, relay2: OutputPin) -> FilterController {
        FilterController {
            relay1,
            relay2
        }
    }

    fn start_filter_process(&mut self) {
        self.relay1.set_high();
        self.relay2.set_low();
    }

    fn stop_filter_process(&mut self) {
        self.relay1.set_low();
        self.relay2.set_low();
    }

    pub fn run_filter_process(&mut self) {
       self.start_filter_process();
       sleep(Duration::from_secs(5));
       self.stop_filter_process();
       sleep(Duration::from_secs(5));
    }
} 