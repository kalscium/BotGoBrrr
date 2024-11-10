//! My own hand-rolled std-less maths library

#![no_std]
#![allow(internal_features)]
#![feature(core_intrinsics)]

use core::f32::consts::PI;

/// Raises an `f32` to the power of another `f32`
pub fn powf(x: f32, y: f32) -> f32 {
    unsafe { core::intrinsics::powf32(x, y) }
}

/// Finds the square-root of an `f32`
pub fn sqrt(x: f32) -> f32 {
    unsafe { core::intrinsics::sqrtf32(x) }
}

/// Divides a number while checking for zero
pub fn checked_div(x: f32, y: f32) -> Option<f32> {
    if y == 0. {
        None
    } else {
        Some(x / y)
    }
}

/// Approximates the Arc-Tan (in degrees) of an `f32`
pub fn atan(x: f32) -> f32 {
    // compute the atan
    let atan = powf(PI, 2.0) * x / (4.0 + sqrt((powf(PI, 2.0) - 4.0) * sqrt(32.0) + powf(2.0 * PI * x, 2.0)));

    // normalise it to be within -180..=180
    let normal = atan * 180.0 / PI;

    // return the nomalised version
    normal
}

/// Finds the sign number (1 or -1) of an f32
pub fn signumf(x: f32) -> f32 {
    if x.is_sign_negative() {
        -1.0
    } else {
        1.0
    }
}

/// Finds the absolute value of an f32
pub fn absf(x: f32) -> f32 {
    x * signumf(x)
}

/// Finds the average of two f32s
pub fn avgf(x: f32, y: f32) -> f32 {
    (x + y) / 2.0
}
