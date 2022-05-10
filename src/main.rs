// eternal-os::main

#![no_std]
#![no_main]

use arduino_hal::prelude::*;

// Custom panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // First steal access to the peripherals from `main`
    // This is considered `unsafe` but because `main` will never
    // be using references to the peripherals again, it is acceptable
    let peripherals = unsafe {
        arduino_hal::Peripherals::steal()
    };
    let pins = arduino_hal::pins!(peripherals);
    
    // Set up a serial connection to inform the user (in case they're plugged in)
    // of the panic!
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 57600);

    ufmt::uwriteln!(&mut serial, "eternalOS panic!").void_unwrap();
    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "Panic occurred in file `{}` at line number {}",
            loc.file(),
            loc.line()
        ).void_unwrap();
    } else {
        ufmt::uwriteln!(&mut serial, "Unable to determine panic location").void_unwrap();
    }

    // Blink the LED rapidly to alert the user to the error
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

// Program entry point
#[arduino_hal::entry]
fn main() -> ! {
    // Get pins using the arduino_hal `pins!` macro
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    
    // Set up a serial connection
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 57600);

    // Set up an I2C connection
    // #[allow(unused_variables)]
    // #[allow(unused_mut)]
    let mut i2c = arduino_hal::I2c::new(
        peripherals.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50_000,
    );

    

    // Create a variable for the MPU6050 IMU
    let mpu6050: u8 = 68;
    
    // Program starts here

    // let mut led = pins.d13.into_output();

    loop {

    }
}