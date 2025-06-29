//! Defines the driver-control routine

const std = @import("std");

const pros = @import("pros");

const odom = @import("odom.zig");
const pid = @import("pid.zig");
const port = @import("port.zig");
const vector = @import("vector.zig");

/// The delay in ms, between each 'cycle' of autonomous (the lower the more precise though less stable)
pub const tick_delay = 10;

/// The path to the autonomous port buffers file
const port_buffer_path = "/usd/auton_port_buffers.bin";

/// The 'precision' (in mm) that the robot must achieve before moving onto the next path coordinate
pub const precision_mm: f64 = 5;
/// The 'precision' (in radians) that the robot must achieve before moving onto the next path coordinate
pub const precision_rad: f64 = std.math.degreesToRadians(1);

/// The *tuned* movement (Y) PID controller
pub const mov_pid_param = pid.Param {
    // 1 / max_error
    // max_error = pure pursuit search-radius
    // 1" as it's half a field tile
    .kp = 1.0 / 304.8,
    // arbitrary, before tuning
    .ki = (1.0 / 304.8) / 100.0,
    .kd = (1.0 / 304.8) * 5.0,
    .saturation = 1,
    .low_pass_a = 0.8,
};

/// The *tuned* yaw (radians) PID controller
pub const yaw_pid_param = pid.Param {
    // 1 / max_error
    .kp = 1.0 / std.math.degreesToRadians(90.0),
    // arbitrary, before tuning
    .ki = (1.0 / std.math.degreesToRadians(90.0)) / 100.0,
    .kd = (1.0 / std.math.degreesToRadians(90.0)) * 5.0,
    .saturation = 1,
    .low_pass_a = 0.8,
};

export fn autonomous() callconv(.C) void {
    // open the motor disconnect file
    const port_buffer_file = pros.fopen(port_buffer_path, "wb");
    defer if (port_buffer_file) |file| {
        _ = pros.fclose(file);
    };

    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);

    // wait a long time
    wait(60000, &odom_state, &port_buffer);

    // write the port buffer to the port_buffer file
    if (port_buffer_file) |file|
        _ = pros.fwrite(@ptrCast(&port_buffer), comptime @bitSizeOf(port.PortBuffer)/8, 1, file);
}

/// Waits a certain amount of time, whilst still updating odom
fn wait(delay_ms: u32, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    var now = pros.rtos.millis();
    while (now / delay_ms < 1) {
        odom_state.update(port_buffer);
        pros.rtos.task_delay_until(&now, tick_delay);
    }
}
