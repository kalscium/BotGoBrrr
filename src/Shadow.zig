//! Basic idea -- an intuitively imperative way to write auton that depends on coordinates
//! E.g convert `turn(90); move(20);` to `(0, -20)`

const std = @import("std");
const odom = @import("odom.zig");
const port = @import("port.zig");
const pid = @import("pid.zig");

/// Calculated x position in mm
x: f64 = 0,
/// Calculated y position in mm
y: f64 = 0,
/// Current yaw in radians
yaw: f64 = 0,

/// Rotates the shadow to a set degree
pub fn rotateToDeg(self: *@This(), deg: f64) void {
    self.yaw = std.math.degreesToRadians(deg);
}

/// Moves the robot by the set amount of centemetres
pub fn moveCM(self: *@This(), distance: f64) void {
    self.x += @sin(self.yaw) * distance * 10;
    self.y += @cos(self.yaw) * distance * 10;
}

/// Converts itself into a coordinate
pub fn toCoord(self: @This()) odom.Coord {
    return .{
        self.x,
        self.y,
    };
}

/// Blocking state machine to rotate robot with PID
pub fn rotateToDegPID(self: *@This(), degree: f64, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    self.yaw = std.math.degreesToRadians(degree);
    pid.rotateDeg(degree, odom_state, port_buffer);
}

/// Blocking state machine to move robot with PID
pub fn moveMMPID(self: *@This(), distance: f64, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    self.x += @sin(self.yaw) * distance;
    self.y += @cos(self.yaw) * distance;
    pid.moveCoord(.{ self.x, self.y }, odom_state, port_buffer); // try moveChainCoord for motion chaining
}

/// Blocking state machine to move the robot with a dual pid
pub fn moveDDP(self: *@This(), left_mm: f64, right_mm: f64, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    const delta_yaw = (right_mm - left_mm)/@import("pure_pursuit.zig").robot_width;
    const distance = (left_mm + right_mm)/2;

    // calculate updated yaw
    self.yaw += delta_yaw;
    if (self.yaw > std.math.pi)
        self.yaw -= std.math.tau
    else if (self.yaw < -std.math.pi)
        self.yaw += std.math.tau;

    // calculate updated coords
    self.x += @sin(self.yaw) * distance;
    self.y += @cos(self.yaw) * distance;

    // drive it
    pid.driveDDP(left_mm, right_mm, odom_state, port_buffer);
}
