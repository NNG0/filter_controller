use rppal::gpio::Gpio;
use std::time::{Duration, Instant};
mod filter_controller;

fn main() {
    const RELAY_1_PIN: u8 = 17;
    const RELAY_2_PIN: u8 = 27;
    const SENSOR_2_PIN: u8 = 22;

    let gpio = Gpio::new().expect("Failed to access GPIO");
    let relay1 = gpio.get(RELAY_1_PIN).expect("Failed to get relay 1").into_output();
    let relay2 = gpio.get(RELAY_2_PIN).expect("Failed to get relay 2").into_output();
    let float_sensor = gpio.get(SENSOR_2_PIN).expect("Failed to get sensor 2").into_input_pullup();

    let mut filter_controller = filter_controller::FilterController::new( relay1, relay2); 

    let mut last_filter_run = Instant::now();

    loop {
        let now =Instant::now();
        if float_sensor.is_low() || now.duration_since(last_filter_run) >= Duration::from_secs(1800) {
            filter_controller.run_filter_process();
            last_filter_run = Instant::now();
        }
    }
}
