// Custom MPU6050 driver code

#![no_std]

pub struct Mpu6050<I> {
    i2c: I,
    address: u8
}

#[derive(Debug)]
pub enum Mpu6050Error<E> {
    I2c(E),
    Accel(E),
}

// Custom driver code
impl<I, E> Mpu6050<I>
    where I: embedded_hal::blocking::i2c::WriteRead<Error = E>
{
    // Creates a new instance of the Mpu6050 struct
    pub fn new(i2c: I) -> Self {
        Mpu6050 {
            i2c: i2c,
            address: DEFAULT_MPU6050_ADDRESS
        }
    }

    // Reads a series of bytes into the buffer from the specified register
    pub fn read_bytes(&mut self, register: u8, buffer: &mut [u8]) -> Result<(), Mpu6050Error<E>> {
        self.i2c.write_read(self.address, &[register], buffer)
            .map_err(Mpu6050Error::I2c)?;
        Ok(())
    }

    // Read accelerations
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


// Create a variable for the default I2C address for the IMU
const DEFAULT_MPU6050_ADDRESS: u8 = 0x68;

pub struct Accel {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// Create variables for the MPU6050 IMU registers
const _MPU6050_RESET_REGISTER: u8 = 0x6B;
const MPU6050_ACCEL_REGISTER: u8 = 0x3B;

impl Accel {
    
}