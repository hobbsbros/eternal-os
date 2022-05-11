//! Some simple math functions used by the MPU6050 driver.

#![no_std]

/// Defines a function that can take integer powers of 32-bit floating numbers.
///
/// # Examples
/// ```
/// ```
/// 
/// # Panics
/// 
/// # Errors
///
/// # Safety
/// 
pub fn pow(base: f32, power: u8) -> f32 {
    let mut result: f32 = 1.0;
    for _ in 0..power {
        result *= base;
    }
    result
}

/// Returns the **approximate** value for the arctangent of x (in radians).
/// This requires that the input is between -0.7 and 0.7 to get accurate results.
///
/// # Examples
/// ```
/// ```
/// 
/// # Panics
/// 
/// # Errors
///
/// # Safety
/// 
pub fn arctan(x: f32) -> f32 {
    let arctan_x: f32 = x - pow(x, 3)/3.0 + pow(x, 5)/5.0;
    arctan_x
}