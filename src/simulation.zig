//! A non-physics simulation of the Vex V5 brain.
//! 
//! Or, in other words, custom implementations of the PROS function calls
//! to run natively.

comptime { _ = @import("simulation/stub.zig"); } // so stubs are linked
const std = @import("std");

extern fn initialize() callconv(.C) void;
extern fn disabled() callconv(.C) void;
extern fn autonomous() callconv(.C) void;
extern fn opcontrol() callconv(.C) void;

/// The entry function that runs the exposed 'init', 'disabled', 'opcontrol', 'autonomous' functions etc
pub fn main() void {
    std.debug.print("<<< SIMULATION ROUTINE START >>>\n  * init -> disabled -> auton -> disabled -> opctrl\n\n", .{});
    initialize();
    disabled();
    autonomous();
    disabled();
    opcontrol();
}
