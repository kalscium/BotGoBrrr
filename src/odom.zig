//! Functions & Calculations for the robot's odometry coordinate system

const std = @import("std");
const pros = @import("pros");
const port = @import("port.zig");
const def = @import("def.zig");
const vector = @import("vector.zig");

/// The current diameter of the robot's odometry wheel in mm
const wheel_diameter = 101.6;

/// The starting coordinate of the robot
pub const start_coord = Coord{ 0, 0 };

/// The port of the odometry rotation sensor
const rotation_port = 12;
/// The port of the IMU sensor
const imu_port = 12;

/// A single coordinate/vector
pub const Coord = @Vector(2, f64);

/// Finds the minimal possible difference in angle between two angles (radians)
pub fn minimalAngleDiff(x: f64, y: f64) f64 {
    // should work probably
    var raw_diff = y - x;
    if (raw_diff > std.math.pi)
        raw_diff -= std.math.tau
    else if (raw_diff < -std.math.pi)
        raw_diff += std.math.tau;

    return raw_diff;
}

test minimalAngleDiff {
    // tests for the imu (0 forwards, positive is right, negative is left)
    std.debug.assert(std.math.round(comptime std.math.degreesToRadians(45)) == std.math.round(minimalAngleDiff(comptime std.math.degreesToRadians(0), comptime std.math.degreesToRadians(45))));
    std.debug.assert(std.math.round(comptime std.math.degreesToRadians(-45)) == std.math.round(minimalAngleDiff(comptime std.math.degreesToRadians(90), comptime std.math.degreesToRadians(45))));
    std.debug.assert(std.math.round(comptime std.math.degreesToRadians(20)) == std.math.round(minimalAngleDiff(comptime std.math.degreesToRadians(170), comptime std.math.degreesToRadians(-170))));
    std.debug.assert(std.math.round(comptime std.math.degreesToRadians(-20)) == std.math.round(minimalAngleDiff(comptime std.math.degreesToRadians(-170), comptime std.math.degreesToRadians(170))));
    std.debug.assert(std.math.round(comptime std.math.degreesToRadians(-45)) == std.math.round(minimalAngleDiff(comptime std.math.degreesToRadians(-45), comptime std.math.degreesToRadians(-90))));
    std.debug.assert(std.math.round(comptime std.math.degreesToRadians(20)) == std.math.round(minimalAngleDiff(comptime std.math.degreesToRadians(-10), comptime std.math.degreesToRadians(10))));
    std.debug.assert(std.math.round(comptime std.math.degreesToRadians(-20)) == std.math.round(minimalAngleDiff(comptime std.math.degreesToRadians(10), comptime std.math.degreesToRadians(-10))));

    // tests for odom tracking wheels
    std.debug.assert(std.math.round(comptime std.math.degreesToRadians(20)) == std.math.round(minimalAngleDiff(comptime std.math.degreesToRadians(350), comptime std.math.degreesToRadians(10))));
    std.debug.assert(std.math.round(comptime std.math.degreesToRadians(-20)) == std.math.round(minimalAngleDiff(comptime std.math.degreesToRadians(10), comptime std.math.degreesToRadians(350))));
}

/// Calculates the distance travelled in mm based upon odom wheel rotation
/// angle in radians through circumference calculations
pub fn odomMagnitude(angle: f64) f64 {
    const circumference = comptime wheel_diameter * std.math.pi;
    return angle / 2 / std.math.pi * circumference;
}

/// Gets the yaw value of an IMU sensor in radians, reports any disconnects
pub fn getYaw(port_buffer: *port.PortBuffer) f64 {
    const result = pros.imu.imu_get_yaw(imu_port);

    // check for errors
    if (result == def.pros_err_f64) {
        if (pros.__errno().* == def.pros_error_code.enodev) {
            port_buffer.portWrite(imu_port, false);
        }
    }

    return std.math.degreesToRadians(@as(f64, @floatCast(result)));
}

/// Gets the rotation value of a rotation sensor, reports any disconnects
pub fn getRotation(comptime rport: u8, port_buffer: *port.PortBuffer) f64 {
    const result = pros.rotation.rotation_get_angle(rport);

    // check for errors
    if (result == def.pros_err_i32) {
        if (pros.__errno().* == def.pros_error_code.enodev) {
            port_buffer.portWrite(rport, false);
        }
    }

    return std.math.degreesToRadians(std.math.degreesToRadians(@as(f64, @floatFromInt(result)) / 100));
}

/// Odometry state variables
pub const State = struct {
    /// The previous rotation sensor reading
    prev_rotation: f64,
    /// The previous imu sensor reading (for yaw)
    prev_yaw: f64,
    /// The previous time in ms
    prev_time: u32,
    /// The robot's current coordinate
    coord: Coord,
    /// The robot's current movement velocity
    mov_vel: f64,
    /// The robot's current rotational velocity
    rot_vel: f64,

    /// Initializes the odometry state variables
    pub fn init(port_buffer: *port.PortBuffer) State {
        return .{
            .prev_rotation = getRotation(rotation_port, port_buffer),
            .prev_yaw = getYaw(port_buffer),
            .prev_time = pros.rtos.millis(),
            .mov_vel = 0,
            .rot_vel = 0,
            .coord = start_coord,
        };
    }
};

/// Updates the odometry coordinates based upon previous and current rotation
/// sensor values (right and left)
pub fn updateOdom(state: *State, port_buffer: *port.PortBuffer) void {
    // get the current sensor readings/values
    const yaw = getYaw(port_buffer);
    const rotation = getRotation(rotation_port, port_buffer);
    const now = pros.rtos.millis();

    // calculate the distance travelled for the rotation sensor
    const distance = odomMagnitude(minimalAngleDiff(state.prev_rotation, rotation));

    // update the current coordinate with the distance moved
    const moved = vector.polarToCartesian(distance, yaw);
    state.coord += moved;

    // calculate the velocities
    const dt: f64 = @floatFromInt(now - state.prev_time);
    state.mov_vel = distance / dt;
    state.rot_vel = (yaw - state.prev_yaw) / dt;

    // update the previous values
    state.prev_rotation = rotation;
    state.prev_yaw = yaw;
    state.prev_time = now;
}

/// The CSV header for the coordinate log
pub const csv_header_coords = "x (mm),y (mm)\n";

/// Checks and logs the coordinates of the robot from the odom state
pub fn logCoords(file: *std.c.FILE, state: State) void {
    _ = pros.fprintf(file, "%lf,%lf\n", state.coord[0], state.coord[1]);
}

/// The CSV header for the velocity log
pub const csv_header_velocity = "time (ms),movement (mm/s),rotation (°/s)\n";

/// Checks and logs the movement velocity and rotational velocity of the robot
pub fn logVelocity(file: *std.c.FILE, state: State) void {
    _ = pros.fprintf(file, "%d,%lf,%lf\n", state.prev_time, state.mov_vel, state.rot_vel);
}
