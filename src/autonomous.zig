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
pub const precision_mm: f64 = 4;
/// The 'precision' (in radians) that the robot must achieve before moving onto the next path coordinate
pub const precision_rad: f64 = std.math.degreesToRadians(1);

/// The speed at which auton will drive at
pub const auton_drive_speed: f64 = 0.5;

/// The *tuned* movement (Y) PID controller
pub const mov_pid_param = pid.Param {
    .kp = 0.0012,
    // .kp = 0.0015,
    .ki = 0,
    .kd = 0,
    .saturation = 1,
    .low_pass_a = 0.8,
};

/// The *tuned* yaw (radians) PID controller
pub const yaw_pid_param = pid.Param {
    // 1 / max_error
    .kp = 0.22, // seems to work perfectly
    //.kp = 0.27,
    // .kp = 1.0 / (std.math.pi / 1.0), // 180 diff full speed
    // proportional alone is enough, due to us setting velocity instead of voltage
    .ki = 0,
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

    if (comptime std.mem.eql(u8, routine, "right"))
        autonomousRight(&shadow, &odom_state, &port_buffer)
    else if (comptime std.mem.eql(u8, routine, "left-pland"))
        autonomousLeftPland(&shadow, &odom_state, &port_buffer)
    else if (comptime std.mem.eql(u8, routine, "park-bank"))
        pros.adi.adi_digital_write(tower.park_port, true)
    else
        @compileError("invalid autonomous routine build flag");

    // write the port buffer to the port_buffer file
    if (port_buffer_file) |file|
        _ = pros.fwrite(@ptrCast(&port_buffer), comptime @bitSizeOf(port.PortBuffer)/8, 1, file);
}

pub fn autonomousSkills(shadow: *Shadow, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // just do the normal left match auton
    autonomousLeftPland(shadow, odom_state, port_buffer);

    // now park
}

/// An auton for the left side that starts aligned to the parking zone and wall of the field.
pub fn autonomousLeftPland(shadow: *Shadow, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // robot starts on the left-front side of the parking

    // OBJECTIVE: grab the 3 blocks near mid

    // moves forwards and rotates towards the 3 blocks near mid
    drive.driveVel(0.25, 0.25, port_buffer);
    wait(600, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    shadow.rotateToDegPID(-50, odom_state, port_buffer);

    // activates intake, moves towards them slowly,
    // once in range, engages little will to hold them there
    // and then continues to move forwards to intake
    //
    // works PERFECTLY every time
    tower.storeBlocks(1, port_buffer);
    drive.driveVel(0.25, 0.25, port_buffer);
    wait(500, odom_state, port_buffer);
    _ = pros.adi.adi_digital_write(tower.little_will_port, true);
    wait(800, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);

    // OBJECTIVE: go to middle and score one block
    drive.driveVel(-0.25, -0.25, port_buffer);
    // wait(170, odom_state, port_buffer);
    wait(135, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    shadow.rotateToDegPID(46.5, odom_state, port_buffer);
    drive.driveVel(0.25, 0.25, port_buffer);
    wait(250, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    tower.spin(0.5, port_buffer);
    wait(800, odom_state, port_buffer);
    tower.spin(0, port_buffer);
    shadow.rotateToDegPID(46, odom_state, port_buffer);

    // move back and align with match matchloader
    drive.driveVel(-0.4, -0.4, port_buffer);
    wait(1350, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    shadow.rotateToDegPID(179, odom_state, port_buffer);
    // if (true) return;
    drive.driveVel(0.4, 0.4, port_buffer);
    wait(300, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);

    // intake and align with long goal
    tower.storeBlocks(1, port_buffer);
    wait(2000, odom_state, port_buffer);

    // turn around, align and score
    drive.driveVel(-0.4, -0.4, port_buffer);
    wait(400, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    _ = pros.adi.adi_digital_write(tower.little_will_port, false);
    shadow.rotateToDegPID(0, odom_state, port_buffer);
    drive.driveVel(0.25, 0.25, port_buffer);
    wait(310, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    tower.spin(5, port_buffer);
    wait(2000, odom_state, port_buffer);

    // turn around again and lineup for match loading
    tower.storeBlocks(1, port_buffer);
    drive.driveVel(-0.25, -0.25, port_buffer);
    wait(310, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    shadow.rotateToDegPID(180, odom_state, port_buffer);
     _ = pros.adi.adi_digital_write(tower.little_will_port, true);
    drive.driveVel(0.4, 0.4, port_buffer);
    wait(400, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
}

pub fn autonomousRight(shadow: *Shadow, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    // robot starts on the left-front side of the parking

    // OBJECTIVE: grab the 3 blocks near mid

    // moves forwards and rotates towards the 3 blocks near mid
    drive.driveVel(0.25, 0.25, port_buffer);
    wait(600, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    shadow.rotateToDegPID(50, odom_state, port_buffer);

    // activates intake, moves towards them slowly,
    // once in range, engages little will to hold them there
    // and then continues to move forwards to intake
    //
    // works PERFECTLY every time
    tower.storeBlocks(1, port_buffer);
    drive.driveVel(0.25, 0.25, port_buffer);
    wait(500, odom_state, port_buffer);
    _ = pros.adi.adi_digital_write(tower.little_will_port, true);
    wait(800, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
       _ = pros.adi.adi_digital_write(tower.little_will_port, false);

    // OBJECTIVE: go to middle and score one block
    drive.driveVel(-0.25, -0.25, port_buffer);
    wait(180, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    shadow.rotateToDegPID(-46, odom_state, port_buffer);
   drive.driveVel(0.25, 0.25, port_buffer);
    wait(100, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    tower.spin(-1, port_buffer);
    wait(1200, odom_state, port_buffer);
    tower.spin(0, port_buffer);
    shadow.rotateToDegPID(-47, odom_state, port_buffer);

     if (true) return;

    // move back and align with match matchloader
    drive.driveVel(-0.4, -0.4, port_buffer);
    wait(1000, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    shadow.rotateToDegPID(-179, odom_state, port_buffer);
    drive.driveVel(0.4, 0.4, port_buffer);
    wait(300, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);

    // intake and align with long goal
    tower.storeBlocks(1, port_buffer);
    wait(2000, odom_state, port_buffer);

    // turn around, align and score
    drive.driveVel(-0.4, -0.4, port_buffer);
    wait(400, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    _ = pros.adi.adi_digital_write(tower.little_will_port, false);
    shadow.rotateToDegPID(5, odom_state, port_buffer);
    drive.driveVel(0.25, 0.25, port_buffer);
    wait(310, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    tower.spin(5, port_buffer);
    wait(2000, odom_state, port_buffer);

    // turn around again and lineup for match loading
    tower.storeBlocks(1, port_buffer);
    drive.driveVel(-0.25, -0.25, port_buffer);
    wait(310, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
    shadow.rotateToDegPID(180, odom_state, port_buffer);
     _ = pros.adi.adi_digital_write(tower.little_will_port, true);
    drive.driveVel(0.4, 0.4, port_buffer);
    wait(400, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
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

/// Drives for a certain amount of time at a set speed before stopping
fn driveVelFor(speed: f64, time_ms: u32, odom_state: *odom.State, port_buffer: *port.PortBuffer) void {
    drive.driveVel(speed, speed, port_buffer);
    wait(time_ms, odom_state, port_buffer);
    drive.driveVel(0, 0, port_buffer);
}
