Automatic Plant Watering System
===============================

By Bryce W. Frazier and Joey Kirnan

Rust project for the _Arduino Uno_, _CS241_ 2025 Spring at UAF. Full documentation can be aquired by running 
`cargo doc --open` in the repo directory.


## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.


## Libaries Used
- [`avr-hal`](https://github.com/Rahix/avr-hal)
- [`ravedude`]


# Hardware Info
Built from Arduino UNO Platform

**Major Components**
- [SparkFun Soil Moisture Sensor](https://www.sparkfun.com/sparkfun-soil-moisture-sensor.html)
- [Pulco DC USB Water Pump](https://www.amazon.com/PULACO-Submersible-Fountain-Aquarium-Hydroponics/dp/B07Y27SVPP/)

**Pins**
- 5V:  Primary Power Bus
- GND: Central Ground Bus
- D2:  Tank Low Indicator
- D3:  Overflow Sensor
- D7:  Pump Relay Actuator/Switch
- A0:  Soil Sensor Input
- A1:  Soil Sensor Power (Dynamic) **WIP**


**TODO**

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude
