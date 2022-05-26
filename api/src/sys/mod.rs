//! Defines system information as used by the `radio` and `remote-id` crates.

/// Holds UAS serial number information.
///
/// # Fields
/// `bytes`: An array of 20 bytes indicating the ASCII encoding of the UAS's serial number.
pub struct SerialNumber {
    pub bytes: [u8; 20],
}

impl SerialNumber {
    /// Constructs a new `SerialNumber` struct.
    pub fn new(bytes: [u8; 20]) -> Self {
        SerialNumber {
            bytes: bytes,
        }
    }
}


/// Holds Remote ID broadcast timestamp information.
/// 
/// # Fields
/// `year`: Year in the Gregorian calendar
/// `month`: Month (`1` = January, ..., `12` = December)
/// `day`: Numeric day (ranges from 1 to 31)
/// `millis`: Milliseconds past midnight (UTC)
pub struct Timestamp {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub millis: u32,
}


/// Holds position data (for a UAS or its control station).
///
/// # Fields
/// `lat`: Latitude (positive indicates north, negative indicates south)
/// `long`: Longitude (positive indicates east, negative indicates west)
/// `alt`: Altitude above sea level
pub struct Position {
    pub lat: f32,
    pub long: f32,
    pub alt: u16,
}


/// Holds unmanned aerial system (UAS) velocity information.
///
/// # Fields
/// `x`: Easterly velocity (positive is to the east, negative is to the west)
/// `y`: Northerly velocity (positive is to the north, negative is to the south)
/// `z`: Vertical velocity (positive is upwards, negative is downwards)
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


/// Enumerates subsystem status codes.
///
/// # Options
/// `Ok`: All systems are operational.
/// `Emergency`: One or more subsystems are in emergency.
#[repr(u8)]
pub enum StatusCode {
    Ok = 0,
    Emergency = 255,
}


/// Holds drone subsystem statuses.
/// 
/// # Fields
/// `propulsion`: propulsion subsystem statrus
/// `radio`: radio transmission subsystem status
/// `remote_id`: FAA-compliant Remote ID subsystem status
/// `guidance`: positional guidance and PID control subsystems status
/// `power`: electrical power subsystem status
#[allow(dead_code)]
pub struct Status {
    propulsion: StatusCode,
    radio: StatusCode,
    remote_id: StatusCode,
    guidance: StatusCode,
    power: StatusCode,
}