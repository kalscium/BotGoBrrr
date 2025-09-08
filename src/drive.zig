//! Functions for driving the robot (calculations & drivetrain)

const std = @import("std");
const Motor = @import("Motor.zig");
const port = @import("port.zig");
const pros = @import("pros");
const def = @import("def.zig");
const options = @import("options");
const controller = @import("controller.zig");
const pid = @import("pid.zig");
const opcontrol = @import("opcontrol.zig");
const auton = @import("autonomous.zig");
const vector = @import("vector.zig");
const odom = @import("odom.zig");

/// Driving in reverse toggle button
pub const reverse_toggle: c_int = pros.misc.E_CONTROLLER_DIGITAL_RIGHT;

/// For turning to absolute angles (from start)
pub const absolute_turn_held: c_int = pros.misc.E_CONTROLLER_DIGITAL_DOWN;

/// Drivetrain motor configs
pub const drivetrain_motors = struct {
    // The order for drivetrain motors is as follows,
    // l0 are the motors of the left side of the drivetrain.
    // r0 are the motors of the right side of the drivetrain.
    // x1 are the front motors of the drivetrain
    // x2 are the back motors of the drivetrain
    // x3 are the top motors of the drivetrain
    // 
    // Also, the tower is at the **FRONT** of the robot
    pub const l1 = drivetrainMotor(-16);
    pub const l2 = drivetrainMotor(-10);
    pub const l3 = drivetrainMotor(9);
    pub const r1 = drivetrainMotor(11);
    pub const r2 = drivetrainMotor(2);
    pub const r3 = drivetrainMotor(-1);
};

/// The multiplier applied to the robot's turning & movement speed normally
pub const drive_multiplier = 1.0;
pub const turn_multiplier = 1.0;

pub const DriveState = struct {
    yaw_pid: pid.State = .{},
    reverse: bool = false,
};

/// Reads the controller and updates the drivetrain accordingly based upon the
/// enabled build options
/// 
/// Also takes in whether the robot is currently driving backwards (in reverse)
/// 
/// Updates the port buffer on any motor disconnects
pub fn controllerUpdate(drive_state: *DriveState, port_buffer: *port.PortBuffer) void {
    // hopefully gets set by one of the options
    var ldr: i32 = 0;
    var rdr: i32 = 0;

    if (controller.get_digital(absolute_turn_held)) {
        ldr, rdr = absTurn(drive_state, port_buffer);
    } else {
        // gets the normalized x and y from the right and left joystick
        const x = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_RIGHT_X))) / 127.0;
        const y = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127.0;

        // rest is just normal arcade drive
        ldr, rdr = arcadeDrive(x, y);
    }

    // check for toggling of the reverse toggle
    if (controller.get_digital_new_press(reverse_toggle)) {
        drive_state.reverse = !drive_state.reverse;
        // if toggled on, vibrate long, else short
        _ = if (drive_state.reverse)
            pros.misc.controller_rumble(pros.misc.E_CONTROLLER_MASTER, "-")
        else
            pros.misc.controller_rumble(pros.misc.E_CONTROLLER_MASTER, ".");
    }

    // check for reverse driving toggle
    if (drive_state.reverse) {
        // swap then negate
        const tmp = ldr;
        ldr = -rdr;
        rdr = -tmp;
    }

    // drive the drivetrain
    driveVolt(ldr, rdr, port_buffer);
}

/// Drivetrain default configs (port is negative for reversed)
pub fn drivetrainMotor(comptime mport: comptime_int) Motor {
    return Motor{
        .port = mport,
        .gearset = pros.motors.E_MOTOR_GEAR_BLUE,
        .encoder_units = pros.motors.E_MOTOR_ENCODER_DEGREES,
    };
}

// Initializes the drivetrain (MUST BE RUN AT PROGRAM INIT)
pub fn init() void {
    drivetrain_motors.l1.init();
    drivetrain_motors.l2.init();
    drivetrain_motors.l3.init();
    drivetrain_motors.r1.init();
    drivetrain_motors.r2.init();
    drivetrain_motors.r3.init();
}

/// Turns to an absolute angle (from IMU) relative to it's starting angle
pub fn absTurn(drive_state: *DriveState, port_buffer: *port.PortBuffer) struct { i32, i32 } {
    // gets the normalized y from the left joystick, and the x and y from the right joystick
    const y = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127.0;
    const j2x = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_RIGHT_X))) / 127.0;
    const j2y = @as(f64, @floatFromInt(controller.get_analog(pros.misc.E_CONTROLLER_ANALOG_RIGHT_Y))) / 127.0;

    // get the 'x' value for the turn from the PID (if the right joystick is being used)
    var x: f64 = 0;
    if (j2x > 0 or j2y > 0) {
        const desired = vector.calDir(f64, .{ j2x, j2y }); // calculate the desired angle
        const yaw = odom.getYaw(port_buffer) orelse 0;
        const err = odom.minimalAngleDiff(yaw, desired);
        x = drive_state.yaw_pid.update(auton.yaw_pid_param, err, opcontrol.cycle_delay);
    }

    // pass it through arcade drive
    return arcadeDrive(x, y);
}

/// Converts -1..=1 x & y values into left & right drive voltages
pub fn arcadeDrive(x: f64, y: f64) struct { i32, i32 } {
    // apply the rotation and movement multipliers
    const n_x = x * turn_multiplier;
    const n_y = y * drive_multiplier;

    const ldr = std.math.clamp(n_y + n_x, -1, 1);
    const rdr = std.math.clamp(n_y - n_x, -1, 1);

    return .{ @intFromFloat(ldr * 12000), @intFromFloat(rdr * 12000) };
}

/// Drives the drivetrain based upon the input voltages for left and right,
/// reports any motor disconnects to the port buffer
pub fn driveVolt(ldr: i32, rdr: i32, port_buffer: *port.PortBuffer) void {
    drivetrain_motors.l1.setVoltage(ldr, port_buffer);
    drivetrain_motors.l2.setVoltage(ldr, port_buffer);
    drivetrain_motors.l3.setVoltage(ldr, port_buffer);
    drivetrain_motors.r1.setVoltage(rdr, port_buffer);
    drivetrain_motors.r2.setVoltage(rdr, port_buffer);
    drivetrain_motors.r3.setVoltage(rdr, port_buffer);
}

/// Drives the drivetrain based upon the input velocities for left and right,
/// reports any motor disconnects to the port buffer
pub fn driveVel(ldr: f64, rdr: f64, port_buffer: *port.PortBuffer) void {
    drivetrain_motors.l1.setVelocity(ldr, port_buffer);
    drivetrain_motors.l2.setVelocity(ldr, port_buffer);
    drivetrain_motors.l3.setVelocity(ldr, port_buffer);
    drivetrain_motors.r1.setVelocity(rdr, port_buffer);
    drivetrain_motors.r2.setVelocity(rdr, port_buffer);
    drivetrain_motors.r3.setVelocity(rdr, port_buffer);
}
