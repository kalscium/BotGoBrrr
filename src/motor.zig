//! Functions for dealing with motors in zig

const std = @import("std");
const pros = @import("pros");
const def = @import("def.zig");
const mport = @import("port.zig");

/// Stores the configs for a motor
pub const Config = struct {
    /// The port of the motor
    port: comptime_int,
    /// The gearset of the motor
    gearset: pros.motors.motor_gearset_e_t,
    /// The encoder units of the motor
    encoder_units: pros.motors.motor_encoder_units_e_t,

    /// Configures/Initializes the motor with PROS (should ALWAYS be run on program init)
    pub fn init(self: Config) void {
        _ = pros.motors.motor_set_encoder_units(self.port, self.encoder_units);
        _ = pros.motors.motor_set_gearing(self.port, self.gearset);
    }
};

/// Sets the voltage for a motor.
/// 
/// Returns whether setting the voltage of the motor was successful or not
pub fn setVoltage(comptime port: i8, voltage: i32) bool {
    // move the motor & check for errors
    if (pros.motors.motor_move_voltage(port, voltage) == def.pros_err_i32 and pros.__errno().* == def.pros_error_code.enodev)
        return false
    else
        return true;
}
