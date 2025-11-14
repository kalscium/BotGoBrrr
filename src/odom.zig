//! Functions & Calculations for the robot's odometry coordinate system

const std = @import("std");
const pros = @import("pros");
const port = @import("port.zig");
const def = @import("def.zig");
const vector = @import("vector.zig");
const controller = @import("controller.zig");
const pure_pursuit = @import("pure_pursuit.zig");

/// The controller button for recording the current odom coordinate
pub const record_coord_button = pros.misc.E_CONTROLLER_DIGITAL_UP;

/// The current radius of the robot's odometry wheel in mm
pub const wheel_radius = 34.925;

/// The starting coordinate of the robot
pub const start_coord = Coord{ 0, 0 };

/// The port of the vertical odometry rotation sensor
pub const rotation_port_vertical = 13;
/// The port of the lateral odometry rotation sensor
pub const rotation_port_lateral = 16;
/// The offset from the midde (in mm), along the vertical axis of the lateral rotation sensor
pub const rot_lateral_offset = 0; // quickfix
/// The port of the IMU sensor
pub const imu_port = 15;

/// A single coordinate/vector
pub const Coord = @Vector(2, f64);

/// Undoes a lateral sensor offset in the perpendicular (Y) axis.
/// 
/// Takes in the delta distance (in mm), delta yaw (in radians) and
/// also the y offset (in mm) and returns the new corrected delta distance
pub inline fn undoLatOffset(delta_distance: f64, delta_yaw: f64, offset: f64) f64 {
    //   Δyaw(Δdistance/Δyaw + offset)
    // = Δdistance + offsetΔyaw
    // 
    // as the radian is just a step around a unit circle,
    // and a the angle times the radius is the segment of the circumference
    // that angle makes up.
    // 
    // first, find the radius of the arc (ICR), then add the offset to it,
    // and find the new arc from the new radius.

    return delta_distance + delta_yaw * offset;
}

test undoLatOffset {
    const offset = -20;
    const delta_yaw = std.math.degreesToRadians(10);
    const delta_distance = 16 - offset * delta_yaw;
    const true_distance = undoLatOffset(delta_distance, delta_yaw, offset);
    std.debug.assert(true_distance == 16);
}

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
    // the neat thing about radians, is that they're just steps around a unit-circle
    // and by simply multiplying by a new radius, the unit circle angle becomes the
    // segment in the circumference
    return angle * wheel_radius;
}

/// Gets the yaw value of an IMU sensor in radians, reports any disconnects
pub fn getYaw(port_buffer: *port.PortBuffer) ?f64 {
    const result = pros.imu.imu_get_yaw(imu_port);

    // check for errors
    if (result == def.pros_err_f64) {
        if (pros.__errno().* == def.pros_error_code.enodev) {
            port_buffer.portWrite(imu_port, false);
        }
        return null;
    }

    return std.math.degreesToRadians(@as(f64, @floatCast(result)));
}

/// Gets the rotation value of a rotation sensor, reports any disconnects
pub fn getRotation(comptime rport: u8, port_buffer: *port.PortBuffer) ?f64 {
    const result = pros.rotation.rotation_get_angle(rport);

    // check for errors
    if (result == def.pros_err_i32) {
        if (pros.__errno().* == def.pros_error_code.enodev) {
            port_buffer.portWrite(rport, false);
        }
        return null;
    }

    return std.math.degreesToRadians(@as(f64, @floatFromInt(result)) / 100.0);
}

/// # MUST BE RUN AT PROGRAM INIT
/// 
/// Necessary initialization code (such as taring/calibrating) required for odom
pub fn programInit() void {
    _ = pros.imu.imu_reset(imu_port);
    _ = pros.rotation.rotation_set_reversed(rotation_port_lateral, true); // lateral rotation sensor is physically reversed from the robot's movements, remove this if it's fixed
}

/// Odometry state variables
pub const State = struct {
    /// The previous vertical rotation sensor reading
    prev_ver_rotation: f64,
    /// The previous lateral rotation sensor reading
    prev_lat_rotation: f64,
    /// The previous imu sensor reading (for yaw)
    prev_yaw: f64,
    /// The previous time in ms
    prev_time: u32,
    /// The robot's current coordinate
    coord: Coord,
    /// The robot's current vertical movement velocity in mm/ms
    mov_ver_vel: f64,
    /// The robot's current lateral movement velocity in mm/ms
    mov_lat_vel: f64,
    /// The robot's current rotational velocity in rad/ms
    rot_vel: f64,

    /// The calculated left & right moved distances
    left_dist: f64,
    right_dist: f64,

    /// Initializes the odometry state variables
    pub fn init(port_buffer: *port.PortBuffer) State {
        return .{
            .prev_ver_rotation = getRotation(rotation_port_vertical, port_buffer) orelse 0,
            .prev_lat_rotation = getRotation(rotation_port_lateral, port_buffer) orelse 0,
            .prev_yaw = getYaw(port_buffer) orelse 0,
            .prev_time = pros.rtos.millis(),
            .mov_ver_vel = 0,
            .mov_lat_vel = 0,
            .rot_vel = 0,
            .coord = start_coord,
            .left_dist = 0,
            .right_dist = 0,
        };
    }

    /// Updates the odometry coordinates based upon previous and current rotation
    /// sensor values (right and left)
    pub fn update(state: *State, port_buffer: *port.PortBuffer) void {
        // debug log of odom
        if (pros.rtos.millis() % 200 < 10)
            _ = pros.printf("odom_coord: (%lf, %lf), yaw: %lf\n", state.coord[0], state.coord[1], std.math.radiansToDegrees(state.prev_yaw));

        // get the current sensor readings/values
        const yaw = getYaw(port_buffer) orelse 0;
        const ver_rotation = getRotation(rotation_port_vertical, port_buffer) orelse 0;
        const lat_rotation = getRotation(rotation_port_lateral, port_buffer);
        const now = pros.rtos.millis();

        // calculate the distance travelled for the rotation sensors
        const ver_distance = odomMagnitude(minimalAngleDiff(state.prev_ver_rotation, ver_rotation));
        const lat_distance = if (lat_rotation) |rotation| distance: {
            const raw_distance = odomMagnitude(minimalAngleDiff(state.prev_lat_rotation, rotation));
            const true_distance = undoLatOffset(raw_distance, minimalAngleDiff(state.prev_yaw, yaw), rot_lateral_offset);
            break :distance true_distance;
        } else 0; // so calculations are correct

        // update the current coordinate with the distance moved
        const moved_ver = vector.polarToCartesian(ver_distance, yaw);
        const moved_lat = vector.polarToCartesian(lat_distance, yaw + comptime std.math.degreesToRadians(90)); // perpendicular
        state.coord += moved_ver;
        state.coord += moved_lat;

        // calculate the velocities
        const dt: f64 = @floatFromInt(now - state.prev_time);
        state.mov_ver_vel = ver_distance / dt;
        state.mov_lat_vel = lat_distance / dt;
        const delta_yaw = minimalAngleDiff(state.prev_yaw, yaw);
        state.rot_vel = delta_yaw / dt;

        // calculate the left & right moved distances
        if (delta_yaw > 0) { // turning right
            state.left_dist += ver_distance + pure_pursuit.robot_width/2 * @abs(delta_yaw);
            state.right_dist += ver_distance - pure_pursuit.robot_width/2 * @abs(delta_yaw);
        } else { // turning left
            state.left_dist += ver_distance - pure_pursuit.robot_width/2 * @abs(delta_yaw);
            state.right_dist += ver_distance + pure_pursuit.robot_width/2 * @abs(delta_yaw);
        }

        // update the previous values
        state.prev_ver_rotation = ver_rotation;
        state.prev_lat_rotation = lat_rotation orelse 0;
        state.prev_yaw = yaw;
        state.prev_time = now;
    }

    /// Reads the controller and updates the odom state accordingly, or does
    /// other actions that read the odom state
    pub fn controllerUpdate(self: State, file: ?*std.c.FILE) void {
        // check for the 'record position' button press, print to both file & stdout
        if (controller.get_digital_new_press(record_coord_button)) {
            // super compact and efficient binary files are cool and all but they
            // just aren't worth it for something like this where it'd be written
            // to like 8 times at most instead of EVERY TICK
            _ = pros.printf("Recorded Coord at: .{ %lf, %lf }\nCurrent Yaw: %lf*\n", self.coord[0], self.coord[1], std.math.radiansToDegrees(self.prev_yaw));
            if (file) |f|
                _ = pros.fprintf(f, "Recorded Coord at: .{ %lf, %lf }\nCurrent Yaw: %lf*\n", self.coord[0], self.coord[1], std.math.radiansToDegrees(self.prev_yaw));
        }
    }
};
