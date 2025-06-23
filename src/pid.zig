//! Functions for using, graphing, and tuning a hand-rolled PID

/// The parameters of a PID
pub const Param = struct {
    kp: f64,
    ki: f64,
    fd: f64,
    // the max plant output value physically achievable (for motor voltage it's 12000mV)
    saturation: f64,
    // the low pass filter measurement gain (0-1) (higher is lower change in error (stronger filter)) (for the D term) (a good value is 0.8)
    low_pass_a: f64,
};

/// The state of a PID
pub const State = struct {
    last_err: f64 = 0,
    integral: f64 = 0,
    /// The last derivative
    last_derv: f64 = 0,

    /// Updates the state of a PID, based on the current error and delta-time.
    /// 
    /// Upon goal change, the state should be wiped
    pub fn update(self: *State, params: Param, current_err: f64, delta_time: f64) f64 {
        // compute the low pass filter
        self.last_derv = (params.low_pass_a * self.last_derv) + (1-params.low_pass_a) * (current_err - self.last_err);

        // compute the derivative
        const derivative = self.last_derv / delta_time;

        // compute the integral (with clamped windup)
        if (@abs(self.integral * params.ki) <= params.saturation)
            self.integral += current_err * delta_time;

        // compute the PID output
        const output =
            (params.kp * current_err) +
            (params.ki * self.integral) +
            (params.kd * derivative);

        // update last error
        self.last_err = current_err;

        return @min(output, params.saturation); // clamp it
    }
};
