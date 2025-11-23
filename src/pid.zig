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

    /// Updates the state of a PID, based on the current error and delta-time (in ms).
    /// 
    /// Upon goal change, the state should be wiped
    pub fn update(self: *State, params: Param, current_err: f64, dt: f64) f64 {
        // compute the low pass filter
        self.last_derv = (params.low_pass_a * self.last_derv) + (1-params.low_pass_a) * (current_err - self.last_err);

        const delta_time = dt / 1000.0;

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
            // drive.driveVolt(-12000 * @as(i32, @intFromFloat(math.sign(distance))), -12000 * @as(i32, @intFromFloat(math.sign(distance))), port_buffer);
            // pros.rtos.task_delay(40);
            // drive.driveVel(0, 0, port_buffer);
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

/// State-machine for moving to a certain coord in mm until a precision threshold is met (auton) with a PID
pub fn moveCoord(goal: odom.Coord, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    var rel_goal = goal - odom_state.coord;
    var direction = vector.polarToCartesian(1.0, odom_state.prev_yaw);
    var distance = vector.dotProduct(f64, rel_goal, direction);
    var goal_angle = math.radiansToDegrees(vector.calDir(f64, rel_goal));
    if (distance < 0) // if goal is behind then face it with the back of the robot
        goal_angle += 180;
    rotateDeg(goal_angle, odom_state, port_buffer); // goal angle relative to current coord // we'll try without it

    // state machine state
    var now = pros.rtos.millis();
    var pid = State{};
    while (true) {
        // update odom
        odom_state.update(port_buffer);

        // get the relative goal
        rel_goal = goal - odom_state.coord;
        goal_angle = math.radiansToDegrees(vector.calDir(f64, rel_goal));

        // get the current reachable distance (through dotproduct)
        direction = vector.polarToCartesian(1.0, odom_state.prev_yaw);
        distance = vector.dotProduct(f64, rel_goal, direction);

        // if it's within precision, break
        if (@abs(distance) < auton.precision_mm) {
            // stop driving
            drive.driveVel(0, 0, port_buffer);
            break;
        }

        // get speed from the PID
        const speed = pid.update(auton.mov_pid_param, distance, auton.cycle_delay);
        const drive_vel = @as(@Vector(2, f64), @splat(speed));

        // drive it
        drive.driveVel(drive_vel[0], drive_vel[1], port_buffer);

        // wait for the next cycle
        pros.rtos.task_delay_until(&now, auton.cycle_delay);
    }
}

/// State-machine for moving to a certain coord in mm until a precision threshold is met (auton) with a PID
pub fn moveCoordBoomer(goal: odom.Coord, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    var rel_goal = goal - odom_state.coord;
    var direction = vector.polarToCartesian(1.0, odom_state.prev_yaw);
    var distance = vector.dotProduct(f64, rel_goal, direction);
    var goal_angle = math.radiansToDegrees(vector.calDir(f64, rel_goal));
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
        goal_angle = math.radiansToDegrees(vector.calDir(f64, rel_goal));

        // get the current reachable distance (through dotproduct)
        direction = vector.polarToCartesian(1.0, odom_state.prev_yaw);
        distance = vector.dotProduct(f64, rel_goal, direction);

        // if it's within precision, break
        if (@abs(distance) < auton.precision_mm) {
            // stop driving
            drive.driveVel(0, 0, port_buffer);
            break;
        }

        // get the yaw and current angle error
        const yaw = odom.getYaw(port_buffer) orelse 0;
        const err = odom.minimalAngleDiff(yaw, goal_angle);

        // get speed from the PID
        const steer = pid.update(auton.yaw_pid_param, err, auton.cycle_delay);
        const speed = pid.update(auton.mov_pid_param, distance, auton.cycle_delay);
        const drive_vel = drive.arcadeDrive(steer, speed);

        // drive it
        drive.driveVel(drive_vel[0], drive_vel[1], port_buffer);

        // wait for the next cycle
        pros.rtos.task_delay_until(&now, auton.cycle_delay);
    }
}

/// State-machine for moving to a certain coord in mm until it passes it
pub fn moveChainCoord(goal: odom.Coord, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    var rel_goal = goal - odom_state.coord;
    var direction = vector.polarToCartesian(1.0, odom_state.prev_yaw);
    var distance = vector.dotProduct(f64, rel_goal, direction);
    var goal_angle = math.radiansToDegrees(vector.calDir(f64, goal - odom_state.coord));
        if (distance < 0) // if goal is behind then face it with the back of the robot
            goal_angle += 180;
        rotateDeg(goal_angle, odom_state, port_buffer); // goal angle relative to current coord

    // get the original distance sign
    const og_dist_sgn = math.sign(distance);

    // state machine state
    var now = pros.rtos.millis();
    while (true) {
        // update odom
        odom_state.update(port_buffer);

        // get the relative goal
        rel_goal = goal - odom_state.coord;

        // get the current reachable distance (through dotproduct)
        direction = vector.polarToCartesian(1.0, odom_state.prev_yaw);
        distance = vector.dotProduct(f64, rel_goal, direction);
        const distance_sgn = math.sign(distance);

        // if the distance is reached/passed (sign flipped)
        if (distance_sgn != og_dist_sgn) {
            // stop driving
            drive.driveVel(0, 0, port_buffer);
            break;
        }

        // do a constant speed
        const speed = distance_sgn * auton.auton_drive_speed; // in the right direction
        const drive_vel = @as(@Vector(2, f64), @splat(speed));

        // drive it
        drive.driveVel(drive_vel[0], drive_vel[1], port_buffer);

        // wait for the next cycle
        pros.rtos.task_delay_until(&now, auton.cycle_delay);
    }
}

/// Move to a coord with dual differential PIDs
pub fn moveCoordDDP(goal: odom.Coord, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    const rel_goal = goal - odom_state.coord;
    const direction = vector.polarToCartesian(1.0, odom_state.prev_yaw);
    const distance = vector.dotProduct(f64, rel_goal, direction);

    var goal_angle = vector.calDir(f64, rel_goal);
    if (distance < 0) // if goal is behind then face it with the back of the robot
        goal_angle += math.pi;

    // rotate towards the goal and move towards it
    rotateDegDDPTo(math.radiansToDegrees(goal_angle), odom_state, port_buffer);
    driveDDP(distance, distance, odom_state, port_buffer);
}

/// Rotates with dual differential PIDs in DEGREES to an absolute degrees
pub fn rotateDegDDPTo(angle: f64, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // get the yaw
    const yaw = odom.getYaw(port_buffer) orelse 0;
    // calculate the movement from the desired angle
    const diff = odom.minimalAngleDiff(yaw, math.degreesToRadians(angle));
    rotateDegDDP(math.radiansToDegrees(diff), odom_state, port_buffer);
}

/// Turn with dual differential PIDs in DEGREES
pub fn rotateDegDDP(angle: f64, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    const rads = math.degreesToRadians(angle);

    // calculate the turn
    const dist = rads * pure_pursuit.robot_width/2;

    driveDDP(dist, -dist, odom_state, port_buffer);
}

/// Drive with dual differential PIDs in mm
pub fn driveDDP(left_mm: f64, right_mm: f64, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // state machine state
    var now = pros.rtos.millis();
    var left_pid = State{};
    var right_pid = State{};
    const start_left = odom_state.left_dist;
    const start_right = odom_state.right_dist;

    while (true) {
        // update odom
        odom_state.update(port_buffer);

        const dist_left = left_mm - (odom_state.left_dist - start_left);
        const dist_right = right_mm - (odom_state.right_dist - start_right);

        // if it's within precision, break
        if (@abs(dist_left) < auton.precision_mm and @abs(dist_right) < auton.precision_mm) {
            // stop driving
            drive.driveVel(0, 0, port_buffer);
            break;
        }

        // get controls from pid
        const left_v = left_pid.update(auton.mov_pid_param, dist_left, auton.cycle_delay);
        const right_v = right_pid.update(auton.mov_pid_param, dist_right, auton.cycle_delay);

        // drive it
        drive.driveVel(left_v, right_v, port_buffer);

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
        if (@abs(err) < auton.precision_rad and @abs(odom_state.rot_vel) < math.degreesToRadians(1)) {
            // stop driving
            // drive.driveVolt(-12000 * @as(i32, @intFromFloat(math.sign(err))), 12000 * @as(i32, @intFromFloat(math.sign(err))), port_buffer);
            // pros.rtos.task_delay(40);
            // drive.driveVel(0, 0, port_buffer);
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

/// State-machine for rotating towards a yaw goal until it's been passed twice (osccilates) with a PID
pub fn rotateDegFastre(desired_yaw_deg: f64, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    const desired_yaw = math.degreesToRadians(desired_yaw_deg);
    // state machine state
    var now = pros.rtos.millis();
    var pid = State{};
    var passed = false;

    // get the yaw and current angle error
    var yaw = odom.getYaw(port_buffer) orelse 0;
    var err = odom.minimalAngleDiff(yaw, desired_yaw);
    const og_sign = err < 0;

    while (true) {
        // update odom
        odom_state.update(port_buffer);

        // get the yaw and current angle error
        yaw = odom.getYaw(port_buffer) orelse 0;
        err = odom.minimalAngleDiff(yaw, desired_yaw);

        // if it's within precision, break
        if (err == 0) {
            // stop driving
            drive.driveVel(0, 0, port_buffer);
            break;
        }

        // if it passes
        if (passed) {
            if (og_sign == (err < 0)) {
                // stop driving/break
                drive.driveVel(0, 0, port_buffer);
                break;
            }
        } else {
            if (og_sign != (err < 0))
                passed = true;
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
