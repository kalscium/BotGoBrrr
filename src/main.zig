const pros = @import("pros");
const options = @import("options");

pub const motor = @import("motor.zig");
pub const drive = @import("drive.zig");
pub const def = @import("def.zig");
pub const odom = @import("odom.zig");
pub const port = @import("port.zig");
pub const vector = @import("vector.zig");
pub const mopcontrol = @import("opcontrol.zig");

// prevent lazy loading
// so that the files are actually included in the outputted binary
// and so that they are also tested
comptime {
    _ = mopcontrol;
    _ = @import("autonomous.zig");
    _ = odom;
    _ = vector;
    _ = drive;
    _ = port;
}

/// Calls either the zig opcontrol, or the arm asm opcontrol
export fn opcontrol() callconv(.C) void {
    if (comptime options.asm_opcontrol) {
        struct { // i know it looks weird, but it's the only way to load the external function only if the flag is set
                 // it's fine cuz it has no runtime cost (aside from this extra function call)
            extern fn asm_opcontrol() callconv(.C) void;
        }.asm_opcontrol();
    } else {
        mopcontrol.opcontrol();
    }
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
