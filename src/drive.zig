//! Functions for driving the robot (calculations & drivetrain)

const std = @import("std");
const motor = @import("motor.zig");
const port = @import("port.zig");
const pros = @import("pros");
const def = @import("def.zig");

/// Daniel's magic number for nice, smooth and exponential controls
/// 
/// `f(x) = 1024a**x - 1024`
pub const DMN: f32 = 12.71875;

/// Drivetrain default configs (port is negative for reversed)
pub fn drivetrainMotor(comptime mport: comptime_int) motor.Config {
    return motor.Config{
        .port = mport,
        .gearset = pros.motors.E_MOTOR_GEAR_BLUE,
        .encoder_units = pros.motors.E_MOTOR_ENCODER_DEGREES,
    };
}

/// Drivetrain motor configs
pub const drivetrain_motors = struct {
    pub const l1 = drivetrainMotor(11);
    pub const l2 = drivetrainMotor(-12);
    pub const l3 = drivetrainMotor(13);
    pub const r1 = drivetrainMotor(-18);
    pub const r2 = drivetrainMotor(19);
    pub const r3 = drivetrainMotor(-20);
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

/// Amounts of drive-motors on each side of the robot
pub const drive_mtr_side_cnt = 3;

/// Drives the drivetrain side based upon the input voltages, reports any motor
/// disconnects to the port buffer
pub fn driveLeft(voltage: i32, port_buffer: *port.PortBuffer) void {
    port_buffer.portWrite(drivetrain_motors.l1.port, motor.setVoltage(drivetrain_motors.l1.port, voltage));
    port_buffer.portWrite(drivetrain_motors.l2.port, motor.setVoltage(drivetrain_motors.l2.port, voltage));
    port_buffer.portWrite(drivetrain_motors.l3.port, motor.setVoltage(drivetrain_motors.l3.port, voltage));
}

/// Drives the drivetrain side based upon the input voltages
/// 
/// Disconnect buffer is a buffer of disconnected motor ports, 0s are ignored
pub fn driveRight(voltage: i32, port_buffer: *port.PortBuffer) void {
    port_buffer.portWrite(drivetrain_motors.r1.port, motor.setVoltage(drivetrain_motors.r1.port, voltage));
    port_buffer.portWrite(drivetrain_motors.r2.port, motor.setVoltage(drivetrain_motors.r2.port, voltage));
    port_buffer.portWrite(drivetrain_motors.r3.port, motor.setVoltage(drivetrain_motors.r3.port, voltage));
}
