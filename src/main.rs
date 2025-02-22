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
    //Pins init
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    //Control/Input pins for devices
    let mut pump = pins.d7.into_output();
    let mut error_led = pins.d2.into_output();
    let soil_prob = pins.a0.into_analog_input(&mut adc);
    let overflow_detector = pins.d3.into_pull_up_input();
    
    //Init serial
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    loop {
        let input = soil_prob.analog_read(&mut adc);
        ufmt::uwriteln!(&mut serial, "{}", input).unwrap_infallible();

        //Stop if plant pot is overflowing
        while overflow_detector.is_low() {
            pump.set_low(); // to insure pump is off
            error_led.set_high(); // Visual indcator TODO use for empty tank warning NOT OVERFLOW
            ufmt::uwriteln!(&mut serial, "Error:\tContainer Overflow").unwrap_infallible();
            arduino_hal::delay_ms(10);
            error_led.set_low();
        }

        if input < 256 {
            pump.set_high();
        } else {
            pump.set_low();
        }
    }
}
