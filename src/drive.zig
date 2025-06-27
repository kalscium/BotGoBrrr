//! Functions for driving the robot (calculations & drivetrain)

const std = @import("std");
const Motor = @import("Motor.zig");
const port = @import("port.zig");
const pros = @import("pros");
const def = @import("def.zig");
const options = @import("options");

/// Daniel's magic number for nice, smooth and exponential controls
/// 
/// `f(x) = 1024a**x - 1024`
pub const DMN: f32 = 12.71875;

/// Reads the controller and updates the drivetrain accordingly based upon the
/// enabled build options
/// 
/// Updates the port buffer on any motor disconnects
pub fn controllerUpdate(port_buffer: *port.PortBuffer) void {
    // hopefully gets set by one of the options
    var ldr: i32 = 0;
    var rdr: i32 = 0;

    if (options.arcade) {
        // get the normalized main joystick values
        const jx = @as(f32, @floatFromInt(pros.misc.controller_get_analog(pros.misc.E_CONTROLLER_MASTER, pros.misc.E_CONTROLLER_ANALOG_LEFT_X))) / 127;
        const jy = @as(f32, @floatFromInt(pros.misc.controller_get_analog(pros.misc.E_CONTROLLER_MASTER, pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127;
        ldr, rdr = arcadeDrive(jx, jy);
    } else if (options.split_arcade) {
        // get the normalized main joystick values
        const j1 = @as(f32, @floatFromInt(pros.misc.controller_get_analog(pros.misc.E_CONTROLLER_MASTER, pros.misc.E_CONTROLLER_ANALOG_LEFT_X))) / 127;
        const j2 = @as(f32, @floatFromInt(pros.misc.controller_get_analog(pros.misc.E_CONTROLLER_MASTER, pros.misc.E_CONTROLLER_ANALOG_RIGHT_Y))) / 127;
        ldr, rdr = arcadeDrive(j1, j2);
    } else {
        // get the normalized main joystick values
        const j1 = @as(f32, @floatFromInt(pros.misc.controller_get_analog(pros.misc.E_CONTROLLER_MASTER, pros.misc.E_CONTROLLER_ANALOG_LEFT_Y))) / 127;
        const j2 = @as(f32, @floatFromInt(pros.misc.controller_get_analog(pros.misc.E_CONTROLLER_MASTER, pros.misc.E_CONTROLLER_ANALOG_RIGHT_Y))) / 127;

        // just do a simple tank drive
        ldr = @intFromFloat(j1 * 12000);
        rdr = @intFromFloat(j2 * 12000);
    }

    // drive the drivetrain
    driveLeft(ldr, port_buffer);
    driveRight(rdr, port_buffer);
}

/// Drivetrain default configs (port is negative for reversed)
pub fn drivetrainMotor(comptime mport: comptime_int) Motor {
    return Motor{
        .port = mport,
        .gearset = pros.motors.E_MOTOR_GEAR_BLUE,
        .encoder_units = pros.motors.E_MOTOR_ENCODER_DEGREES,
    };
}

/// Drivetrain motor configs
pub const drivetrain_motors = struct {
    pub const l1 = drivetrainMotor(12);
    pub const l2 = drivetrainMotor(-12);
    pub const l3 = drivetrainMotor(12);
    pub const r1 = drivetrainMotor(-12);
    pub const r2 = drivetrainMotor(12);
    pub const r3 = drivetrainMotor(-12);
};

// Initializes the drivetrain (MUST BE RUN AT PROGRAM INIT)
pub fn init() void {
    drivetrain_motors.l1.init();
    drivetrain_motors.l2.init();
    drivetrain_motors.l3.init();
    drivetrain_motors.r1.init();
    drivetrain_motors.r2.init();
    drivetrain_motors.r3.init();
}

/// Passes a normalized value through daniel's algorithm to produce an exponential voltage
pub fn expDaniel(x: f32) f32 {
    return std.math.copysign(1024 * std.math.pow(f32, DMN, @abs(x)) - 1024, x);
}

test expDaniel {
    std.debug.assert(expDaniel(1) == 12000);
    std.debug.assert(expDaniel(0) == 0);
    std.debug.assert(expDaniel(-1) == -12000);
}

/// Converts normalized x & y values into left & right voltages
pub fn arcadeDrive(x: f32, y: f32) struct { i32, i32 } {
    const ldr = std.math.clamp(y + x, -1, 1) * 12000;
    const rdr = std.math.clamp(y - x, -1, 1) * 12000;

    return .{ @intFromFloat(ldr), @intFromFloat(rdr) };
}

/// Drives the drivetrain side based upon the input voltages, reports any motor
/// disconnects to the port buffer
pub fn driveLeft(voltage: i32, port_buffer: *port.PortBuffer) void {
    drivetrain_motors.l1.setVoltage(voltage, port_buffer);
    drivetrain_motors.l2.setVoltage(voltage, port_buffer);
    drivetrain_motors.l3.setVoltage(voltage, port_buffer);
}

/// Drives the drivetrain side based upon the input voltages
/// 
/// Disconnect buffer is a buffer of disconnected motor ports, 0s are ignored
pub fn driveRight(voltage: i32, port_buffer: *port.PortBuffer) void {
    drivetrain_motors.r1.setVoltage(voltage, port_buffer);
    drivetrain_motors.r2.setVoltage(voltage, port_buffer);
    drivetrain_motors.r3.setVoltage(voltage, port_buffer);
}
