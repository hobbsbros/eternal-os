//! Main code for the Phoenix flight controller.

#![no_std]
#![no_main]

#[allow(unused_imports)]
use arduino_hal::prelude::*;
use arduino_hal::spi::Settings;

use api::imu::{
    Mpu6050,
};
#[allow(unused_imports)]
use api::id::RemoteID;
#[allow(unused_imports)]
use api::rf::NRF24L01;
use api::pid::{
    ControlVariable,
};

// Implementation of custom `core_unwrap` function for core::result::Result
trait Unwrap<T, E> {
    fn unwrap(self) -> T;
    fn unwrap_or(self, alt: T) -> T;
}

impl<T, E> Unwrap<T, E> for core::result::Result<T, E> {
    /// Provides a simple implementation for `Result::unwrap()`.
    fn unwrap(self) -> T {
        match self {
            Ok(t) => t,
            Err(_) => panic!(),
        }
    }

    /// Provides a simple implementation for `Result::unwrap_or(alternative)`.
    fn unwrap_or(self, alt: T) -> T {
        match self {
            Ok(t) => t,
            Err(_) => alt,
        }
    }
}


/// Defines a custom panic handler for the eternalOS flight control software.
/// 
/// # Examples
/// The following code will call the `panic` panic handler.
/// ```no_run
/// panic!();
/// ```
/// 
/// # Attributes
/// `panic` has the `#[panic_handler]` attribute, marking it as the program's panic handler.
/// 
/// # Panics
/// This function never panics (as it is the panic handler).
/// 
/// # Errors
/// This function returns no errors.
/// 
/// # Safety
/// This function requires `unsafe` code to gain access to the Arduino HAL peripherals using the `arduino_hal::Peripherals::steal()` function.
/// No invariants are required as the main firmware routine will never access the peripherals after the panic handler has been executed (because the main firmware has panicked).
/// 
#[panic_handler]
#[allow(unused_variables)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // First steal access to the peripherals from `main`
    // This is considered `unsafe` but because `main` will never
    // be using references to the peripherals again, it is acceptable
    let peripherals = unsafe {
        arduino_hal::Peripherals::steal()
    };
    let pins = arduino_hal::pins!(peripherals);

    // Note to developers: uncommenting the following block (to initiate a serial connection) will add 600 B to 700 B to the final compile

    /*

    // Set up a serial connection to inform the user of the panic!
    // This obviously assumes that the user is plugged in.
    
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
    
    */

    // Blink the LED rapidly to alert the user to the error
    let mut led = pins.d13.into_output();
    loop {

        led.toggle();
        arduino_hal::delay_ms(100);
    }
}


/// Provides an entry point for the program.
#[arduino_hal::entry]
fn main() -> ! {
    // Get pins using the arduino_hal `pins!` macro
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    
    // Set up a serial connection
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 57600);

    // Set SDA (serial data) and SCL (serial clock) lines
    let sda = pins.a4;
    let scl = pins.a5;

    // Set up an I2C connection
    #[allow(unused_mut)]
    let mut i2c = arduino_hal::I2c::new(
        peripherals.TWI,
        sda.into_pull_up_input(),
        scl.into_pull_up_input(),
        50_000,
    );

    // Actual user program begins here

    // Create an instance of the Mpu6050 struct to represent the accelerometer
    let mut mpu6050 = Mpu6050::new(i2c);
    // Wake up the MPU6050
    mpu6050.wake().unwrap();

    // Create an instance of the NRF24L01 struct to represent the radio transceiver
    let ce = pins.d3.into_output();
    let csn = pins.d4.into_output();
    let (mut spi, _) = arduino_hal::Spi::new(
        peripherals.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        Settings::default(),
    );

    let mut nrf24 = NRF24L01::new(ce, csn, spi).unwrap();

    // Define a timestep (in microseconds)
    const TIMESTEP: u16 = 1000;

    let mut control_roll = ControlVariable::new(0.0f32, TIMESTEP);
    let mut control_pitch = ControlVariable::new(0.0f32, TIMESTEP);

    control_pitch.set_expected(0.0);

    loop {
        #[allow(unused_variables)]
        let angles = mpu6050.read_angles().unwrap();

        control_roll.step(angles.roll);
        control_pitch.step(angles.pitch);

        #[allow(unused_variables)]
        let roll_correction = control_roll.get_correction();
        #[allow(unused_variables)]
        let pitch_correction = control_pitch.get_correction();

        // Uncomment for debugging purposes only

        /*

        ufmt::uwriteln!(
            &mut serial,
            "roll: {} | correction: {} || pitch: {} | correction: {}\n",
            angles.roll as i32,
            roll_correction as i32,
            angles.pitch as i32,
            pitch_correction as i32,
        ).void_unwrap();

        */

        // Note to developers: uncommenting the above block of code will add several kilobytes to the final compiled binary
        // Doing so may overload the ATMega328P flash memory (32kB maximum)

        arduino_hal::delay_us(TIMESTEP as u32);
    }
}