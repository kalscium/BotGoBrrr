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
const options = @import("options");

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
    .search_radius = 180.0, // we know 240 works well enough (little osccilation) and 280 is too inaccurate
    .kp = 0.3, // reasonable speed (accurate and fast enough), try 0.5 if you want it to be faster
    .lookahead_window = 10.0, // if it's still pre-maturely stopping just set it to zero
};

export fn autonomous() callconv(.C) void {
    autonomousNew();
    if (true) return; // remove this later
    if (comptime options.auton_routine) |routine|
        if (comptime std.mem.eql(u8, routine, "left"))
            autonomousLeft()
        else if (comptime std.mem.eql(u8, routine, "right"))
            autonomousRight()
        else
            @compileError("invalid autonomous routine build flag");
}

pub fn autonomousNew() void {
    _ = pros.printf("hello, world from autonomous (new side)!\n");
    // open the motor disconnect file
    const port_buffer_file = pros.fopen(port_buffer_path, "wb");
    defer if (port_buffer_file) |file| {
        _ = pros.fclose(file);
    };

    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);

    pid.rotateDeg(-25.0, &odom_state, &port_buffer);
    drive.driveLeft(6000, &port_buffer);
    drive.driveRight(6000, &port_buffer);
    wait(500, &odom_state, &port_buffer);
    drive.driveLeft(0, &port_buffer);
    drive.driveRight(0, &port_buffer);
}

pub fn autonomousLeft() void {
    _ = pros.printf("hello, world from autonomous (left side)!\n");
    // open the motor disconnect file
    const port_buffer_file = pros.fopen(port_buffer_path, "wb");
    defer if (port_buffer_file) |file| {
        _ = pros.fclose(file);
    };

    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);

    // NOTE: ALL COORDINATES AND ANGLES USED ARE ALL PLACEHOLDERS UNTIL I HAVE ACCESS TO THE FIELD

    // move to the long goals and align to them
    pure_pursuit.autonFollowPath(&.{ .{ -767.478678, 335.350953 }, .{ -801.328123, 748.487009 }, .{ -776.244942, 238.132108 } }, false, &odom_state, &port_buffer);
    pid.rotateDeg(0, &odom_state, &port_buffer);
    // score the pre-load by spinning the tower for 1 seconds
    tower.spin(tower.tower_velocity, &port_buffer);
    wait(1000, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);

    // move backwards to get out of the way of the long goal (just remove if doesn't work)
    pure_pursuit.autonFollowPath(&.{ .{ -402.464634, 751.293460 }, }, false, &odom_state, &port_buffer);

    // go to the 3 blocks in front of the centre goals (but do not touch)
    pure_pursuit.autonFollowPath(&.{ .{ -40.844710, 1237.229800 }, }, false, &odom_state, &port_buffer);
    // rotate towards the blocks (at the right angle)
    pid.rotateDeg(135.0, &odom_state, &port_buffer);
    // move forwards for 1 second at a slow speed with the tower active
    tower.spin(tower.tower_velocity, &port_buffer);
    drive.driveLeft(0.2, &port_buffer);
    drive.driveRight(0.2, &port_buffer);
    wait(500, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);
    drive.driveLeft(0, &port_buffer);
    drive.driveRight(0, &port_buffer);

    // drive to the centre goals, align and score
    pure_pursuit.autonFollowPath(&.{ .{ -40.844710, 1237.229800 }, }, false, &odom_state, &port_buffer);
    pid.rotateDeg(45.0, &odom_state, &port_buffer);
    tower.spin(-tower.tower_outtake_vel, &port_buffer);
    wait(1000, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);

    // move backwards to get to a nicer spot (remove if it doesn't work)
    pure_pursuit.autonFollowPath(&.{ .{ -325.859882, 368.488989 } }, false, &odom_state, &port_buffer);

    // write the port buffer to the port_buffer file
    if (port_buffer_file) |file|
        _ = pros.fwrite(@ptrCast(&port_buffer), comptime @bitSizeOf(port.PortBuffer)/8, 1, file);
}

pub fn autonomousRight() void {
    _ = pros.printf("hello, world from autonomous (right side)!\n");
    // open the motor disconnect file
    const port_buffer_file = pros.fopen(port_buffer_path, "wb");
    defer if (port_buffer_file) |file| {
        _ = pros.fclose(file);
    };

    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);

    // NOTE: ALL COORDINATES AND ANGLES USED ARE ALL PLACEHOLDERS UNTIL I HAVE ACCESS TO THE FIELD

    // move to the long goals and align to them
    pure_pursuit.autonFollowPath(&.{ .{ 0, 0 }, .{ 1, 1 }, .{ 2, 2 } }, false, &odom_state, &port_buffer);
    pid.rotateDeg(90, &odom_state, &port_buffer);
    // score the pre-load by spinning the tower for 1 seconds
    tower.spin(tower.tower_velocity, &port_buffer);
    wait(1000, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);

    // move backwards to get out of the way of the long goal (just remove if doesn't work)
    pure_pursuit.autonFollowPath(&.{ .{ 0, 0 } }, true, &odom_state, &port_buffer);

    // go to the 3 blocks in front of the centre goals (but do not touch)
    pure_pursuit.autonFollowPath(&.{ .{ 0, 0 }, .{ 1, 1 }, .{ 2, 2 } }, false, &odom_state, &port_buffer);
    // rotate towards the blocks (at the right angle)
    pid.rotateDeg(135.0, &odom_state, &port_buffer);
    // move forwards for 1 second at a slow speed with the tower active
    tower.spin(tower.tower_velocity, &port_buffer);
    drive.driveLeft(0.2, &port_buffer);
    drive.driveRight(0.2, &port_buffer);
    wait(500, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);
    drive.driveLeft(0, &port_buffer);
    drive.driveRight(0, &port_buffer);

    // drive to the centre goals, align and score
    pure_pursuit.autonFollowPath(&.{ .{ 0, 0 } }, false, &odom_state, &port_buffer);
    pid.rotateDeg(135.0, &odom_state, &port_buffer);
    tower.spin(tower.tower_velocity_down, &port_buffer);
    wait(1000, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);

    // move backwards to get to a nicer spot (remove if it doesn't work)
    pure_pursuit.autonFollowPath(&.{ .{ 0, 0 } }, true, &odom_state, &port_buffer);

    // write the port buffer to the port_buffer file
    if (port_buffer_file) |file|
        _ = pros.fwrite(@ptrCast(&port_buffer), comptime @bitSizeOf(port.PortBuffer)/8, 1, file);
}

/// Waits a certain amount of time, whilst still updating odom
fn wait(delay_ms: u32, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    var now = pros.rtos.millis();
    const start = now;
    while (now - start < delay_ms) {
        odom_state.update(port_buffer);
        pros.rtos.task_delay_until(&now, cycle_delay);
    }
}
