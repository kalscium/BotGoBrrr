//! The 'state' of the simulated robot independant of any API implementation details.
//! 
//! In that it just calcuates the correct states from actions, and doesn't have to deal
//! with ports, etc.
//! 
//! The robot is a two motor differential drive kinematic robot with a vertical and horizontal odom sensor paired with an imu (yaw).
//! Any additional motor velocities/voltages or adi-outs are logged (to be implemented).

const std = @import("std");
const configs = @import("configs.zig");

/// The current waited 'time' in the simulation in ms
time: u32 = 0,

// coordinates and yaw of the robot
x: f64 = 0,
y: f64 = 0,
/// The yaw of the robot in radians
yaw: f64 = 0,

/// The angle of the vertical rotation sensor in radians
odom_rot_ver_angle: f64 = 0,
/// The angle of the lateral rotation sensor in radians
odom_rot_lat_angle: f64 = 0,

// left & right side velocities from -1..=1 (voltages are converted by ÷12000)
drive_left_vel: f64 = 0,
drive_right_vel: f64 = 0,

/// The individual velocities of all the ports (null if never set)
motor_velocities: [21]f64 = [_]f64{ 0 } ** 21,

/// The individual gearsts (max rpm) of all the ports (200 by default)
motor_gearsets: [21]u16 = [_]u16{ 200 } ** 21,

/// Move the robot based upon the left & right side velocities and the time passed in seconds
pub fn forwardKinematics(self: *@This(), delta_time: f64) void {
    const d_left = self.drive_left_vel * configs.drive_speed * delta_time;
    const d_right = self.drive_right_vel * configs.drive_speed * delta_time;

    const d_angle = (d_left - d_right)/configs.robot_width;
    const d_tangent = (d_left + d_right)/2;

    // update the robot's yaw
    self.yaw += d_angle;
    if (self.yaw > std.math.pi)
        self.yaw -= std.math.tau
    else if (self.yaw < -std.math.pi)
        self.yaw += std.math.tau;

    // update the vertical odom rot angle
    self.odom_rot_ver_angle += d_tangent / configs.odom_wheel_radius; // delta in radians
    if (self.odom_rot_ver_angle > std.math.pi)
        self.odom_rot_ver_angle -= std.math.tau
    else if (self.odom_rot_ver_angle < -std.math.pi)
        self.odom_rot_ver_angle += std.math.tau;

    // update the horizontal odom rot angle
    // 0 - offset * delta_yaw / wheel_radius
    self.odom_rot_lat_angle -= (configs.odom_lat_offset * d_angle) / configs.odom_wheel_radius;

    // update coords
    self.x += @sin(self.yaw) * d_tangent;
    self.y += @cos(self.yaw) * d_tangent;
}
