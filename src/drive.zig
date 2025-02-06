//! Functions for driving the robot (calculations & drivetrain)

const std = @import("std");
const motor = @import("motor.zig");
const port = @import("port.zig");

/// Daniel's magic number for nice, smooth and exponential controls
/// 
/// `f(x) = 1024a**x - 1024`
pub const DMN: f32 = 12.71875;

/// Drivetrain ports
pub const drivetrain_ports = struct {
    const l1 = motor.motorPort(12, true);
    const l2 = motor.motorPort(12, true);
    const l3 = motor.motorPort(12, true);
    const r1 = motor.motorPort(12, true);
    const r2 = motor.motorPort(12, true);
    const r3 = motor.motorPort(12, true);
};

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
pub fn arcadeDrive(x: f32, y: f32) struct { f32, f32 } {
    const ldr = std.math.clamp(y + x, -1, 1) * 12000;
    const rdr = std.math.clamp(y - x, -1, 1) * 12000;

    return .{ ldr, rdr };
}

/// Amounts of drive-motors on each side of the robot
pub const drive_mtr_side_cnt = 3;

/// Drives the drivetrain side based upon the input voltages, reports any motor
/// disconnects to the port buffer
pub fn driveLeft(voltage: i32, port_buffer: *port.PortBuffer) void {
    port_buffer.portWrite(drivetrain_ports.l1, motor.setVoltage(drivetrain_ports.l1, voltage));
    port_buffer.portWrite(drivetrain_ports.l2, motor.setVoltage(drivetrain_ports.l2, voltage));
    port_buffer.portWrite(drivetrain_ports.l3, motor.setVoltage(drivetrain_ports.l3, voltage));
}

/// Drives the drivetrain side based upon the input voltages
/// 
/// Disconnect buffer is a buffer of disconnected motor ports, 0s are ignored
pub fn driveRight(voltage: i32, port_buffer: *port.PortBuffer) void {
    port_buffer.portWrite(drivetrain_ports.r1, motor.setVoltage(drivetrain_ports.r1, voltage));
    port_buffer.portWrite(drivetrain_ports.r2, motor.setVoltage(drivetrain_ports.r2, voltage));
    port_buffer.portWrite(drivetrain_ports.r3, motor.setVoltage(drivetrain_ports.r3, voltage));
}
