# filter_controller

Simple controller for a pool filter. The programm is run on a raspberry pi pico2.

## setup
create binary
```bash
cargo build --release --target riscv32imac-unknown-none-elf

libusb, base-devel, cmake, pkg-config
```

get picotool
https://github.com/raspberrypi/pico-sdk-tools/releases/tag/v2.2.0-3

add to path
sudo install -m 755 picotool /usr/local/bin/picotool

## flashing
To Flash your application onto the Pico 2, press and hold the BOOTSEL button. While holding it, connect the Pico 2 to your computer using a micro USB cable. You can release the button once the USB is plugged in.

cargo build --target=thumbv8m.main-none-eabihf
