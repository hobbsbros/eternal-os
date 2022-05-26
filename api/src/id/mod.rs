// crate::remote-id

use crate::gps as gps;
use crate::rtc as rtc;


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


/// Holds Remote ID information.
pub struct RemoteID {
    pub serial_number: SerialNumber,
    pub ctrl_pos: gps::Position,
    pub uas_pos: gps::Position,
    pub uas_velocity: gps::Velocity,
    pub timestamp: rtc::Timestamp,
    pub status: sys::StatusCode,
}

impl RemoteID {
    /// Constructs a new `RemoteID` for the Phoenix drone.
    #[inline(always)]
    pub fn new(
        serial_number: SerialNumber,
        ctrl_pos: gps::Position,
        uas_pos: gps::Position,
        uas_velocity: gps::Velocity,
        timestamp: rtc::Timestamp,
        status: sys::StatusCode,
    ) -> Self {
        RemoteID {
            serial_number,
            ctrl_pos,
            uas_pos,
            uas_velocity,
            timestamp,
            status,
        }
    }

    /// Converts an instance of the `RemoteID` struct into a bit vector, prepared for transmitting over radio.
    pub fn to_bits(&self) {
        todo!();
    }
}