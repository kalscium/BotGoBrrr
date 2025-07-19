//! Code for 'automating' all my tuning woes

const std = @import("std");
const options = @import("options");
const pros = @import("pros");
const auton = @import("autonomous.zig");
const odom = @import("odom.zig");
const port = @import("port.zig");
const logging = @import("logging.zig");
const pid = @import("pid.zig");
const drive = @import("drive.zig");
const major_minor = @import("major_minor.zig");

/// the CSV movement pid log file path
const mov_pid_path = "tuner_mov_pid.csv";

/// the CSV yaw pid log file path
const yaw_pid_path = "tuner_yaw_pid.csv";

/// Tuning entrypoint
pub fn entry(comptime tune: []const u8) void {
    pros.rtos.delay(2500);
    if (comptime std.mem.eql(u8, tune, "mov-pid"))
        tuneMovPID()
    else if (comptime std.mem.eql(u8, tune, "yaw-pid"))
        tuneYawPID()
    else if (comptime std.mem.eql(u8, tune, "mov-min"))
        tuneMinorMov()
    else if (comptime std.mem.eql(u8, tune, "yaw-min"))
        tuneMinorYaw()
    else @compileError("invalid tuning option");
}

/// Tune a movement PID by graphing a step response
pub fn tuneMovPID() void {
    // we'll ignore the x value, as this is only tuning the movement (Y) PID

    // how long to wait before the step (in ms)
    const grace_period = 200;
    // the desired y distance (in mm)
    const step = 609.6; // 2" field tiles

    // open the logging/graph file
    const file = pros.fopen(mov_pid_path, "w");
    defer logging.closeFile(file);
    logging.writeHeader(file, "desired Y (mm),actual Y (mm)\n");

    // loop state
    const start = pros.rtos.millis();
    var now = start;
    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF));
    var odom_state = odom.State.init(&port_buffer);
    var pid_state = pid.State{};

    while (true) {
        // update odom
        odom_state.update(&port_buffer);

        const desired_y: f64 = if ((now - start) / grace_period > 0) step else 0;

        // log data
        if (file) |f|
            _ = pros.fprintf(f, "%lf,%lf\n", desired_y, odom_state.coord[1]);

        // calculate error
        const err = desired_y - odom_state.coord[1];

        // get the drive controls from the PID
        const pv = pid_state.update(auton.mov_pid_param, err, auton.cycle_delay);

        // drive it
        drive.driveLeft(pv, &port_buffer);
        drive.driveRight(pv, &port_buffer);

        // wait till next cycle
        pros.rtos.task_delay_until(&now, auton.cycle_delay);
    }
}

/// Tune a yaw PID by graphing a step response
pub fn tuneYawPID() void {
    // we'll ignore the y value, as this is only tuning the movement (X) PID

    // how long to wait before the step (in ms)
    const grace_period = 200;
    // the desired angle (in radians)
    const step = std.math.degreesToRadians(90);

    // open the logging/graph file
    const file = pros.fopen(yaw_pid_path, "w");
    defer logging.closeFile(file);
    logging.writeHeader(file, "desired yaw (*),actual yaw (*)\n");

    // loop state
    const start = pros.rtos.millis();
    var now = start;
    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF));
    var pid_state = pid.State{};

    while (true) {
        const desired_yaw: f64 = if ((now - start) / grace_period > 0) step else 0;
        const yaw = odom.getYaw(&port_buffer) orelse 0;

        // log data
        if (file) |f|
            _ = pros.fprintf(f, "%lf,%lf\n", std.math.radiansToDegrees(desired_yaw), std.math.radiansToDegrees(yaw));

        // calculate error
        const err = odom.minimalAngleDiff(yaw, desired_yaw);

        // get the drive controls from the PID
        const pv = pid_state.update(auton.yaw_pid_param, err, auton.cycle_delay);

        // drive it
        const ldr, const rdr = drive.arcadeDrive(pv, 0);
        drive.driveLeft(ldr, &port_buffer);
        drive.driveRight(rdr, &port_buffer);

        // wait till next cycle
        pros.rtos.task_delay_until(&now, auton.cycle_delay);
    }
}

/// Tune a minor yaw integral correction by correcting a 20° error while moving
/// a single tile
pub fn tuneMinorYaw() void {
    // no proper logging as this is all done by eye

    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF));
    var odom_state = odom.State.init(&port_buffer);

    // rotate to 20 degrees
    pid.rotate(std.math.degreesToRadians(20), &odom_state, &port_buffer);
    // move one field tile whilst correcting the error
    major_minor.move(.{ 0, 609.6 }, 0, false, &odom_state, &port_buffer);
}

/// Tune a minor movement integral correction by correcting a 10cm error while
/// turning 45°
pub fn tuneMinorMov() void {
    // no proper logging as this is all done by eye

    var port_buffer: port.PortBuffer = @bitCast(@as(u24, 0xFFFFFF));
    var odom_state = odom.State.init(&port_buffer);

    // move backwards 10cm
    pid.move(.{ 0, -10 * 10 }, &odom_state, &port_buffer);
    // rotate 45 degrees whilst correcting the error
    major_minor.rotate(std.math.degreesToRadians(45), .{ 0, 0 }, &odom_state, &port_buffer);
}
