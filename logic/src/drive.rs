//! Logic code for the drive-train of the robot

use crate::{debug, info, magic, pid::{self, PIDConsts, PIDState}};

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
    delta_seconds: f32,
    rot_pid_consts: &PIDConsts,
    rot_pid_state: &mut PIDState,
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

        // get correction x from yaw corrections
        xv = pid::update(yaw, target_angle, delta_seconds, rot_pid_state, rot_pid_consts, low_angle_diff);
    }

    // pass the ldr and rdr through arcade drive
    let (ldr, rdr) = arcade(xv as i32, yv as i32);

    info!("ldr: {ldr:06}, rdr: {rdr:06}");

    (ldr, rdr)
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

/// Corrects the rotation of the robot based upon the error (difference in) angle (-180..=180) and returns the x correction voltage
pub fn rot_correct(
    target: f32,
    yaw: f32,
    delta_seconds: f32,
    pid_consts: &PIDConsts,
    pid_state: &mut PIDState,
) -> f32 {
    info!("correcting for rotation/yaw");
    info!("yaw: {yaw:04.02}");
    info!("target angle: {target:04.02}");

    let correct_x = pid::update(yaw, target, delta_seconds, pid_state, pid_consts, low_angle_diff);

    debug!("angle correction x: {correct_x:08.02}");

    // return the correction
    correct_x
}

/// Corrects for the odometry's y coord (in mm) based upon the error (difference in) location and returns the y correction voltage
pub fn y_coord_correct(
    target: f32,
    coord: f32,
    delta_seconds: f32,
    pid_consts: &PIDConsts,
    pid_state: &mut PIDState,
) -> f32 {
    info!("correcting for odom y coordinate");
    info!("y coord: {coord}");
    info!("target y coord: {target}");

    let correct_y = pid::update(
        coord,
        target,
        delta_seconds,
        pid_state,
        pid_consts,
        |target, measure| target - measure,
    );

    debug!("y correction: {correct_y}");

    // return the correction
    correct_y
}

/// Finds the angle of the x and y values of the joystick according to the top of the joystick
pub fn xy_to_angle(x: f32, y: f32) -> f32 {
    if y < 0.0 {
        maths::atan(maths::absf(y) / (x + 0.0001 * maths::signumf(x)))
            + 90.0 * maths::signumf(x)
    } else {
        maths::atan(x / (y + 0.0001 * maths::signumf(y)))
    }.clamp(-179.0, 179.0)
}
