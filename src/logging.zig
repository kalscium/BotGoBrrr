//! A centralized place for all the functions that do logging for each component of this program

const std = @import("std");
const pros = @import("pros");
const drive = @import("drive.zig");
const def = @import("def.zig");
const odom = @import("odom.zig");

/// The CSV header for the coordinate log
pub const csv_header_coords = "x (mm),y (mm)\n";

/// Checks and logs the coordinates of the robot from the odom state
pub fn coords(file: *std.c.FILE, state: odom.State) void {
    _ = pros.fprintf(file, "%lf,%lf\n", state.coord[0], state.coord[1]);
}

/// The CSV header for the velocity log
pub const csv_header_velocity = "time (ms),movement (mm/s),rotation (*/s)\n";

/// Checks and logs the movement velocity and rotational velocity of the robot
pub fn velocity(file: *std.c.FILE, state: odom.State) void {
    _ = pros.fprintf(file, "%d,%lf,%lf\n", state.prev_time, state.mov_vel, state.rot_vel);
}

/// The CSV header for the drive motor temperature log
pub const csv_header_temp = "time (s),battery (*C),l1 (*C),l2 (*C),l3 (*C),r1 (*C),r2 (*C),r3 (*C)\n";

/// Checks and logs the temperatures of all the motors and battery alongside the tick
pub fn temp(ms: u32, file: *std.c.FILE) void {
    const time = @as(f64, @floatFromInt(ms)) / 1000;

    var battery_temp = pros.misc.battery_get_temperature();
    if (battery_temp == def.pros_err_f64) // in case it fails
        battery_temp = 0;

    var l1_temp = pros.motors.motor_get_temperature(drive.drivetrain_motors.l1.port);
    if (l1_temp == def.pros_err_f64) // in case it fails
        l1_temp = 0;
    var l2_temp = pros.motors.motor_get_temperature(drive.drivetrain_motors.l2.port);
    if (l2_temp == def.pros_err_f64) // in case it fails
        l2_temp = 0;
    var l3_temp = pros.motors.motor_get_temperature(drive.drivetrain_motors.l3.port);
    if (l3_temp == def.pros_err_f64) // in case it fails
        l3_temp = 0;

    var r1_temp = pros.motors.motor_get_temperature(drive.drivetrain_motors.r1.port);
    if (r1_temp == def.pros_err_f64) // in case it fails
        r1_temp = 0;
    var r2_temp = pros.motors.motor_get_temperature(drive.drivetrain_motors.r2.port);
    if (r2_temp == def.pros_err_f64) // in case it fails
        r2_temp = 0;
    var r3_temp = pros.motors.motor_get_temperature(drive.drivetrain_motors.r3.port);
    if (r3_temp == def.pros_err_f64) // in case it fails
        r3_temp = 0;

    // write it to the logfile
    _ = pros.fprintf(
        file,
        "%lf,%lf,%lf,%lf,%lf,%lf,%lf,%lf\n",
        time,
        battery_temp,
        l1_temp,
        l2_temp,
        l3_temp,
        r1_temp,
        r2_temp,
        r3_temp,
    );
}
