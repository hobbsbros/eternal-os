// eternal-os::main

#![no_std]
#![no_main]

use arduino_hal::prelude::*;


// Implementation of custom `core_unwrap` function for core::result::Result
trait CoreUnwrap {
    fn core_unwrap(&mut self);
}

impl<T, E> CoreUnwrap for core::result::Result<T, E> {
    fn core_unwrap(&mut self) {
        if let Err(_) = self {
            panic!()
        }
    }
}


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


// Create variables for the MPU6050 IMU registers
const RESET_REGISTER: u8 = 107;
const ACCEL_REGISTER: u8 = 59;


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
    #[allow(unused_mut)]
    let mut i2c = arduino_hal::I2c::new(
        peripherals.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50_000,
    );

    // Program starts here

    // Create an instance of the Mpu6050 struct to represent the accelerometer
    let mut mpu6050 = mpu6050::Mpu6050::new(i2c);
    // Initialize the MPU6050
    mpu6050.init().core_unwrap();
    arduino_hal::delay_ms(50);

    // let mut led = pins.d13.into_output();

    let mut raw_bytes: [u8; 6] = [0u8; 6];

    loop {
        mpu6050.read_bytes(ACCEL_REGISTER, &mut raw_bytes).core_unwrap();

        // Unpack values
        let acc_x: i32 = (raw_bytes[0] as i32) << 8 | raw_bytes[1] as i32;
        let acc_y: i32 = (raw_bytes[2] as i32) << 8 | raw_bytes[3] as i32;
        let acc_z: i32 = (raw_bytes[4] as i32) << 8 | raw_bytes[5] as i32;

        // Display directional accelerations
        ufmt::uwriteln!(&mut serial, "x acc: {}\ny acc: {}\nz acc: {}\n", acc_x, acc_y, acc_z).void_unwrap();

        arduino_hal::delay_ms(1000);
        raw_bytes = [0; 6];
    }
}