// crate::subsystems

#![no_std]

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


/// Enumerates drone subsystems.
///
/// # Options
/// `Propulsion`: propulsion subsystem
/// `Radio`: radio transmission subsystem
/// `RemoteID`: FAA-compliant Remote ID subsystem
/// `Guidance`: positional guidance and PID control subsystems
/// `Power`: electrical power subsystem
#[repr(u8)]
pub enum Subsystem {
    Propulsion = 1,
    Radio = 2,
    RemoteID = 3,
    Guidance = 4,
    Power = 5,
}


/// Enumerates drone subsystem statuses.
/// 
/// `Ok`: Subsystem is operational.
/// `Emergency`: Subsystem is in emergency.
#[repr(u8)]
pub enum SubsystemStatus {
    Ok = 0,
    Emergency = 255,
}


/// Enumerates drone status.
///
/// `Ok`: All systems are operational.
/// `Emergency`: One or more subsystems are in emergency.
#[repr(u8)]
pub enum Status {
    Ok = 0,
    Emergency = 255,
}