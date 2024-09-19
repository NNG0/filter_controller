use rppal::gpio::Gpio;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let gpio = Gpio::new().expect("Failed to access GPIO");
    let mut pin17 = gpio.get(17).expect("Failed to get pin 17").into_output();

    loop {
        pin17.set_high();
        println!("LED ON");
        sleep(Duration::from_secs(1));

        pin17.set_low();
        println!("LED OFF");
        sleep(Duration::from_secs(1));
    }
}
