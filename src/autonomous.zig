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
const Shadow = @import("Shadow.zig");

/// The delay in ms, between each 'cycle' of autonomous (the lower the more precise though less stable)
pub const cycle_delay = 10;

/// The path to the autonomous port buffers file
const port_buffer_path = "/usd/auton_port_buffers.bin";

/// The 'precision' (in mm) that the robot must achieve before moving onto the next path coordinate
pub const precision_mm: f64 = 10;
/// The 'precision' (in radians) that the robot must achieve before moving onto the next path coordinate
pub const precision_rad: f64 = std.math.degreesToRadians(1);

/// The *tuned* movement (Y) PID controller
pub const mov_pid_param = pid.Param {
    .kp = 0.001,
    .ki = 0,
    .kd = 0,
    .saturation = 1,
    .low_pass_a = 0.8,
};

/// The *tuned* yaw (radians) PID controller
pub const yaw_pid_param = pid.Param {
    // 1 / max_error
    .kp = 0.21, // seems to work perfectly
    // proportional alone is enough, due to us setting velocity instead of voltage
    .ki = 0.0,
    .kd = 0,
    .saturation = 1,
    .low_pass_a = 0.8,
};

/// The *tuned* pure pursuit parameters
pub const pure_pursuit_params = pure_pursuit.Parameters{
    .search_radius = 300.0,
    .bounding_radius = 20,
    .kp = 0.4,
    .yaw_limit_deg = 25,
};

export fn autonomous() callconv(.C) void {
    if (comptime options.auton_routine) |routine|
        if (comptime std.mem.eql(u8, routine, "shad"))
            autonomousShad()
        else if (comptime std.mem.eql(u8, routine, "left"))
            autonomousLeft1()
        else if (comptime std.mem.eql(u8, routine, "left_old"))
            autonomousLeft()
        else if (comptime std.mem.eql(u8, routine, "right"))
            autonomousRight()
        else
            @compileError("invalid autonomous routine build flag");
}

pub fn autonomousShad() void {
    _ = pros.printf("hello, world from autonomous (shadow side)!\n");
    // open the motor disconnect file
    const port_buffer_file = pros.fopen(port_buffer_path, "wb");
    defer if (port_buffer_file) |file| {
        _ = pros.fclose(file);
    };
    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);
    var shadow: Shadow = .{};

    // turn on intake and move forwards in a curve towards 3 blocks
    shadow.moveCM(50); // move forwards 50cm before turning
    const c1 = shadow.toCoord();
    tower.storeBlocks(tower.tower_velocity, &port_buffer);
    shadow.rotateToDeg(-20);
    shadow.moveCM(32);
    const c2 = shadow.toCoord();
    shadow.rotateToDeg(20);
    shadow.moveCM(32);
    const c3 = shadow.toCoord();
    shadow.rotateToDeg(45);
    shadow.moveCM(20);
    const c4 = shadow.toCoord();
    pure_pursuit.autonFollowPath(&.{ c1, c2, c3, c4 }, false, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);
}

pub fn autonomousLeft1() void {
    _ = pros.printf("hello, world from autonomous (left side)!\n");
    // open the motor disconnect file
    const port_buffer_file = pros.fopen(port_buffer_path, "wb");
    defer if (port_buffer_file) |file| {
        _ = pros.fclose(file);
    };

    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF)); // assume all ports are connected/working initially
    var odom_state = odom.State.init(&port_buffer);
    var shadow: Shadow = .{};

    // do not collide with akibot
    if (options.w_akibot)
        wait(2000, &odom_state, &port_buffer);

    // go to the 3 blocks at a slight angle with intake spinnign
    shadow.moveMMPID(300, &odom_state, &port_buffer);
    shadow.rotateToDegPID(-45, &odom_state, &port_buffer);
    tower.storeBlocks(1, &port_buffer);
    shadow.moveMMPID(400, &odom_state, &port_buffer);

    // move into them slowly for a while to intake
    drive.driveVel(0.1, 0.1, &port_buffer);
    wait(1200, &odom_state, &port_buffer);
    drive.driveVel(0, 0, &port_buffer);

    // go back to where the robot was + a bit
    shadow.moveMMPID(125, &odom_state, &port_buffer);

    // rotate towards and go to the middle goal and score ~half the blocks
    shadow.rotateToDegPID(45, &odom_state, &port_buffer);
    shadow.moveMMPID(420, &odom_state, &port_buffer);
    tower.spin(1, &port_buffer);
    wait(500, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);

    // move backwards diagonally to the corner, and then move fowards to the long goal before scoring the rest of the blocks
    shadow.moveMMPID(-1270, &odom_state, &port_buffer);
    shadow.rotateToDegPID(0, &odom_state, &port_buffer);
    shadow.moveMMPID(560, &odom_state, &port_buffer);
    tower.spin(1, &port_buffer);
    wait(1000, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);

    // write the port buffer to the port_buffer file
    if (port_buffer_file) |file|
        _ = pros.fwrite(@ptrCast(&port_buffer), comptime @bitSizeOf(port.PortBuffer)/8, 1, file);
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
    var shadow: Shadow = .{};

    // go to the 3 blocks at a slight angle with intake spinnign
    shadow.moveMMPID(200, &odom_state, &port_buffer);
    shadow.rotateToDegPID(-7, &odom_state, &port_buffer);
    tower.storeBlocks(1, &port_buffer);
    shadow.moveMMPID(810, &odom_state, &port_buffer);

    // move into them slowly for a while to intake
    drive.driveVel(0.3, 0.3, &port_buffer);
    wait(600, &odom_state, &port_buffer);
    drive.driveVel(0, 0, &port_buffer);

    // go back to where the robot was
    shadow.moveMMPID(75, &odom_state, &port_buffer);

    // rotate towards and go to the middle goal and score ~half the blocks
    shadow.rotateToDegPID(45, &odom_state, &port_buffer);
    shadow.moveMMPID(420, &odom_state, &port_buffer);
    tower.spin(1, &port_buffer);
    wait(500, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);

    // move backwards diagonally to the corner, and then move fowards to the long goal before scoring the rest of the blocks
    shadow.moveMMPID(-1270, &odom_state, &port_buffer);
    shadow.rotateToDegPID(0, &odom_state, &port_buffer);
    shadow.moveMMPID(620, &odom_state, &port_buffer);
    tower.spin(1, &port_buffer);
    wait(1000, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);

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
    drive.driveVel(0.2, 0.2, &port_buffer);
    wait(500, &odom_state, &port_buffer);
    tower.spin(0, &port_buffer);
    drive.driveVel(0, 0, &port_buffer);

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
