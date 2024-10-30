//! Logic code for the drive-train of the robot

use crate::{debug, info, magic, pie};

/// Performs an arcade drive transformation on x and y values (-12000..=12000) to produce left and right drive voltages
pub fn arcade(x: i32, y: i32) -> (i32, i32) {
    let ldr = (y + x).clamp(-12000, 12000);
    let rdr = (y - x).clamp(-12000, 12000);

    (ldr, rdr)
}


/// Finds the lowest difference in angle between two angles (-180..=180 (for both arguments and return value))
fn low_angle_diff(x: f32, y: f32) -> f32 {
    // get the first possible difference in angle
    let diff1 = x - y;
    // get the second possible difference in angle
    let diff2 = (360.0 - maths::absf(x) - maths::absf(y))
        * maths::signumf(y);

    // return the smaller difference
    if maths::absf(diff2) < maths::absf(diff1) {
        diff2
    } else {
        diff1 // slight preference for diff 1 but still the same distance travelled
    }
}

/// Uses user joystick inputs to generate left and right drive voltages for the robot
pub fn user_control(
    j1x: f32,
    j1y: f32,
    j2x: f32,
    j2y: f32,
    yaw: f32,
    delta_seconds: f32,
    angle_integral: &mut f32,
) -> (i32, i32) {
    // log info

    // get the initial calculated voltages from the first controller
    let mut xv = magic::exp_daniel(j1x);
    let yv = magic::exp_daniel(j1y);

    info!("yv: {yv}");
    info!("xv: {xv}");

    // if the second joystick is active, then use the second joystick to derive an True Bearing target angle and correct for it
    if j2x != 0.0 || j2y != 0.0 {
        info!("using exact rotation (second joystick) for driver control's x");

        // get the target angle (from x and y) and correction x
        let target_angle = xy_to_angle(j2x, j2y);
        let correct_x = rot_correct(target_angle, yaw, delta_seconds, angle_integral);

        xv = correct_x;
    }

    // pass the final x y values through arcade drive for ldr rdr
    let (ldr, rdr) = arcade(xv as i32, yv as i32);

    info!("ldr: {ldr}, rdr: {rdr}");

    (ldr, rdr)
}

/// Corrects the rotation of the robot based upon the error (difference in) angle (-180..=180) and returns the new x value
pub fn rot_correct(
    target: f32,
    yaw: f32,
    delta_seconds: f32,
    integral: &mut f32
) -> f32 {
    let error = low_angle_diff(target, yaw);

    // use the pie
    let correct_x = pie::correct(
        low_angle_diff(target, yaw),
        180.0,
        delta_seconds,
        integral
    ) * 12000.0;

    // logs
    info!("yaw: {yaw}");
    info!("target angle: {target}");
    debug!("angle error: {error}");
    debug!("angle integral: {}", *integral);
    debug!("angle correction x: {correct_x}");

    // return the correction
    correct_x
}

/// Finds the angle of the x and y values of the joystick according to the top of the joystick
fn xy_to_angle(x: f32, y: f32) -> f32 {
    if y < 0.0 {
        maths::atan(maths::absf(y) / (x + 0.0001 * maths::signumf(x)))
            + 90.0 * maths::signumf(x)
    } else {
        maths::atan(x / (y + 0.0001 * maths::signumf(y)))
    }.clamp(-179.0, 179.0)
}
