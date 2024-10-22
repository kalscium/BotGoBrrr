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
    pid: drive_controls::pid::Pid<f32>,
}

/// Initialises the control state
pub fn init_state() -> ControlState {
    ControlState {
        pid: drive_controls::new_pid(),
    }
}

pub fn controls(x: f32, y: f32, _delta_seconds: f32, yaw: f32, state: &mut ControlState) -> (i32, i32, Vec<String>) {
    // pick either driving method until i get a controller with two joysticks
    // pure_driver(x, y, yaw)
    abs_rotation(x, y, yaw, state)
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

    debug!(debug_info: "joystick x: {x}");
    debug!(debug_info: "joystick y: {y}");
    debug!(debug_info: "joyvolt  x: {xv}");
    debug!(debug_info: "joyvolt  y: {yv}\n");

    debug!(debug_info: "yaw       : {yaw}\n");

    debug!(debug_info: "(ldr, rdr): ({ldr}, {rdr})");
    
    // return them
    (ldr, rdr, debug_info)
}

/// A form of control that rotates the robot in an absolute way
pub fn abs_rotation(x: f32, y: f32, yaw: f32, state: &mut ControlState) -> (i32, i32, Vec<String>) {
    let mut debug_info = Vec::new();

    // get the angle desired angle (from x and y)
    let desired_angle = drive_controls::xy_to_angle(x, y);

    // get the correction x
    let xc = drive_controls::rot_correct(desired_angle, yaw, &mut state.pid);

    // get the final left and right drive voltages
    let (ldr, rdr) = drive_controls::arcade(xc as i32, 0);

    // print the debug information
    debug!(debug_info: "# IMU exact rotation\n");

    debug!(debug_info: "joystick x: {x}");
    debug!(debug_info: "joystick y: {y}\n");
    debug!(debug_info: "yaw       : {yaw}");
    debug!(debug_info: "desired   : {desired_angle}");
    debug!(debug_info: "correct  x: {xc}\n");

    debug!(debug_info: "(ldr, rdr): ({ldr}, {rdr})");

    // return them
    (ldr, rdr, debug_info)
}

/// A function that controls the noise of the left and right drives of the robot (from -1..=1)
pub fn noise(ldr: f32, rdr: f32) -> (f32, f32) {    
    // add random noise
    // let ldr = ldr + rand::thread_rng().gen_range(-100..100) as f32 * 0.01;
    // let rdr = rdr + rand::thread_rng().gen_range(-100..100) as f32 * 0.01;
    let rdr = rdr * 0.8;

    (ldr, rdr)
}
