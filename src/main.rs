#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::{Duration, Instant};
mod filter_controller;
use {defmt_rtt as _, panic_probe as _};

const RUN_DURATION_SECS: u64 = 10;
const PAUSE_DURATION_SECS: u64 = 20;
const MAX_FILTER_INTERVAL_SECS: u64 = 1800;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let relay1 = Output::new(p.PIN_2, Level::Low);
    let relay2 = Output::new(p.PIN_17, Level::Low);
    let float_sensor = Input::new(p.PIN_22, Pull::Up);

    let mut filter_controller = filter_controller::FilterController::new( relay1, relay2);

    if float_sensor.is_low() {
        filter_controller.start_filter_process().await;
    }

    let mut last_filter_run = Instant::now();

    loop {
        let now = Instant::now();

        // check if it is time to start filter
        if !filter_controller.is_running()
            && now.checked_duration_since(last_filter_run).unwrap_or_default()
                >= Duration::from_secs(PAUSE_DURATION_SECS)
        {
            if float_sensor.is_low()
                || now.checked_duration_since(last_filter_run).unwrap_or_default()
                    >= Duration::from_secs(MAX_FILTER_INTERVAL_SECS)
            {
                filter_controller.start_filter_process().await;
                last_filter_run = Instant::now();
            }
        }

        // check if it is time to stop filter
        if filter_controller.is_running()
            && now.checked_duration_since(last_filter_run).unwrap_or_default()
                >= Duration::from_secs(RUN_DURATION_SECS)
        {
            filter_controller.stop_filter_process().await;
            last_filter_run = Instant::now();
        }
    }
}
