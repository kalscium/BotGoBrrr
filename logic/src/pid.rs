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

    /// The rate of linear interpolation throuogh the derivative components
    pub derive_lerp: f32,

    /// The saturation point (output limit) (must be POSITIVE)
    pub saturation: f32,
}

/// A state of the PID controller
#[derive(Debug, Clone, Default)]
pub struct PIDState {
    /// The current integral of the PID controller
    pub integral: f32,
    /// The previous error (required for the integral)
    pub prev_error: f32,
    /// The previous measurement (required for the derivative)
    pub prev_measure: f32,
    /// The previous velocity (required for the derivative)
    pub avg_velocity: f32,
    /// The previous accelleration (required for the derivative)
    pub avg_accel: f32,
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

    // clamp the proportional corerction
    let prop = prop.clamp(-consts.saturation, consts.saturation);

    // find the integral correction
    state.integral += consts.ki * delta_seconds * error;

    // clamp the integral
    let limit_int = consts.saturation - prop;
    state.integral = state.integral.clamp(-limit_int, limit_int);

    // calculate the 1st, 2nd and 3rd order derivatives
    let velocity = diff(measurement, state.prev_measure) / delta_seconds;
    let acceleration = diff(velocity, state.avg_velocity) / delta_seconds;
    let jerk = diff(acceleration, state.avg_accel) / delta_seconds;

    // find the derivative correction
    let derivative = -(consts.kd * jerk);

    // clamp the derivative correction
    let derivative = derivative.clamp(-maths::absf(prop), maths::absf(prop));

    // update the avg derivatives with anti-windup
    state.prev_error = error;
    state.prev_measure = measurement;
    state.avg_velocity = maths::lerp(state.avg_velocity, velocity, (consts.derive_lerp * delta_seconds).clamp(0., 1.));
    state.avg_accel = maths::lerp(state.avg_accel, acceleration, (consts.derive_lerp * delta_seconds).clamp(0., 1.));

    // compute the output and apply the limits
    let output = (prop + state.integral + derivative).clamp(-consts.saturation, consts.saturation);

    info!("error: {error}");
    info!("pid proportional: {prop}");
    info!("pid integral: {}", state.integral);
    info!("pid prev_measure: {}", state.prev_measure);
    info!("pid avg_velocity: {}", state.avg_velocity);
    info!("pid avg_accel: {}", state.avg_accel);
    info!("pid jerk: {}", jerk);
    info!("pid derivative: {}", derivative);
    info!("pid out: {output}");
    
    // return the output
    output
}
