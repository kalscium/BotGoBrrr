const pros = @import("pros");
const options = @import("options");

pub const Motor = @import("Motor.zig");
pub const drive = @import("drive.zig");
pub const def = @import("def.zig");
pub const odom = @import("odom.zig");
pub const port = @import("port.zig");
pub const vector = @import("vector.zig");
pub const mopcontrol = @import("opcontrol.zig");
pub const pid = @import("pid.zig");
pub const logging = @import("logging.zig");
pub const tower = @import("tower.zig");
pub const controller = @import("controller.zig");
pub const tuner = @import("tuner.zig");
pub const major_minor = @import("major_minor.zig");
pub const pure_pursuit = @import("pure_pursuit.zig");
pub const debug_mode = @import("debug_mode.zig");
pub const debug_mode_pid = @import("debug_mode_pid.zig");

// prevent lazy loading
// so that the files are actually included in the outputted binary
// and so that they are also tested
comptime {
    _ = mopcontrol;
    _ = @import("autonomous.zig");
    _ = odom;
    _ = Motor;
    _ = vector;
    _ = drive;
    _ = port;
    _ = pid;
    _ = logging;
    _ = controller;
    _ = tuner;
    _ = major_minor;
    _ = pure_pursuit;
    _ = debug_mode;
}

/// Calls either the zig opcontrol, or the arm asm opcontrol
export fn opcontrol() callconv(.C) void {
    if (comptime options.asm_opcontrol) {
        struct { // i know it looks weird, but it's the only way to load the external function only if the flag is set
                 // it's fine cuz it has no runtime cost (aside from this extra function call)
            extern fn asm_opcontrol() callconv(.C) void;
        }.asm_opcontrol();
    } else if (comptime options.debug_mode_pid) {
        debug_mode_pid.entry();
    } else if (comptime options.debug_mode) {
        debug_mode.entry();
    } else if (comptime options.tune) |tune| {
        tuner.entry(tune);
    } else {
        mopcontrol.opcontrol();
    }
}

/// Gets called upon the initialization of the user-program
export fn initialize() callconv(.C) void {
    _ = pros.printf("hello, world from the initialization function\n");
    _ = pros.printf("here's an int: %d %lf%%, %d!\n", @as(c_int, 12), @as(f32, 12.2), @as(c_int, 9));
    const file = pros.fopen("/usd/file.txt", "w+") orelse unreachable;
    _ = pros.fprintf(file, "There will be %d messages!\n", @as(c_int, 2));
    _ = pros.fprintf(file, "wow, another message\n");
    _ = pros.fclose(file);
    odom.programInit();
    drive.init();
    tower.init();
}

/// Gets called upon the initialization of the user-program during a competition
export fn competition_initialize() callconv(.C) void {
    _ = pros.printf("hello, world from the competition function\n");
}

/// Gets called during the robot-disabled period
export fn disabled() callconv(.C) void {
    _ = pros.printf("hello, world from the disabled function\n");
}
