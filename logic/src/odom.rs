//! Functions for an odometry coordinate system
//!
//! **Note:** All measurements are in millimetres

use core::f32::consts::PI;

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
        delta1 // slight preference for diff 1
    }
}

/// Modifies an absolute coordinate based upon the current and previous angle of the rotation sensor
pub fn account_for(current: f32, prev: &mut f32, coord: &mut f32) {
    // find the delta
    let angle_delta = lowest_rot_delta(*prev, current);

    // update the previous angle to the current one
    *prev = current;

    // find the coordinate delta though the circumference of the wheel
    const CIRCUMFERENCE: f32 = DIAMETER*PI; // circumference in millimeters
    let coord_delta = angle_delta/360. * CIRCUMFERENCE;

    // add the coordinate delta to the coordinate
    *coord += coord_delta;
}
