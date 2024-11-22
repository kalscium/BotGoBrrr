//! A simple PID implementation

use crate::info;

/// PID Controller Constants
#[derive(Debug, Clone)]
pub struct PIDConsts {
    /// The proportional gain
    pub kp: f32,
    /// The integral gain
    pub ki: f32,
    /// The derivative gain
    pub kd: f32,

    /// Derivative low-pass filter time constant
    pub tau: f32,

    /// The minimum output limit
    pub limit_min: f32,
    /// The maximum output limit
    pub limit_max: f32,
}

/// A state of the PID controller
#[derive(Debug, Clone, Default)]
pub struct PIDState {
    /// The current integral of the PID controller
    pub integral: f32,
    /// The previous error (required for the integral)
    pub prev_error: f32,
    /// The current derivative of the PID controller
    pub derivative: f32,
    /// The previous measurement (required for the derivative)
    pub prev_measure: f32,
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
    // find the error
    let error = diff(target, measurement);

    // find the proportional correction
    let prop = consts.kp * error;

    // find the integral correction
    state.integral += 0.5 * consts.ki * delta_seconds * (error + state.prev_error);

    // anti-windup via dynamic intergrator clamping

    // set the upper bounds for the integral
    let limit_max_int = if consts.limit_max > prop {
        consts.limit_max - prop
    } else {
        0.
    };

    // set the lower bounds for the integral
    let limit_min_int = if consts.limit_min < prop {
        consts.limit_min - prop
    } else {
        0.
    };

    // clamp the integral
    state.integral = state.integral.clamp(limit_min_int, limit_max_int);

    // derive the derivative (band-limited differentiator)
    state.derivative = (2. * consts.kd * diff(measurement, state.prev_measure)) // note: derivative on measurement!
                     + (2. * consts.tau - delta_seconds) * state.derivative
                     / (2. * consts.tau + delta_seconds);

    // compute the output and apply the limits
    let out = (prop + state.integral + state.derivative).clamp(consts.limit_min, consts.limit_max);

    info!("error: {error}");
    info!("pid proportional: {prop}");
    info!("pid integral: {}", state.integral);
    info!("pid derivative: {}", state.derivative);
    info!("pid out: {out}");

    // update the error and previous measurement
    state.prev_measure = measurement;
    state.prev_error = error;
    
    // return the output
    out
}
