//! Defines the driver-control routine

const std = @import("std");

const pros = @import("pros");

const odom = @import("odom.zig");
const pid = @import("pid.zig");
const port = @import("port.zig");
const vector = @import("vector.zig");
const drive = @import("drive.zig");
const tower = @import("tower.zig");
const pure_pursuit = @import("pure_pursuit.zig");

/// The delay in ms, between each 'cycle' of autonomous (the lower the more precise though less stable)
pub const cycle_delay = 10;

/// The path to the autonomous port buffers file
const port_buffer_path = "/usd/auton_port_buffers.bin";

/// The 'precision' (in mm) that the robot must achieve before moving onto the next path coordinate
pub const precision_mm: f64 = 10; // try 5
/// The 'precision' (in radians) that the robot must achieve before moving onto the next path coordinate
pub const precision_rad: f64 = std.math.degreesToRadians(1);

/// The *tuned* movement (Y) PID controller
pub const mov_pid_param = pid.Param {
    // 1 / max_error
    // max_error = pure pursuit search-radius
    // 1" as it's half a field tile
    .kp = 1.0 / 304.8 / 4.0,
    // arbitrary, before tuning
    // .ki = (1.0 / 304.8 / 4.0) * tick_delay,
    // .kd = (1.0 / 304.8 / 4.0) * 5.0,
    .ki = 0,
    .kd = 0,
    .saturation = 1,
    .low_pass_a = 0.8,
};

/// The *tuned* yaw (radians) PID controller
pub const yaw_pid_param = pid.Param {
    // 1 / max_error
    .kp = 1.0 / std.math.degreesToRadians(90.0) / 3.0, // seems to work perfectly
    // proportional alone is enough, due to us setting velocity instead of voltage
    .ki = 0,
    .kd = 0,
    .saturation = 1,
    .low_pass_a = 0.8,
};

/// The *tuned* pure pursuit parameters
pub const pure_pursuit_params = pure_pursuit.Parameters{
    .search_radius = 240.0, // works well enough, but robot osccilates a bit, so try 300 if it's not too inaccurate
    .kp = 0.4, // reasonable speed (accurate and fast enough), try 0.5 if you want it to be faster
    .lookahead_window = 20.0, // works as is, might be too high as robot is pre-maturely stopping
};

export fn autonomous() callconv(.C) void {
    _ = pros.printf("hello, world from autonomous!\n");
    // open the motor disconnect file
    const port_buffer_file = pros.fopen(port_buffer_path, "wb");
    defer if (port_buffer_file) |file| {
        _ = pros.fclose(file);
    };

    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);

    // pid.rotate(55.44, &odom_state, &port_buffer);
    // tower.spinScoreHigh(12000, &port_buffer);
    // pid.move(.{ 369.8, 537 }, &odom_state, &port_buffer);
    // tower.spinScoreHigh(0, &port_buffer);
    // pid.rotate(135, &odom_state, &port_buffer);
    // pid.move(.{ 1011, 0 }, &odom_state, &port_buffer);
    // pid.rotate(0, &odom_state, &port_buffer);
    // pid.move(.{ 1011, 600 }, &odom_state, &port_buffer);
    // tower.spinScoreHigh(12000, &port_buffer);
    // wait(2000, &odom_state, &port_buffer);
    // tower.spinScoreHigh(0, &port_buffer);

    // last ditch in case I can't get auton working before first comp
    drive.driveLeft(1, &port_buffer);
    drive.driveRight(1, &port_buffer);

    // wait a while time
    wait(200, &odom_state, &port_buffer);

    drive.driveLeft(0, &port_buffer);
    drive.driveRight(0, &port_buffer);    

    // write the port buffer to the port_buffer file
    if (port_buffer_file) |file|
        _ = pros.fwrite(@ptrCast(&port_buffer), comptime @bitSizeOf(port.PortBuffer)/8, 1, file);
}

/// Waits a certain amount of time, whilst still updating odom
fn wait(delay_ms: u32, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    var now = pros.rtos.millis();
    const start = now;
    while ((now - start) / delay_ms < 1) {
        odom_state.update(port_buffer);
        pros.rtos.task_delay_until(&now, cycle_delay);
    }
}
