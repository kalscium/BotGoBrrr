use bevy::prelude::Resource;
use rand::Rng;

/// A macro for printing debug information
macro_rules! debug {
    ($debug_info:ident: $($tt:tt)*) => {
        $debug_info.push(format!($($tt)*));
    };
}

/// State for the controls
///
/// Don't do this, this is just so that I can rapidly test new controls without modifing the game code
#[derive(Resource)]
pub struct ControlState {
    integral: f32,
}

/// Initialises the control state
pub fn init_state() -> ControlState {
    ControlState {
        integral: 0.0,
    }
}

pub fn controls(x: f32, y: f32, delta_seconds: f32, yaw: f32, state: &mut ControlState) -> (i32, i32, Vec<String>) {
    // pick either driving method until i get a controller with two joysticks
    // pure_driver(x, y, yaw)
    abs_rotation(x, y, yaw, delta_seconds, state)
}

const TURNING_MUL: f32 = 0.64;

/// A form of control that doesn't use the inertial sensor and is pure driver-control
pub fn pure_driver(x: f32, y: f32, yaw: f32) -> (i32, i32, Vec<String>) {
    let mut debug_info = Vec::new();

    // get the voltage values
    let xv = drive_controls::exp_daniel(x) * TURNING_MUL;
    let yv = drive_controls::exp_daniel(y);

    // get the final left and right drive voltages
    let (ldr, rdr) = drive_controls::arcade(xv as i32, yv as i32);

    // print the debug information
    debug!(debug_info: "# Only driver control\n");

    debug!(debug_info: "joystick x: {x:.4}");
    debug!(debug_info: "joystick y: {y:.4}");
    debug!(debug_info: "joyvolt  x: {xv:.4}");
    debug!(debug_info: "joyvolt  y: {yv:.4}\n");

    debug!(debug_info: "yaw       : {yaw:.4}\n");

    debug!(debug_info: "(ldr, rdr): ({ldr}, {rdr})");
    
    // return them
    (ldr, rdr, debug_info)
}

/// A form of control that rotates the robot in an absolute way
pub fn abs_rotation(x: f32, y: f32, yaw: f32, delta_seconds: f32, state: &mut ControlState) -> (i32, i32, Vec<String>) {
    let mut debug_info = Vec::new();

    // get the target angle (from x and y)
    let target_angle = drive_controls::xy_to_angle(x, y);

    // get the error angle and the correction x
    let diff = drive_controls::low_angle_diff(target_angle, yaw);
    let xc = drive_controls::rot_correct(diff, delta_seconds, &mut state.integral);

    // get the final left and right drive voltages
    let (ldr, rdr) = drive_controls::arcade(xc as i32, 0);

    // print the debug information
    debug!(debug_info: "# IMU exact rotation\n");

    debug!(debug_info: "joystick x: {x:.4}");
    debug!(debug_info: "joystick y: {y:.4}\n");

    debug!(debug_info: "yaw       : {yaw:.4}");
    debug!(debug_info: "target    : {target_angle:.4}");
    debug!(debug_info: "correct  x: {xc:.4}\n");

    debug!(debug_info: "error     : {diff:.4}");
    debug!(debug_info: "integral  : {:.4}", state.integral);

    debug!(debug_info: "(ldr, rdr): ({ldr}, {rdr})");

    // return them
    (ldr, rdr, debug_info)
}

/// A function that controls the noise of the left and right drives of the robot (from -1..=1)
pub fn noise(ldr: f32, rdr: f32) -> (f32, f32) {    
    // add random noise
    let ldr = ldr + rand::thread_rng().gen_range(-100..100) as f32 * 0.01;
    let rdr = rdr + rand::thread_rng().gen_range(-100..100) as f32 * 0.01;
    // let rdr = rdr * 0.8;

    (ldr, rdr)
}
