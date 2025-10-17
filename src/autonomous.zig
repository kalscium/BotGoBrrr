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

/// The speed at which auton will drive at
pub const auton_drive_speed: f64 = 0.5;

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
    _ = pros.printf("hello, world from autonomous!\n");

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

    const routine = options.auton_routine orelse "left"; // left is default

    if (comptime std.mem.eql(u8, routine, "left"))
        autonomousLeft(&shadow, &odom_state, &port_buffer)
    else if (comptime std.mem.eql(u8, routine, "left-parked"))
        autonomousLeftParked(&shadow, &odom_state, &port_buffer)
    else if (comptime std.mem.eql(u8, routine, "right"))
        autonomousRight(&shadow, &odom_state, &port_buffer)
    else if (comptime std.mem.eql(u8, routine, "skills"))
        autonomousSkills(&shadow, &odom_state, &port_buffer)
    else
        @compileError("invalid autonomous routine build flag");

    // write the port buffer to the port_buffer file
    if (port_buffer_file) |file|
        _ = pros.fwrite(@ptrCast(&port_buffer), comptime @bitSizeOf(port.PortBuffer)/8, 1, file);
}

pub fn autonomousSkills(shadow: *Shadow, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // just do the normal left match auton
    autonomousLeft(shadow, odom_state, port_buffer);

    // now park

    // move backwards from the long-goal, rotate to face the parking and then full-send it (look at the video it's so cool)
    // also probably safe to keep this here as it's outside of the 15s time limit for matches
    shadow.moveMMPID(-550, odom_state, port_buffer);
    shadow.rotateToDegPID(90, odom_state, port_buffer);
    drive.driveVel(1.0, 1.0, port_buffer);
    wait(1700, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
}

/// An auton for the left side that starts aligned to the parking zone and wall of the field.
pub fn autonomousLeftParked(shadow: *Shadow, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // go forwards to clear the parking (to not hit it when rotating)
    shadow.moveMMPID(600, odom_state, port_buffer);
    
    // rotate towards 3 blocks
    shadow.rotateToDegPID(-30, odom_state, port_buffer);
    tower.storeBlocks(1, port_buffer);

    // move into them slowly for a while to intake
    drive.driveVel(0.35, 0.2, port_buffer);
    wait(1200, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);

    // IT GOES TO MIDDLE GOAL!!! and score
    tower.spin(1, port_buffer);
    wait(1500, odom_state, port_buffer);
    tower.spin(0, port_buffer);

    // move out of middle goal
    drive.driveVel(-0.15, -0.15, port_buffer);
    wait(400, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);

    // go to match loader
    shadow.rotateToDegPID(75, odom_state, port_buffer);
    shadow.moveMMPID(-800, odom_state, port_buffer);
    shadow.rotateToDegPID(180, odom_state, port_buffer);
    _ = pros.adi.adi_digital_write(tower.little_will_port, true);

    // penetrate match-loader
    tower.storeBlocks(1, port_buffer);
    drive.driveVel(0.4, 0.4, port_buffer);
    wait(750, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    
    // stop intake, fly out backwards, disable will
    shadow.moveMMPID(-180, odom_state, port_buffer);
    _ = pros.adi.adi_digital_write(tower.little_will_port, false);
    shadow.rotateToDegPID(0, odom_state, port_buffer);

    // fly into goal, and score
    shadow.moveMMPID(115, odom_state, port_buffer);
    tower.spin(1, port_buffer);
    wait(3000, odom_state, port_buffer);
    tower.spin(0, port_buffer);
}

pub fn autonomousLeft(shadow: *Shadow, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // go to the 3 blocks at a slight angle with intake spinning
    shadow.moveMMPID(230, odom_state, port_buffer);
    shadow.rotateToDegPID(-45, odom_state, port_buffer);
    tower.storeBlocks(1, port_buffer);
    shadow.moveMMPID(415, odom_state, port_buffer);

    // move into them slowly for a while to intake
    drive.driveVel(0.15, 0.1, port_buffer);
    wait(1200, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);

    // go back to where the robot was + a bit
    shadow.moveMMPID(55, odom_state, port_buffer);

    // rotate towards and go to the middle goal and score ~half the blocks
    shadow.rotateToDegPID(45, odom_state, port_buffer);
    shadow.moveMMPID(315, odom_state, port_buffer);
    tower.spin(1, port_buffer);
    wait(500, odom_state, port_buffer);
    tower.spin(0, port_buffer);

    // move backwards diagonally to the corner, and then move fowards to the long goal before scoring the rest of the blocks
    shadow.moveMMPID(-1255, odom_state, port_buffer);
    shadow.rotateToDegPID(0, odom_state, port_buffer);
    shadow.moveMMPID(470, odom_state, port_buffer);
    tower.spin(1, port_buffer);
    wait(1200, odom_state, port_buffer);
    tower.spin(0, port_buffer);
}

pub fn autonomousRight(shadow: *Shadow, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // go to the 3 blocks at a slight angle with intake spinning
    shadow.moveMMPID(230, odom_state, port_buffer);
    shadow.rotateToDegPID(45, odom_state, port_buffer);
    tower.storeBlocks(1, port_buffer);
    shadow.moveMMPID(415, odom_state, port_buffer);

    // move into them slowly for a while to intake
    drive.driveVel(0.1, 0.15, port_buffer);
    wait(1200, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);

    // go back to where the robot was + a bit
    shadow.moveMMPID(55, odom_state, port_buffer);

    // rotate towards and go to the middle goal and score ~half the blocks
    shadow.rotateToDegPID(-45, odom_state, port_buffer);
    shadow.moveMMPID(315, odom_state, port_buffer);
    tower.spin(-1, port_buffer);
    wait(1500, odom_state, port_buffer);
    tower.spin(0, port_buffer);

    // move backwards diagonally to the corner, and then move fowards to the long goal before scoring the rest of the blocks
    shadow.moveMMPID(-1275, odom_state, port_buffer);
    shadow.rotateToDegPID(0, odom_state, port_buffer);
    shadow.moveMMPID(470, odom_state, port_buffer);
    tower.spin(1, port_buffer);
    wait(1200, odom_state, port_buffer);
    tower.spin(0, port_buffer);
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
