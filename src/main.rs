#![doc = include_str!("../README.md")]
//! # _main.rs_ Info
//!
//! - rustc 1.79.0-nightly
//! - By: Bryce W. Frazier, Joey Kirnan
//! - Started: 2025-02-21 Updated: 2025-02-23
//! - Main file for CS241 project, automatic plant wating system
#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use arduino_hal::port; // for pin numbers D7, D3, etc., idealy this gets fixed
use arduino_hal::port::Pin; 
use arduino_hal::port::mode; 
use arduino_hal::port::mode::PullUp;
use ufmt::uwriteln;

//  alarm functions
//
//  overflow_alarm
/// checks if pot is overflowing, if so returns a flag set to true, 
/// otherwise false.
fn overflow_alarm(sensor_pin: &Pin<mode::Input<PullUp>, port::D3>) -> bool {
    if sensor_pin.is_low() { 
        return true;
    } 
    return false;
}

//  tank_low_alarm
/// checks if the water reservoir is low, if so then returns a flag set
/// to true and turns on a indactor led, otherwise flag is false and
/// the led will be turned off.
fn tank_low_alarm(sensor_pin: &Pin<mode::Input::<PullUp>, port::D4>, 
    led_pin: &mut Pin<mode::Output, port::D2>) -> bool {

    if sensor_pin.is_high() {
        led_pin.set_high();
        return true;    
    }
    led_pin.set_low();
    return false;
}

// Pump
// 
/// Manages the pump, regulates how much water is pumped and when
/// water can be pumped.
struct Pump {
    switch_pin: Pin<mode::Output, port::D7>,
    flow_rate_liter_sec: u8,
}

impl Pump {
    //  water_plant
    /// Run the pump long anough to get the approximate proper amount of water 
    /// in the pot basied from the pump's flow rate and size of pot. 
    /// Will check that no alarms relating to flooding or an empty are 
    /// present during and before excution.
    fn water_plant(&mut self) {
        self.switch_pin.set_high(); // TODO Dummy
        //
    }

    //  stop_pump
    /// Cuts stops the pump, may be called if the pump is running or not.
    fn stop_pump(&mut self) {
        self.switch_pin.set_low();
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    //Pins init
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    //Control/Input pins for devices
    let pump_switch = pins.d7.into_output();
    let mut error_led = pins.d2.into_output();
    let soil_prob = pins.a0.into_analog_input(&mut adc);
    let overflow_detector = pins.d3.into_pull_up_input();
    let tank_low = pins.d4.into_pull_up_input();

    //Init objects
    let mut pump = Pump {
        switch_pin: pump_switch,
        flow_rate_liter_sec: 1,
    };
    
    //Init serial
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    loop {
        //Get soil moisture and print to serial
        let input = soil_prob.analog_read(&mut adc);
        uwriteln!(&mut serial, "{}", input)
            .unwrap_infallible();

        //Shut down pump If needed
        while overflow_alarm(&overflow_detector) |
        tank_low_alarm(&tank_low, &mut error_led) {

          pump.stop_pump();
        } 

        //run pump when soil is dry
        if input < 256 {
            pump.water_plant();
        } else {
            pump.stop_pump();
        }
    }
}
