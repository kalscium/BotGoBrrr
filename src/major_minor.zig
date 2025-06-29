//! Functions for using my very own 'Major Minor Control'

const std = @import("std");
const pros = @import("pros");
const vector = @import("vector.zig");
const odom = @import("odom.zig");
const port = @import("port.zig");
const auton = @import("autonomous.zig");
const drive = @import("drive.zig");

/// The rate of deceleration assuming constant friction, coefficient of drag, and a flat surface
/// 
/// `F=ma`
/// `Friction = coefficient of drag * normal force`
/// `Normal Force = mg`
/// therefore, `a` should also be constant
/// 
/// *Used for **Major** control
/// 
/// note: should be negative as it's decelerating not accelerating
pub const coast_rate = struct{
    /// The coast rate of movement (Y)
    pub const mov = 0; // not found yet
    /// The coast rate of rotating (yaw)
    pub const yaw = 0; // not found yet
};

/// The tuned integral (ki) for minor controllers
pub const integrals = struct{
    /// The integral for the minor movement controller
    /// 
    /// Found by finding the ki value just large enough for minor
    /// control to correct a 10cm movement error whilst turning 45°
    pub const mov = 0;

    ///  The integral for the minor yaw controller
    /// 
    /// Found by finding the ki value just large enough for minor
    /// control to correct a 20° yaw error whilst moving one field tile
    pub const yaw = 0;
};

/// Does a major movement whilst correcting for a minor yaw error
pub fn move(desired_coord: odom.Coord, desired_yaw: f64, reverse: bool, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // state machine state
    var now = pros.rtos.millis();
    var integral: f64 = 0;
    while (true) {
        // update odom
        odom_state.update(port_buffer);

        // get the yaw of the robot
        const yaw = odom.getYaw(port_buffer) orelse 0;

        // calculate coasting distance
        const displacement = desired_coord - odom_state.coord;
        const distance = vector.dotProduct(f64, displacement, vector.polarToCartesian(1, yaw));
        // calculate coast distance `s = (v^2 - u^2)/(2a)`
        const coast_displacement = -@exp2(odom_state.mov_vel) / (2 * coast_rate.mov);
        const coasting_distance = distance - coast_displacement;

        // calculate yaw error
        const yaw_err = odom.minimalAngleDiff(yaw, desired_yaw);
        // calculate the integral (with anti-windup)
        if (@abs(integral * integrals.yaw) <= 1)
            integral += yaw_err * auton.tick_delay;

        // get the x (yaw) & y (movement) values
        var y: f64 = if (std.math.signbit(coasting_distance) != reverse) 0 else 1; // note: the XOR checks for overshoot
        if (reverse)
            y *= -1;
        const x: f64 = integral * integrals.yaw;
        const ldr, const rdr = drive.arcadeDrive(std.math.clamp(x, -1, 1), std.math.clamp(y, -1, 1));
        
        // drive
        drive.driveLeft(ldr, port_buffer);
        drive.driveRight(rdr, port_buffer);

        // if the robot has stopped moving and the distance has coasted break
        if (odom_state.mov_vel == 0 and std.math.signbit(coasting_distance) != reverse) // note: the XOR checks for overshoot
            break;

        // wait for next cycle
        pros.rtos.task_delay_until(&now, auton.tick_delay);
    }
}

/// Does a major movement whilst correcting for a minor yaw error
pub fn rotate(desired_yaw: f64, desired_coord: odom.Coord, reverse: bool, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // state machine state
    var now = pros.rtos.millis();
    var integral: f64 = 0;
    while (true) {
        // update odom
        odom_state.update(port_buffer);

        // get the yaw of the robot
        const yaw = odom.getYaw(port_buffer) orelse 0;

        // calculate coasting distance
        const distance = odom.minimalAngleDiff(yaw, desired_yaw);
        // calculate coast distance `s = (v^2 - u^2)/(2a)`
        const coast_displacement = -@exp2(odom_state.rot_vel) / (2 * coast_rate.yaw);
        const coasting_distance = odom.minimalAngleDiff(coast_displacement, distance);

        // calculate movement error
        const displacement = desired_coord - odom_state.coord;
        const mov_err = vector.dotProduct(f64, displacement, vector.polarToCartesian(1, yaw));
        // calculate the integral (with anti-windup)
        if (@abs(integral * integrals.mov) <= 1)
            integral += mov_err * auton.tick_delay;

        // get the x (yaw) & y (movement) values
        var x: f64 = if (std.math.signbit(coasting_distance) != reverse) 0 else 1; // note: the XOR checks for overshoot
        if (reverse)
            x *= -1;
        const y: f64 = integral * integrals.yaw;
        const ldr, const rdr = drive.arcadeDrive(std.math.clamp(x, -1, 1), std.math.clamp(y, -1, 1));
        
        // drive
        drive.driveLeft(ldr, port_buffer);
        drive.driveRight(rdr, port_buffer);

        // if the robot has stopped rotating and the distance has coasted, break
        if (odom_state.rot_vel == 0 and std.math.signbit(coasting_distance) != reverse) // note: the XOR checks for overshoot
            break;

        // wait for next cycle
        pros.rtos.task_delay_until(&now, auton.tick_delay);
    }
}
