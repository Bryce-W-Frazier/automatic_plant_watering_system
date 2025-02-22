// main.rs (rustc 1.79.0-nightly)
// Bryce W. Frazier
// Started: 2025-02-21
// Updated: 2025-02-21
//
// Main file for CS241 project, automatic plant wating system
#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp); 

    let mut led = pins.d13.into_output();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello, World!").unwrap_infallible();

    loop {
        ufmt::uwriteln!(&mut serial, "hello").unwrap_infallible();
        led.toggle();
        arduino_hal::delay_ms(60000);
    }
}
