//! Program specific configurations (like port numbers) for the sim

const program_odom = @import("../odom.zig");
const program_drive = @import("../drive.zig");
const program_pure_pursuit = @import("../pure_pursuit.zig");

/// The physical width of the robot in mm
pub const robot_width = program_pure_pursuit.robot_width;

/// The top robot drive speed at max velocity in m/s
pub const drive_speed = 1.0;

pub const drive_left_port = program_drive.drivetrain_motors.l1.port;
pub const drive_right_port = program_drive.drivetrain_motors.r1.port;

pub const odom_ver_rot_port = program_odom.rotation_port_vertical;
pub const odom_lat_rot_port = program_odom.rotation_port_lateral;

/// The odom wheel radius in mm
pub const odom_wheel_radius = program_odom.wheel_radius;

/// The offset of the horizontal odom sensor in mm
pub const odom_lat_offset = program_odom.rot_lateral_offset;
