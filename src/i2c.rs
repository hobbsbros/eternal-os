// eternal-os::i2c

// For debugging and development purposes only
// This script can be used to get I2C addresses of
// peripheral devices

// In order to run this, change the `path` variable under
// [[bin]] in Cargo.toml to the location of this file

#![no_std]
#![no_main]

use arduino_hal::prelude::*;

// Custom panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // We don't need a fancy panic handler here
    loop {
        // Do nothing forever
    }
}

// Program entry point
#[arduino_hal::entry]
fn main() -> ! {
    // Get pins using the arduino_hal `pins!` macro
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 57600);

    // Create an I2C connection with a clock speed of 50 kHz
    let mut i2c = arduino_hal::I2c::new(
        peripherals.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        50_000,
    );

    // Program starts here
    
    ufmt::uwriteln!(&mut serial, "Write test").void_unwrap();
    i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Write).void_unwrap();
    
    ufmt::uwriteln!(&mut serial, "Read test").void_unwrap();
    i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Read).void_unwrap();

    loop {
        // Stop
    }
}