//! Basic idea -- an intuitively imperative way to write auton that depends on coordinates
//! E.g convert `turn(90); move(20);` to `(0, -20)`

const std = @import("std");
const odom = @import("odom.zig");

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
