//! Functions for an odometry coordinate system
//!
//! **Note:** All measurements are in millimetres

/// The diameter of the robot's wheel in **mm**
const DIAMETER: f32 = 69.85;

/// Finds the difference in rotation of the rotation sensors (in degrees)
pub fn lowest_rot_delta(x: f32, y: f32) -> f32 {
    // get the first possible difference in angle
    let delta1 = y - x;

    // get the second possible difference in angle
    let delta2 = delta1 - 360. * maths::signumf(delta1);

    // return the smaller difference
    if maths::absf(delta2) < maths::absf(delta1) {
        delta2
    } else {
        delta1 // slight preference for diff 2
    }
}
