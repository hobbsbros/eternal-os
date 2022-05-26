//! API for the onboard Global Position System module.

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