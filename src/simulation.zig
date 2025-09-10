//! A non-physics simulation of the Vex V5 brain.
//! 
//! Or, in other words, custom implementations of the PROS function calls
//! to run natively.

comptime { // so implementations are linked
    _ = @import("simulation/stub.zig");
    _ = @import("simulation/io.zig");
    _ = @import("simulation/devices.zig");
    _ = @import("simulation/configs.zig");
}

pub const State = @import("simulation/State.zig");

const std = @import("std");

extern fn initialize() callconv(.C) void;
extern fn disabled() callconv(.C) void;
extern fn autonomous() callconv(.C) void;
extern fn opcontrol() callconv(.C) void;

pub var allocator: std.mem.Allocator = undefined;
pub var sim_state: State = State{};
pub var sim_log: std.io.BufferedWriter(4096, std.fs.File.Writer) = undefined;

/// The entry function that runs the exposed 'init', 'disabled', 'opcontrol', 'autonomous' functions etc
pub fn main() !void {
    // set up the allocator
    var gpa_alloc = std.heap.DebugAllocator(.{}).init;
    defer if (gpa_alloc.deinit() == .leak) @panic("simulation memory leak!");
    allocator = gpa_alloc.allocator();

    // open the log file
    var file = try std.fs.cwd().createFile("sim-log.csv", .{});
    defer file.close();
    sim_log = std.io.bufferedWriter(file.writer());
    defer sim_log.flush() catch unreachable;

    // write the header
    try sim_log.writer().writeAll("time (ms),x (mm),y (mm),yaw (*),odom_rot_ver_angle (*),odom_rot_lat_angle (*),ldr%,rdr%,motor_1%,motor_2%,motor_3%,motor_4%,motor_5%,motor_6%,motor_7%,motor_8%,motor_9%,motor_10%,motor_11%,motor_12%,motor_13%,motor_14%,motor_15%,motor_16%,motor_17%,motor_18%,motor_19%,motor_20%,motor_21%\n");

    std.debug.print("<<< SIMULATION ROUTINE START >>>\n  * init -> disabled -> auton\n\n", .{});
    initialize();
    disabled();
    autonomous();
    // disabled();
    // opcontrol();
}
