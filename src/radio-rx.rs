//! Permits testing of radio reception.

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
use api::rf::{
    NRF24L01,
    Configuration,
    DataRate,
};
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
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 9600);

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
    // Set to channel 14: 2.414 GHz
    nrf24.set_frequency(14).unwrap();
    // Set radio frequency to 2 Mbps and -18 dBm
    // Change `0` to `3` to change to 0 dBm
    nrf24.set_rf(&DataRate::R2Mbps, 0).unwrap();
    // Set the acknowledgement packet to be 0x000000
    nrf24.set_auto_ack(&[false; 6]).unwrap();
    
    let mut rx = nrf24.rx().unwrap();

    loop {
        // Uncomment for debugging purposes only

        

        ufmt::uwriteln!(
            &mut serial,
            "{}",
            rx.can_read().unwrap().unwrap(),
        ).void_unwrap();

        

        // Note to developers: uncommenting the above block of code will add several kilobytes to the final compiled binary
        // Doing so may overload the ATMega328P flash memory (32kB maximum)

        arduino_hal::delay_ms(1000u16);
    }
}