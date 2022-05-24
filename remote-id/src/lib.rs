// crate::remote-id

#![no_std]

use system;

/// Holds Remote ID information.
pub struct RemoteID {
    pub serial_number: [u8; 20],
    pub ctrl_lat: f32,
    pub ctrl_long: f32,
    pub ctrl_alt: u16,
    pub uas_lat: f32,
    pub uas_long: f32,
    pub uas_alt: u16,
    pub uas_velocity: system::Velocity,
    pub timestamp: system::Timestamp,
    pub status: system::Status,
}

impl RemoteID {

}