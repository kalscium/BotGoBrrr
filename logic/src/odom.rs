//! Functions for an odometry coordinate system
//!
//! **Note:** All measurements are in millimetres

use core::f32::consts::PI;

/// The diameter of the robot's wheel in **mm**
const DIAMETER: f32 = 101.6;

/// Finds the difference in rotation of the rotation sensors (in degrees)
pub fn lowest_rot_delta(prev: f32, measurement: f32) -> f32 {
    // get the first possible difference in angle
    let delta1 = measurement - prev;

    // get the second possible difference in angle
    let delta2 = delta1 - 360. * maths::signumf(delta1);

    // return the smaller difference
    if maths::absf(delta2) < maths::absf(delta1) {
        delta2
    } else {
        delta1 // slight preference for diff 1
    }
}

#[derive(Debug, Clone)]
pub struct OdomState {
    /// The previous left rotational sensor for measuring the y coordinate
    prev_ly: f32,
    /// The previous right rotational sensor for measuring the y coordinate
    prev_ry: f32,
    /// The current y-coordinate
    pub y_coord: f32,
}

/// Approximates the distance travelled from the previous and current angle of the rotational sensor
pub fn distance(current: f32, prev: &mut f32) -> f32 {
    // find the delta
    let angle_delta = lowest_rot_delta(*prev, current);

    // update the previous angle
    *prev = current;

    // calculate the distance travelled from the angle
    const CIRCUMFERENCE: f32 = DIAMETER * PI; // circumference in mm
    let coord_delta = angle_delta / 360. * CIRCUMFERENCE;

    // return it
    coord_delta
}

/// Modifies an absolute coordinate based upon the current and previous angle of the rotation sensors
pub fn account_for(
    current_ly: f32,
    current_ry: f32,
    state: &mut OdomState,
) {
    // find the coordinate delta for the left y rotation sensor
    let ly_delta = distance(current_ly, &mut state.prev_ly);

    // find the coordinate delta for the right y rotation sensor
    let ry_delta = distance(current_ry, &mut state.prev_ry);

    // average the deltas for the y coord
    let y_delta = maths::avgf(ly_delta, ry_delta);

    // add the coordinate deltas to the state
    state.y_coord += y_delta;
}
