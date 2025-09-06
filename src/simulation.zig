//! A non-physics simulation of the Vex V5 brain.
//! 
//! Or, in other words, custom implementations of the PROS function calls
//! to run natively.

comptime { // so implementations are linked
    _ = @import("simulation/stub.zig");
    _ = @import("simulation/io.zig");
}

const std = @import("std");

extern fn initialize() callconv(.C) void;
extern fn disabled() callconv(.C) void;
extern fn autonomous() callconv(.C) void;
extern fn opcontrol() callconv(.C) void;

pub var allocator: std.mem.Allocator = undefined;

/// The entry function that runs the exposed 'init', 'disabled', 'opcontrol', 'autonomous' functions etc
pub fn main() void {
    // set up the allocator
    var gpa_alloc = std.heap.DebugAllocator(.{}).init;
    defer if (gpa_alloc.deinit() == .leak) @panic("simulation memory leak!");
    allocator = gpa_alloc.allocator();

    std.debug.print("<<< SIMULATION ROUTINE START >>>\n  * init -> disabled -> auton -> disabled -> opctrl\n\n", .{});
    initialize();
    disabled();
    autonomous();
    disabled();
    opcontrol();
}
