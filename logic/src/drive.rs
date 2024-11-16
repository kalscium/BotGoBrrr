//! Logic code for the drive-train of the robot

use crate::{debug, info, magic};

/// Performs an arcade drive transformation on x and y values (-12000..=12000) to produce left and right drive voltages
pub fn arcade(x: i32, y: i32) -> (i32, i32) {
    let ldr = (y + x).clamp(-12000, 12000);
    let rdr = (y - x).clamp(-12000, 12000);

    (ldr, rdr)
}


/// Finds the lowest difference in angle between two angles (-180..=180 (for both arguments and return value))
pub fn low_angle_diff(x: f32, y: f32) -> f32 {
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
    turning_mul: f32,
    j1x: f32,
    j1y: f32,
    j2x: f32,
    j2y: f32,
    yaw: f32,
    prev_vdr: &mut (i32, i32),
) -> (i32, i32) {
    // get the initial calculated voltages from the first controller
    let mut xv = magic::exp_daniel(j1x) * turning_mul;
    let yv = magic::exp_daniel(j1y);

    info!("xv: {xv:08.02}, yv/thrust: {yv:08.02}");

    // if the second joystick is active, then use the second joystick to derive an True Bearing target angle and correct for it
    if j2x != 0.0 || j2y != 0.0 {
        info!("using exact rotation (second joystick) for driver control's x");

        // get the target angle (from x and y) and correction x
        let target_angle = xy_to_angle(j2x, j2y);
        let correct_x = rot_correct(target_angle, yaw);

        xv = correct_x;
    }

    // pass the ldr and rdr through arcade drive
    let (ldr, rdr) = arcade(xv as i32, yv as i32);

    // pass the ldr and rdr through a voltage dampener
    let (ldr, rdr) = damp_volts((ldr, rdr), prev_vdr);

    info!("ldr: {ldr:06}, rdr: {rdr:06}");

    (ldr, rdr)
}

/// Uses thrust, target angle yaw, and initial yaw to generate left and right drives
pub fn inst_control(
    target_angle: f32,
    yaw: f32,
    prev_vdr: &mut (i32, i32),
) -> (i32, i32) {
    // corrects for the rotation
    let cx = rot_correct(target_angle, yaw);

    // passes the x and y values through arcade drive
    let (ldr, rdr) = arcade(cx as i32, 0);

    // dampes the ldr and rdr
    let (ldr, rdr) = damp_volts((ldr, rdr), prev_vdr);

    info!("ldr: {ldr:06}, rdr: {rdr:06}");

    (ldr, rdr)
}

/// Corrects for any errors (delta) based upon it's inital error (delta) and returns the correction voltage
pub fn correct_volt(error: f32, max_error: f32) -> f32 {
    magic::exp_daniel(error / max_error * 1.2)
        .clamp(-6000., 6000.)
    // 6000.0 * maths::signumf(error)
}

/// Dampens any sudden changes to the voltage drives of the robot
pub fn damp_volts(new_vdr: (i32, i32), prev_vdr: &mut (i32, i32)) -> (i32, i32) {
    /// The percentage used to determine the linear interpolation
    const LERP_CONST: f32 = 0.16;

    // linearly interpolate between the two based upon a constant
    let ldr_lerp = maths::lerp(prev_vdr.0 as f32, new_vdr.0 as f32, LERP_CONST);
    let rdr_lerp = maths::lerp(prev_vdr.1 as f32, new_vdr.1 as f32, LERP_CONST);

    // find the minimum votlage between the two
    let vdr = (
        new_vdr.0.abs().min((ldr_lerp as i32).abs()) * new_vdr.0.signum(),
        new_vdr.1.abs().min((rdr_lerp as i32).abs()) * new_vdr.1.signum(),
    );

    // logs
    info!("prev ldr: {:06}, prev rdr: {:06}", prev_vdr.0, prev_vdr.1);
    info!("raw ldr: {:06}, raw rdr: {:06}", new_vdr.0, new_vdr.1);

    // update the previous voltage drive
    *prev_vdr = vdr.clone();

    // reutrn the minimum voltage between the two
    vdr
}

/// Corrects the rotation of the robot based upon the error (difference in) angle (-180..=180) and returns the new x value
pub fn rot_correct(target: f32, yaw: f32) -> f32 {
    /// The point in which, the error is so large that the robot runs at full speed
    const MAX_SPEED_THRESHOLD: f32 = 20.;

    let error = low_angle_diff(target, yaw);
    let correct_x = correct_volt(error, MAX_SPEED_THRESHOLD);

    // logs
    info!("yaw: {yaw:04.02}");
    info!("target angle: {target:04.02}");
    debug!("angle error: {error:04.02}");
    debug!("angle correction x: {correct_x:08.02}");

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
