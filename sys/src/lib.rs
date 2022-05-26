//! System information.

#![no_std]

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