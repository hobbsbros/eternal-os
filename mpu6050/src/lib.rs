//! Custom MPU6050 Driver Code for the open-source eternalOS flight control software.
//! This driver receives acceleration data from the MPU6050 6-axis accelerometer/gyroscope.

#![no_std]

/// Defines an abstract object representing an MPU6050 unit.
pub struct Mpu6050<I> {
    i2c: I,
    address: u8
}

/// Defines an enumerated type for different types of errors that the MPU6050 driver could generate.
pub enum Mpu6050Error<E> {
    I2c(E),
    Accel(E),
}

/// Methods available on the `Mpu6050` struct.
impl<I, E> Mpu6050<I>
    where I: embedded_hal::blocking::i2c::WriteRead<Error = E> + embedded_hal::blocking::i2c::Write<Error = E>
{
    /// Creates a new instance of the Mpu6050 struct.
    pub fn new(i2c: I) -> Self {
        Mpu6050 {
            i2c: i2c,
            address: DEFAULT_MPU6050_ADDRESS
        }
    }

    /// Reads a series of bytes into a given buffer buffer from the specified register.
    /// 
    /// # Examples
    /// ```
    /// # let peripherals = arduino_hal::Peripherals::take().unwrap();
    /// # let pins = arduino_hal::pins!(peripherals);
    /// # #[allow(unused_mut)]
    /// # let mut i2c = arduino_hal::I2c::new(
    /// #     peripherals.TWI,
    /// #     pins.a4.into_pull_up_input(),
    /// #     pins.a5.into_pull_up_input(),
    /// #     50_000,
    /// # );
    /// let mpu6050 = Mpu6050::new(i2c);
    /// let mut buffer: [u8; 6] = [0; 6];
    /// mpu6050.read_bytes(MPU6050_ACCEL_REGISTER, &mut buffer)?;
    /// ```
    /// 
    /// # Panics
    /// This function never panics.
    /// 
    /// # Errors
    /// This function returns a `Result` of unit type `()` or an `Mpu6050Error::I2c` error.
    /// See documentation for `Mpu6050Error` for more information about this error.
    /// 
    /// # Safety
    /// This function does not require `unsafe` code.
    pub fn read_bytes(&mut self, register: u8, buffer: &mut [u8]) -> Result<(), Mpu6050Error<E>> {
        self.i2c.write_read(self.address, &[register], buffer)
            .map_err(Mpu6050Error::I2c)?;
        Ok(())
    }

    pub fn wake(&mut self) -> Result<(), Mpu6050Error<E>> {
        self.i2c.write(self.address, &[MPU6050_RESET_REGISTER, 0x00])
            .map_err(Mpu6050Error::I2c)?;
        Ok(())
    }

    /// Read the X, Y, and Z accelerations as measured by the MPU6050.
    pub fn read_accel(&mut self) -> Result<Accel, Mpu6050Error<E>> {
        let mut buffer: [u8; 6] = [0; 6];
        self.read_bytes(MPU6050_ACCEL_REGISTER, &mut buffer)?;

        // Unpack values
        let acc_x: i32 = (buffer[0] as i32) << 8 | buffer[1] as i32;
        let acc_y: i32 = (buffer[2] as i32) << 8 | buffer[3] as i32;
        let acc_z: i32 = (buffer[4] as i32) << 8 | buffer[5] as i32;

        // Return struct instance
        let accelerations = Accel {
            x: acc_x,
            y: acc_y,
            z: acc_z
        };

        Ok(accelerations)
    }
}


/// This is the default MPU6050 I2C address.
const DEFAULT_MPU6050_ADDRESS: u8 = 0x68;

pub struct Accel {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// This is the reset register on board the MPU6050.
/// Writing bytes to this register will reset the IMU.
#[allow(dead_code)]
const MPU6050_RESET_REGISTER: u8 = 0x6B;

/// This is the address of the first acceleration register on the MPU6050.
/// `0x3B` and `0x3C` together hold the X acceleration, `0x3D` and `0x3E` together hold the Y acceleration, and `0x3F` and `0x40` together hold the Z acceleration.
/// See the documentation for `Mpu6050::read_accel` for more information.
const MPU6050_ACCEL_REGISTER: u8 = 0x3B;

impl Accel {
    
}