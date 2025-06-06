//! Functions for dealing with motors in zig

const std = @import("std");
const pros = @import("pros");
const def = @import("def.zig");
const mport = @import("port.zig");

/// Returns a motor port that's comptime checked (1-21)
pub fn motorPort(comptime port: comptime_int, comptime reversed: bool) comptime_int {
    _ = mport.checkedPort(port);

    // return the port
    return port * if (reversed) -1 else 1;
}

/// Sets the voltage for a motor.
/// 
/// Returns whether setting the voltage of the motor was successful or not
pub fn setVoltage(comptime port: i8, voltage: i32) bool {
    // move the motor & check for errors
    if (pros.motors.motor_move_voltage(port, voltage) == def.pros_err_i32) {
        // check if it's a motor disconnect
        if (pros.__errno().* == def.pros_error_code.enodev) {
            return false;
        }
    }

    return true;
}
