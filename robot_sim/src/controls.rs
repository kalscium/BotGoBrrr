use bevy::prelude::Resource;
use logic::{info, pid::{self, PIDState, PIDConsts}};
use rand::Rng;

/// State for the controls
///
/// Don't do this, this is just so that I can rapidly test new controls without modifing the game code
#[derive(Resource)]
pub struct ControlState {
    pid: PIDState,
}

/// Initialises the control state
pub fn init_state() -> ControlState {
    ControlState {
        pid: PIDState::default(),
    }
}

pub fn controls(x: f32, y: f32, delta_second: f32, yaw: f32, state: &mut ControlState) -> (i32, i32) {
    // pick either driving method until i get a controller with two joysticks // pure_driver(x, y, yaw, delta_second, state)
    abs_rotation(x, y, yaw, delta_second, state)
}

const TURNING_MUL: f32 = 0.64;

/// A form of control that doesn't use the inertial sensor and is pure driver-control
pub fn pure_driver(x: f32, y: f32, yaw: f32, _delta_second: f32, state: &mut ControlState) -> (i32, i32) {
    info!("# Only driver control\n");
    todo!()
}

/// A form of control that rotates the robot in an absolute way
pub fn abs_rotation(x: f32, y: f32, yaw: f32, delta_seconds: f32, state: &mut ControlState) -> (i32, i32) {
    logic::info!("# IMU exact rotation");

    const MAX_ERR: f32 = 45.; // the maximum error before saturation
    const PID: PIDConsts = PIDConsts {
        kp: 1. / MAX_ERR * 12000.0,
        ki: 16., // decrease until ocillations reduce enough
        prediction_window: 0., // would be a non-zero value in an environment with physics
        saturation: 12000.,
    };

    let target = logic::drive::xy_to_angle(x, y + 0.0001);

    let correct_x = pid::update(
        yaw,
        target,
        delta_seconds,
        &mut state.pid,
        &PID,
        logic::drive::low_angle_diff,
    );

    let (ldr, rdr) = logic::drive::arcade(correct_x as i32, 0);

    (ldr, rdr)
}

/// A function that controls the noise of the left and right drives of the robot (from -1..=1)
pub fn noise(ldr: f32, rdr: f32) -> (f32, f32) {    
    // add random noise
    // let ldr = ldr + rand::thread_rng().gen_range(-100..100) as f32 * 0.1;
    // let rdr = rdr + rand::thread_rng().gen_range(-100..100) as f32 * 0.1;
    let rdr = rdr * 0.9;

    (ldr, rdr)
}
