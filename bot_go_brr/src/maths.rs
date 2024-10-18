//! My own hand-rolled std-less maths library

use core::f64::consts::PI;

/// Raises an `f64` to the power of another `f64`
pub fn powf(x: f64, y: f64) -> f64 {
    unsafe { core::intrinsics::powf64(x, y) }
}

/// Finds the square-root of an `f64`
pub fn sqrt(x: f64) -> f64 {
    unsafe { core::intrinsics::sqrtf64(x) }
}

/// Approximates the Arc-Tan (in degrees) of an `f64`
pub fn atan(x: f64) -> f64 {
    // compute the atan
    let atan = powf(PI, 2.0) * x / (4.0 + sqrt((powf(PI, 2.0) - 4.0) * sqrt(32.0) + powf(2.0 * PI * x, 2.0)));

    // normalise it to be within -180..=180
    let normal = atan * 180.0 / PI;

    // return the nomalised version
    normal
}

/// Finds the sign number (1 or -1) of an f64
pub fn signumf(x: f64) -> f64 {
    if x.is_sign_negative() {
        -1.0
    } else {
        1.0
    }
}

/// Finds the absolute value of an f64
pub fn absf(x: f64) -> f64 {
    x * -signumf(x)
}
