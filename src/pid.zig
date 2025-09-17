//! Functions for using, graphing, and tuning a hand-rolled PID

const math = @import("std").math;

const pros = @import("pros");

const auton = @import("autonomous.zig");
const odom = @import("odom.zig");
const port = @import("port.zig");
const vector = @import("vector.zig");
const drive = @import("drive.zig");
const pure_pursuit = @import("pure_pursuit.zig");

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

/// State-machine for moving a certain distance in mm until a precision threshold is met (auton) with a PID
pub fn moveMM(goal_distance: f64, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // state machine state
    var now = pros.rtos.millis();
    var pid = State{};
    const direction = vector.polarToCartesian(1, odom_state.prev_yaw);
    const start_pos = vector.dotProduct(f64, odom_state.coord, direction);
    while (true) {
        // update odom
        odom_state.update(port_buffer);

        // get the current reachable distance (through dotproduct)
        const distance_travelled = vector.dotProduct(f64, odom_state.coord, direction) - start_pos;
        const distance = goal_distance - distance_travelled;

        // if it's within precision, break
        if (@abs(distance) < auton.precision_mm) {
            // stop driving
            drive.driveVel(0, 0, port_buffer);
            break;
        }

        // get controls from pid
        const y = pid.update(auton.mov_pid_param, distance, auton.cycle_delay);

        // drive it
        drive.driveVel(y, y, port_buffer);

        // wait for the next cycle
        pros.rtos.task_delay_until(&now, auton.cycle_delay);
    }
}

/// State-machine for moving a certain distance in mm until a precision threshold is met (auton) with a PID
pub fn moveCoord(goal: odom.Coord, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // rotate AOT towards the goal coord (no yaw updates whilst moving)
    // (but to also correct any accumulative yaw errors)
    var rel_goal = goal - odom_state.coord;
    var direction = vector.polarToCartesian(1.0, odom_state.prev_yaw);
    var distance = vector.dotProduct(f64, rel_goal, direction);
    var goal_angle = math.radiansToDegrees(vector.calDir(f64, goal - odom_state.coord));
    if (distance < 0) // if goal is behind then face it with the back of the robot
        goal_angle += 180;
    rotateDeg(goal_angle, odom_state, port_buffer); // goal angle relative to current coord

    // state machine state
    var now = pros.rtos.millis();
    var pid = State{};
    while (true) {
        // update odom
        odom_state.update(port_buffer);

        // get the relative goal
        rel_goal = goal - odom_state.coord;

        // get the current reachable distance (through dotproduct)
        direction = vector.polarToCartesian(1.0, odom_state.prev_yaw);
        distance = vector.dotProduct(f64, rel_goal, direction);

        // if it's within precision, break
        if (@abs(distance) < auton.precision_mm) {
            // stop driving
            drive.driveVel(0, 0, port_buffer);
            break;
        }

        // get the ratios from pure pursuit
        // var drive_vel = pure_pursuit.trueFollowArc(odom_state.coord, goal, odom_state.prev_yaw);

        // get speed from the PID
        const speed = pid.update(auton.mov_pid_param, distance, auton.cycle_delay);
        // drive_vel *= @splat(speed);

        // drive it
        drive.driveVel(speed, speed, port_buffer);

        // wait for the next cycle
        pros.rtos.task_delay_until(&now, auton.cycle_delay);
    }
}

/// State-machine for rotating towards a yaw goal until a precision threshold is met (auton) with a PID
pub fn rotateDeg(desired_yaw_deg: f64, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    const desired_yaw = math.degreesToRadians(desired_yaw_deg);
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
        if (@abs(err) < auton.precision_rad) {
            _ = pros.printf("err: %lf, precision: %lf\n", err, auton.precision_rad);
            // stop driving
            drive.driveVel(0, 0, port_buffer);
            break;
        }

        // get controls from pid
        const x = pid.update(auton.yaw_pid_param, err, auton.cycle_delay);

        // drive it
        const ldr, const rdr = drive.arcadeDrive(x, 0);
        drive.driveVel(ldr, rdr, port_buffer);

        // wait for the next cycle
        pros.rtos.task_delay_until(&now, auton.cycle_delay);
    }
}
