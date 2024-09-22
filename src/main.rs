use ctrlc;
use rppal::gpio::Gpio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
mod filter_controller;

const RUN_DURATION_SECS: u64 = 10;
const PAUSE_DURATION_SECS: u64 = 20;
const MAX_FILTER_INTERVAL_SECS: u64 = 1800;

fn main() {
    println!("Process started");
    const RELAY_1_PIN: u8 = 2;
    const RELAY_2_PIN: u8 = 17;
    const SENSOR_PIN: u8 = 22;

    let gpio = Gpio::new().expect("Failed to access GPIO");
    let mut relay1 = gpio
        .get(RELAY_1_PIN)
        .expect("Failed to get relay 1")
        .into_output();
    let mut relay2 = gpio
        .get(RELAY_2_PIN)
        .expect("Failed to get relay 2")
        .into_output();
    let float_sensor = gpio
        .get(SENSOR_PIN)
        .expect("Failed to get sensor 2")
        .into_input_pullup();

    // ensure relays are off initially
    relay1.set_high();
    relay2.set_high();

    let mut filter_controller = filter_controller::FilterController::new(relay1, relay2);
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("Exiting... Turning off relays");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    filter_controller.start_filter_process();
    let mut last_filter_run = Instant::now();

    while running.load(Ordering::SeqCst) {
        let now = Instant::now();

        // check if it is time to start filter
        if !filter_controller.is_running()
            && now.duration_since(last_filter_run) >= Duration::from_secs(PAUSE_DURATION_SECS)
        {
            if float_sensor.is_low()
                || now.duration_since(last_filter_run)
                    >= Duration::from_secs(MAX_FILTER_INTERVAL_SECS)
            {
                filter_controller.start_filter_process();
                last_filter_run = Instant::now();
            }
        }

        // check if it is time to stop filter
        if filter_controller.is_running()
            && now.duration_since(last_filter_run) >= Duration::from_secs(RUN_DURATION_SECS)
        {
            filter_controller.stop_filter_process();
            last_filter_run = Instant::now();
        }

        sleep(Duration::from_secs(1));
    }

    filter_controller.stop_filter_process();
}
