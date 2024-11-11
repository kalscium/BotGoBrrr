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
    j1x: f32,
    j1y: f32,
    j2x: f32,
    j2y: f32,
    yaw: f32,
    initial_yaw: &mut f32,
    prev_vdr: &mut (i32, i32),
) -> (i32, i32) {
    // get the initial calculated voltages from the first controller
    let mut xv = magic::exp_daniel(j1x);
    let yv = magic::exp_daniel(j1y);

    info!("xv: {xv:08.02}, yv/thrust: {yv:08.02}");

    // if the second joystick is active, then use the second joystick to derive an True Bearing target angle and correct for it
    if j2x != 0.0 || j2y != 0.0 {
        info!("using exact rotation (second joystick) for driver control's x");

        // get the target angle (from x and y) and correction x
        let target_angle = xy_to_angle(j2x, j2y);
        let correct_x = rot_correct(target_angle, yaw, *initial_yaw);

        xv = correct_x;
    } else {
        // update the initial yaw
        *initial_yaw = yaw;
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
    thrust: i32,
    target_angle: f32,
    yaw: f32,
    initial_yaw: f32,
    prev_vdr: &mut (i32, i32),
) -> (i32, i32) {
    info!("thrust: {thrust:04}");

    // corrects for the rotation
    let cx = rot_correct(target_angle, yaw, initial_yaw);

    // passes the x and y values through arcade drive
    let (ldr, rdr) = arcade(cx as i32, thrust);

    // dampens the ldr and rdr
    let (ldr, rdr) = damp_volts((ldr, rdr), prev_vdr);

    info!("ldr: {ldr:06}, rdr: {rdr:06}");

    (ldr, rdr)
}

/// Corrects for any errors (delta) based upon it's inital error (delta) and returns the correction voltage
pub fn correct_volt(error: f32, initial_error: f32) -> f32 {
    magic::exp_ethan(
        (maths::checked_div(error, initial_error) // get the fraction of the delta (location in the slope)
            .unwrap_or(0.) // if no delta, do nothing
            * maths::signumf(error)) // keep the sign of the delta
        .clamp(-1., 1.)
    )
}

/// Dampens any sudden changes to the voltage drives of the robot
pub fn damp_volts(new_vdr: (i32, i32), prev_vdr: &mut (i32, i32)) -> (i32, i32) {
    // find the deltas of ldr and rdr
    let ldr_delta = new_vdr.0 - prev_vdr.0;
    let rdr_delta = new_vdr.1 - prev_vdr.1;

    // find the exponential derivative for ldr and rdr
    const MAX_CHANGE: f32 = 24000.;
    const FRICTION_MUL: f32 = 0.32;
    let exdr_ldr = magic::log_ethan(ldr_delta as f32 * FRICTION_MUL / MAX_CHANGE);
    let exdr_rdr = magic::log_ethan(rdr_delta as f32 * FRICTION_MUL / MAX_CHANGE);

    // get the dampened voltage drive
    let damp_vdr = (
        new_vdr.0 - exdr_ldr as i32,
        new_vdr.1 - exdr_rdr as i32,
    );

    // logs
    debug!("raw ldr: {:06}, raw rdr: {:06}", new_vdr.0, new_vdr.1);
    debug!("prev ldr: {:06}, prev rdr: {:06}", prev_vdr.0, prev_vdr.1);
    debug!("ldr delta: {ldr_delta:06}, rdr delta: {rdr_delta:06}");
    debug!("exdr_ldr: {exdr_ldr:08.02}, exdr_rdr: {exdr_rdr:08.02}");

    // update the previous voltage drive
    *prev_vdr = damp_vdr.clone();

    // return the dampened voltage drives
    damp_vdr
}

/// Corrects the rotation of the robot based upon the error (difference in) angle (-180..=180) and returns the new x value
pub fn rot_correct(
    target: f32,
    yaw: f32,
    initial_yaw: f32,
) -> f32 {
    let error = low_angle_diff(target, yaw);
    let initial_error = low_angle_diff(target, initial_yaw);
    let correct_x = correct_volt(error, initial_error);

    // logs
    info!("yaw: {yaw:04.02}");
    info!("target angle: {target:04.02}");
    debug!("initial angle error: {initial_error:04.02}");
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
