//! Stores the configs for a motor, alongside common functions for dealing
//! with motors

const std = @import("std");
const pros = @import("pros");
const def = @import("def.zig");
const mport = @import("port.zig");

/// The port of the motor
port: comptime_int,
/// The gearset of the motor
gearset: pros.motors.motor_gearset_e_t,
/// The encoder units of the motor
encoder_units: pros.motors.motor_encoder_units_e_t,

/// Configures/Initializes the motor with PROS (should ALWAYS be run on program init)
pub fn init(self: @This()) void {
    _ = pros.motors.motor_set_encoder_units(self.port, self.encoder_units);
    _ = pros.motors.motor_set_gearing(self.port, self.gearset);
}

/// Sets the voltage for a motor.
/// 
/// Updates the port buffer if there are any disconnects
pub fn setVoltage(comptime self: @This(), voltage: i32, port_buffer: *mport.PortBuffer) void {
    // move the motor & check for errors
    if (pros.motors.motor_move_voltage(self.port, voltage) == def.pros_err_i32 and pros.__errno().* == def.pros_error_code.enodev)
        port_buffer.portWrite(self.port, false)
    else
        port_buffer.portWrite(self.port, true);
}

/// Sets the target velocity of a motor.
/// 
/// `velocity`: The new motor velocity from `-1..=1` with 1 being the max speed
/// for the specified motor gearset
/// 
/// Updates the port buffer if there are any disconnects
pub fn setVelocity(comptime self: @This(), velocity: f64, port_buffer: *mport.PortBuffer) void {
    // calculate the velocity in rpm based upon the motor gearset
    const rpm: i32 = @intFromFloat(@round(switch (comptime self.gearset) {
        0 => 100,
        1 => 200,
        2 => 600,
        else => @compileError("invalid motor gearset"),
    } * velocity));

    // move the motor & check for errors
    if (pros.motors.motor_move_velocity(self.port, rpm) == def.pros_err_i32 and pros.__errno().* == def.pros_error_code.enodev)
        port_buffer.portWrite(self.port, false)
    else
        port_buffer.portWrite(self.port, true);
}
