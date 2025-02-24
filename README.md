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

**TODO**

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude
