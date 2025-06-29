//! Functions for using, graphing, and tuning a hand-rolled PID

const math = @import("std").math;

const pros = @import("pros");

const auton = @import("autonomous.zig");
const odom = @import("odom.zig");
const port = @import("port.zig");
const vector = @import("vector.zig");
const drive = @import("drive.zig");

/// The parameters of a PID
pub const Param = struct {
    kp: f64,
    ki: f64,
    kd: f64,
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
        // u(t) = kp*e(t) + ki*integral e(t)dt + kd*de/dt
        const output =
            (params.kp * current_err) +
            (params.ki * self.integral) +
            (params.kd * derivative);

        // update last error
        self.last_err = current_err;

        // clamp the output
        return math.clamp(output, -params.saturation, params.saturation);
    }
};

/// State-machine for moving towards a movement goal until a precision threshold is met (auton) with a PID
pub fn move(desired_coord: odom.Coord, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // state machine state
    var now = pros.rtos.millis();
    var pid = State{};
    while (true) {
        // update odom
        odom_state.update(port_buffer);

        // get the yaw
        const yaw = odom.getYaw(port_buffer) orelse 0;

        // get the current reachable distance (through dotproduct)
        const displacement = desired_coord - odom.coord;
        const distance = vector.dotProduct(f64, displacement, vector.polarToCartesian(1, yaw));

        // if it's within precision, break
        if (distance < auton.precision_mm)
            break;

        // get controls from pid
        const y = pid.update(auton.mov_pid_param, distance, auton.tick_delay);

        // drive it
        drive.driveLeft(y, port_buffer);
        drive.driveRight(y, port_buffer);

        // wait for the next cycle
        pros.rtos.task_delay_until(&now, auton.tick_delay);
    }
}

/// State-machine for rotating towards a yaw goal until a precision threshold is met (auton) with a PID
pub fn rotate(desired_yaw: f64, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // state machine state
    var now = pros.rtos.millis();
    var pid = State{};
    while (true) {
        // update odom
        odom_state.update(port_buffer);

        // get the yaw and current angle error
        const yaw = odom.getYaw(port_buffer) orelse 0;
        const err = odom.minimalAngleDiff(yaw, desired_yaw);

        // if it's within precision, break
        if (err < auton.precision_rad)
            break;

        // get controls from pid
        const x = pid.update(auton.yaw_pid_param, yaw, auton.tick_delay);

        // drive it
        const ldr, const rdr = drive.arcadeDrive(x, 0);
        drive.driveLeft(ldr, port_buffer);
        drive.driveRight(rdr, port_buffer);

        // wait for the next cycle
        pros.rtos.task_delay_until(&now, auton.tick_delay);
    }
}
