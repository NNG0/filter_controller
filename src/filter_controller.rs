use embassy_rp::gpio::Output;

pub struct FilterController<'a> {
    relay1: Output<'a>,
    relay2: Output<'a>,
    running: bool,
}

impl<'a> FilterController<'a> {
    pub fn new(relay1: Output<'a> , relay2: Output<'a>) -> Self {
        Self {
            relay1,
            relay2,
            running: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub async fn start_filter_process(&mut self) {
        if !self.is_running() {
            self.relay1.set_high();
            self.relay2.set_high();
            self.running = true;
        }
    }

    pub async fn stop_filter_process(&mut self) {
        if self.is_running() {
            self.relay1.set_low();
            self.relay2.set_low();
            self.running = false;
        }
    }
}
