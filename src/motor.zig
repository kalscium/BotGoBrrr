//! Functions for dealing with motors in zig

const std = @import("std");
const pros = @import("pros");
const def = @import("def.zig");

/// Returns a motor port that's comptime checked (1-21)
pub fn motorPort(comptime port: comptime_int, comptime reversed: bool) comptime_int {
    // check if the port is valid
    if (port < 1 or port > 21) {
        @compileError("motor port must be within the range 1..=21");
    }

    // return the port
    return port * if (reversed) -1 else 1;
}

/// Sets the voltage for a motor.
/// 
/// Returns a motor disconnect error and sets the disconnected motor upon motor
/// disconnect.
pub fn setVoltage(port: i8, voltage: i32, disconnected_motor: *i8) void {
    // move the motor & check for errors
    if (pros.motors.motor_move_voltage(port, voltage) == def.pros_err_i32) {
        // check if it's a motor disconnect
        if (pros.__errno().* == def.pros_error_code.enodev) {
            disconnected_motor.* = port;
        }
    }
}
