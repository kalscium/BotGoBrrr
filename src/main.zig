const pros = @import("pros");

pub const motor = @import("motor.zig");
pub const drive = @import("drive.zig");
pub const def = @import("def.zig");
pub const odom = @import("odom.zig");
pub const port = @import("port.zig");
pub const vector = @import("vector.zig");

// prevent lazy loading
// so that the files are actually included in the outputted binary
// and so that they are also tested
comptime {
    _ = @import("opcontrol.zig");
    _ = @import("autonomous.zig");
    _ = odom;
    _ = vector;
    _ = drive;
    _ = port;
}

/// Gets called upon the initialization of the user-program
export fn initialize() callconv(.C) void {
    _ = pros.printf("hello, world from the initialize function\n");
}

/// Gets called upon the initialization of the user-program during a competition
export fn competition_initialize() callconv(.C) void {
    _ = pros.printf("hello, world from the competition function\n");
}

/// Gets called during the robot-disabled period
export fn disabled() callconv(.C) void {
    _ = pros.printf("hello, world from the disabled function\n");
}
