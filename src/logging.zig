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

/// The CSV header for the drive motor temperature log
pub const csv_header_temp = "time (s),battery (*C),tower a (*C),tower b (*C),tower c (*C),tower d (*C),l1 (*C),l2 (*C),l3 (*C),r1 (*C),r2 (*C),r3 (*C)\n";
