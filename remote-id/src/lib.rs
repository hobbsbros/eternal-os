// crate::remote-id

#![no_std]

use system;

/// Holds Remote ID information.
pub struct RemoteID {
    pub serial_number: system::SerialNumber,
    pub ctrl_pos: system::Position,
    pub uas_pos: system::Position,
    pub uas_velocity: system::Velocity,
    pub timestamp: system::Timestamp,
    pub status: system::StatusCode,
}

impl RemoteID {
    /// Constructs a new `RemoteID` for the Phoenix drone.
    #[inline(always)]
    pub fn new(
        serial_number: system::SerialNumber,
        ctrl_pos: system::Position,
        uas_pos: system::Position,
        uas_velocity: system::Velocity,
        timestamp: system::Timestamp,
        status: system::StatusCode,
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