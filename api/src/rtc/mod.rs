//! API for the onboard Real-Time Clock.

/// Holds timestamp information.
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