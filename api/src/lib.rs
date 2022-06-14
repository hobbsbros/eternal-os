//! Device API for the Phoenix flight controller board.

#![no_std]

pub mod imu;
pub mod id;
pub mod gps;
pub mod rtc;
pub mod pid;

pub use embedded_nrf24l01 as rf;