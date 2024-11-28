//! A simple PID implementation

use crate::info;

/// PID Controller Constants
#[derive(Debug, Clone)]
pub struct PIDConsts {
    /// The proportional gain
    pub kp: f32,
    /// The integral gain
    pub ki: f32,

    /// The saturation point (output limit) (must be POSITIVE)
    pub saturation: f32,
}

/// A state of the PID controller
#[derive(Debug, Clone, Default)]
pub struct PIDState {
    /// The current integral of the PID controller
    pub integral: f32,
    /// The previous measurement
    pub prev_measure: f32,
    /// The previous velocity
    pub prev_velocity: f32,
    /// The previous acceleration
    pub prev_accel: f32,
}

/// Updates the state of the PID based upon the current target and measurement
pub fn update(
    measurement: f32,
    target: f32,
    delta_seconds: f32,
    state: &mut PIDState,
    consts: &PIDConsts,
    diff: impl Fn(f32, f32) -> f32,
) -> f32 {
    // calculate the velocity, acceleration and jerk
    let velocity = diff(measurement, state.prev_measure) / delta_seconds;
    let accel = velocity - state.prev_velocity / delta_seconds;
    let jerk = accel - state.prev_accel / delta_seconds;

    // update the previous measurement, velocity and acceleration
    state.prev_measure = measurement;
    state.prev_velocity = velocity;
    state.prev_accel = accel;

    // predict the future measurement based on the robot's velocity, acceleration and jerk
    let future_accel = accel + jerk * delta_seconds;
    let future_vel = velocity + future_accel * delta_seconds;
    let measurement = measurement + future_vel * delta_seconds;

    // find the error
    let error = diff(target, measurement);

    // find the proportional correction
    let pc = consts.kp * error;

    // clamp the proportional corerction
    let pc = pc.clamp(-consts.saturation, consts.saturation);

    // find the integral correction
    let ic = state.integral * consts.kp;

    // integrate with dynamic anti-windup
    let limit_inte = consts.saturation - maths::absf(pc);
    if
        maths::absf(ic) < limit_inte
        || state.integral.is_sign_positive() ^ error.is_sign_positive()
    {
        state.integral += consts.ki * delta_seconds * error;
    }

    // compute the output and apply the limits
    let output = (pc + ic).clamp(-consts.saturation, consts.saturation);

    info!("error: {error}");
    info!("pid proportional: {pc}");
    info!("pid integrator: {}", state.integral);
    info!("pid integral: {}", ic);
    info!("pid out: {output}");
    
    // return the output
    output
}
