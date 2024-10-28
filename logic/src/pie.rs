//! My own P-roportional I-ntegral E-xponential controller (an exponential PI controller)

use crate::magic;

/// Corrects for an error based upon the error itself, the largest possible (positive) error, the delta seconds and an integral, and returns a correction value from -1..=1
pub fn correct(
    error: f32,
    max_error: f32,
    delta_seconds: f32,
    integral: &mut f32,
) -> f32 {
    // calculate the proportional correction with Ethan's magic number
    let pc = magic::log_ethan(error / max_error);

    // calculate the integral
    let ic = *integral;
    *integral += magic::log_ethan(error / 180.0) * delta_seconds;
    *integral = integral.clamp(-1.0, 1.0);

    // return it
    (pc + ic).clamp(-1.0, 1.0)
}
