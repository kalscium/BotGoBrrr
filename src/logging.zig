//! A centralized place for all the functions that do logging for each component of this program

const std = @import("std");
const pros = @import("pros");
const drive = @import("drive.zig");
const def = @import("def.zig");
const odom = @import("odom.zig");
const tower = @import("tower.zig");

/// Closes an open *optional* file
pub fn closeFile(file: ?*std.c.FILE) void {
    const f = file orelse return;
    _ = pros.fclose(f);
}

/// Writes a log header to an optional file
pub fn writeHeader(file: ?*std.c.FILE, header: [:0]const u8) void {
    const f = file orelse return;
    _ = pros.fprintf(f, header);
}

/// The CSV header for the benchmark
pub const csv_header_bench = "compute time (ms),logging time (ms),total time (ms)\n";

/// Logs the benchmark times
/// 
/// note: the logging of the benchmark itself is not included in these calculations
pub fn benchmark(file: ?*std.c.FILE, compute_time: u32, log_time: u32, total_time: u32) void {
    const f = file orelse return;
    _ = pros.fprintf(f, "%lu,%lu,%lu\n", compute_time, log_time, total_time);
}

/// The CSV header for the battery percentage (battery & controller)
pub const csv_header_battery = "time (s), battery capacity%,controller level%\n";

/// Checks the battery percentages and logs them
pub fn battery(ms: u32, file: ?*std.c.FILE) void {
    const f = file orelse return;
    var batt = pros.misc.battery_get_capacity();
    var controller = pros.misc.battery_get_capacity();

    if (batt == def.pros_err_f64)
        batt = 0;
    if (controller == def.pros_err_f64)
        controller = 0;

    const time = @as(f64, @floatFromInt(ms))/1000;

    _ = pros.fprintf(f, "%lf,%lf,%lf\n", time, batt, controller);
}

/// The CSV header for the coordinate log
pub const csv_header_coords = "time (ms), x (mm),y (mm)\n";

/// Checks and logs the coordinates of the robot from the odom state
pub fn coords(file: ?*std.c.FILE, state: odom.State) void {
    const f = file orelse return;
    _ = pros.fprintf(f, "%lu,%lf,%lf\n", state.prev_time, state.coord[0], state.coord[1]);
}

/// The CSV header for the velocity log
pub const csv_header_velocity = "time (ms),vertical movement (m/s),lateral movement (m/s),rotation (*/s),tower motor a (rpm),tower motor b (rpm),tower motor c (rpm),tower motor d (rpm)\n";

/// Checks and logs the movement velocity and rotational velocity of the robot
pub fn velocity(file: ?*std.c.FILE, state: odom.State) void {
    const f = file orelse return;

    var tower_a_vel = @abs(pros.motors.motor_get_actual_velocity(tower.motors.a.port));
    if (tower_a_vel == def.pros_err_f64) // in case it fails
        tower_a_vel = 0;
    var tower_b_vel = @abs(pros.motors.motor_get_actual_velocity(tower.motors.b.port));
    if (tower_b_vel == def.pros_err_f64) // in case it fails
        tower_b_vel = 0;
    var tower_c_vel = @abs(pros.motors.motor_get_actual_velocity(tower.motors.c.port));
    if (tower_c_vel == def.pros_err_f64) // in case it fails
        tower_c_vel = 0;
    var tower_d_vel = @abs(pros.motors.motor_get_actual_velocity(tower.motors.d.port));
    if (tower_d_vel == def.pros_err_f64) // in case it fails
        tower_d_vel = 0;

    _ = pros.fprintf(f, "%lu,%lf,%lf,%lf,%lf,%lf,%lf\n", state.prev_time, state.mov_ver_vel, state.mov_lat_vel, std.math.radiansToDegrees(state.rot_vel), tower_a_vel, tower_b_vel, tower_c_vel, tower_d_vel);
}

/// The CSV header for the drive motor temperature log
pub const csv_header_temp = "time (s),battery (*C),tower a (*C),tower b (*C),tower c (*C),tower d (*C),l1 (*C),l2 (*C),l3 (*C),r1 (*C),r2 (*C),r3 (*C)\n";

/// Checks and logs the temperatures of all the motors and battery alongside the tick
pub fn temp(ms: u32, file: ?*std.c.FILE) void {
    const f = file orelse return;
    const time = @as(f64, @floatFromInt(ms)) / 1000;

    var battery_temp = pros.misc.battery_get_temperature();
    if (battery_temp == def.pros_err_f64) // in case it fails
        battery_temp = 0;

    var tower_a_temp = pros.motors.motor_get_temperature(tower.motors.a.port);
    if (tower_a_temp == def.pros_err_f64) // in case it fails
        tower_a_temp = 0;
    var tower_b_temp = pros.motors.motor_get_temperature(tower.motors.b.port);
    if (tower_b_temp == def.pros_err_f64) // in case it fails
        tower_b_temp = 0;
    var tower_c_temp = pros.motors.motor_get_temperature(tower.motors.c.port);
    if (tower_c_temp == def.pros_err_f64) // in case it fails
        tower_c_temp = 0;
    var tower_d_temp = pros.motors.motor_get_temperature(tower.motors.d.port);
    if (tower_d_temp == def.pros_err_f64) // in case it fails
        tower_d_temp = 0;

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
        f,
        "%lf,%lf,%lf,%lf,%lf,%lf,%lf,%lf,%lf,%lf,%lf,%lf\n",
        time,
        battery_temp,
        tower_a_temp,
        tower_b_temp,
        tower_c_temp,
        tower_d_temp,
        l1_temp,
        l2_temp,
        l3_temp,
        r1_temp,
        r2_temp,
        r3_temp,
    );
}
