const pros = @import("pros");

/// Gets called upon the initialization of the user-program
export fn initialize() callconv(.C) void {
    _ = pros.printf("hello, world from the initialize function");
}

/// Gets called upon the initialization of the user-program during a competition
export fn competition_initialize() callconv(.C) void {
    _ = pros.printf("hello, world from the competition function");
}

/// Gets called during the robot-disabled period
export fn disabled() callconv(.C) void {
    _ = pros.printf("hello, world from the disabled function");
}

/// Gets called during the driver-control period
export fn opcontrol() callconv(.C) void {
    _ = pros.printf("hello, world from the opcontrol function");
}

/// Gets called during the autonomous period
export fn autonomous() callconv(.C) void {
    _ = pros.printf("hello, world from the autonomous function");
}
