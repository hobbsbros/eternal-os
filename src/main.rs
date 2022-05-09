// eternal-os::main

#![no_std]
#![no_main]

use panic_halt as _;

// Program entry point
#[arduino_hal::entry]
fn main() -> ! {
    // Get pins using the arduino_hal `pins!` macro
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    
    // Program starts here

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}