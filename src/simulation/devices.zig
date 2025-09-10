//! Sensor API function implementations for the simulation

const std = @import("std");
const pros = @import("pros");
const simulation = @import("../simulation.zig");
const configs = @import("configs.zig");

/// Does ✨ nothing ✨
/// (could maybe change the offset between sensor reading and acutal yaw but I can't be asked)
export fn imu_reset() callconv(.C) c_int {
    return 0;
}

/// Does ✨ nothing ✨
/// (would do something if I actually read the motor encoder values)
export fn motor_set_encoder_units() callconv(.C) c_int {
    return 0;
}

/// Sets the gearset of the specified motor
export fn motor_set_gearing(port: i8, gearset: pros.motors.motor_gearset_e_t) callconv(.C) c_int {
    simulation.sim_state.motor_gearsets[@intCast(@abs(port))] = switch (gearset) {
        pros.motors.E_MOTOR_GEAR_100 => 100,
        pros.motors.E_MOTOR_GEAR_200 => 200,
        pros.motors.E_MOTOR_GEAR_600 => 600,
        else => std.debug.panic("invalid motor gearset '{}'", .{gearset}),
    };

    return 0;
}

/// Does ✨ nothing ✨, as the adi port will always be output 'analog' as it
/// works for digital values aswell, and the input would need a different
/// build of robot.
export fn adi_port_set_config() callconv(.C) c_int {
    return 0;
}

/// Does ✨ nothing ✨
/// Perfect simulation rotation sensors would never need to be reversed.
export fn rotation_set_reversed(_: u8, _: bool) callconv(.C) c_int {
    return 0;
}

/// Returns either the lateral or vertical rotation sensor reading in centi-degrees
export fn rotation_get_angle(port: u8) callconv(.C) i32 {
    const radians = switch (port) {
        configs.odom_ver_rot_port => simulation.sim_state.odom_rot_ver_angle,
        configs.odom_lat_rot_port => simulation.sim_state.odom_rot_lat_angle,
        else => std.debug.panic("unknown rotation sensor port {}", .{port}),
    };

    const centi_degrees = std.math.radiansToDegrees(radians) * 100;
    return @intFromFloat(centi_degrees);
}

/// Returns the yaw of the 'imu sensor'
export fn imu_get_yaw(_: u8) callconv(.C) f64 {
    return std.math.radiansToDegrees(simulation.sim_state.yaw);
}  

/// Moves a motor with the specified velocity (has extra effect for drivetrain motors)
export fn motor_move_velocity(port: i8, rpm: i32) callconv(.C) c_int {
    const uport: usize = @intCast(@abs(port));
    const gearset: f64 = @floatFromInt(simulation.sim_state.motor_gearsets[uport]);

    const velocity = @as(f64, @floatFromInt(rpm)) / gearset;
    simulation.sim_state.motor_velocities[uport] = velocity;

    switch (port) {
        configs.drive_left_port => simulation.sim_state.drive_left_vel = velocity,
        configs.drive_right_port => simulation.sim_state.drive_right_vel = velocity,
        else => {},
    }

    return 0;
} 

/// Moves a motor with the specified voltage (has extra effect for drivetrain motors)
export fn motor_move_voltage(port: i8, voltage: i32) callconv(.C) c_int {
    const uport: usize = @intCast(@abs(port));
    const velocity = @as(f64, @floatFromInt(voltage)) / 12000.0;
    simulation.sim_state.motor_velocities[uport] = velocity;

    switch (port) {
        configs.drive_left_port => simulation.sim_state.drive_left_vel = velocity,
        configs.drive_right_port => simulation.sim_state.drive_right_vel = velocity,
        else => {},
    }

    return 0;
} 

/// Gets the simulation's waited time in milliseconds
export fn millis() callconv(.C) u32 {
    return simulation.sim_state.time;
}

/// Updates the waited time, and performs all the kinematics and calculations
/// that would've happened in that time frame.
export fn task_delay_until(prev_time: *u32, delta_time: u32) callconv(.C) void {
    simulation.sim_state.time += delta_time;
    prev_time.* = simulation.sim_state.time;

    if (prev_time.* % 1000 == 0)
        std.debug.print("x: {d}, y: {d}, yaw: {d}\n", .{simulation.sim_state.x, simulation.sim_state.y, std.math.radiansToDegrees(simulation.sim_state.yaw)});
    simulation.sim_state.forwardKinematics(@floatFromInt(delta_time));
    simulation.sim_state.log() catch unreachable;
}
