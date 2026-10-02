#![no_std] #![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::{Duration, Instant, Timer};
mod filter_controller;
use {defmt_rtt as _, panic_probe as _};

const RUN_DURATION: Duration = Duration::from_secs(10);
const PAUSE_DURATION: Duration = Duration::from_secs(20);
const MAX_FILTER_INTERVAL: Duration = Duration::from_secs(1800);
const TICK_INTERVAL: Duration = Duration::from_millis(200);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let relay1 = Output::new(p.PIN_0, Level::Low);
    let relay2 = Output::new(p.PIN_1, Level::Low);
    let float_sensor = Input::new(p.PIN_2, Pull::Up);

    let mut filter_controller = filter_controller::FilterController::new( relay1, relay2);

    let mut last_filter_run = Instant::now();

    if float_sensor.is_low() {
        filter_controller.start_filter_process();
        last_filter_run = Instant::now();
    }

    loop {
        let now = Instant::now();
        let elapsed = now.checked_duration_since(last_filter_run).unwrap_or_default();
        let float_low = float_sensor.is_low();

        // check if it is time to start filter
        if !filter_controller.is_running() {
            if elapsed >= PAUSE_DURATION && (float_low || elapsed >= MAX_FILTER_INTERVAL) {
                filter_controller.start_filter_process();
                last_filter_run = now;

            } 
        }
        else {
            if elapsed >= RUN_DURATION {
                filter_controller.stop_filter_process();
                last_filter_run = now;
            }
        }
            
        Timer::after(TICK_INTERVAL).await;
    }
}
