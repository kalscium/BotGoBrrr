//! Functions for dealing with the scoring 'tower' motors

const pros = @import("pros");
const Motor = @import("Motor.zig");
const def = @import("def.zig");
const port = @import("port.zig");
const controller = @import("controller.zig");

/// The percentage of the tower's velocity (0..=1)
pub const tower_velocity: f64 = 100 / 100;

/// The motor configs
pub const motors = struct {
    pub const top = towerMotor(9);
    pub const bottom = towerMotor(10);
};

/// The tower controller controls
pub const controls = struct {
    /// The button for spinning the tower up
    pub const up: c_int = pros.misc.E_CONTROLLER_DIGITAL_L1;
    /// The button for spinning the tower up
    pub const down: c_int = pros.misc.E_CONTROLLER_DIGITAL_L2;
};

/// Tower motor default configs (port is negative for reversed)
pub fn towerMotor(comptime mport: comptime_int) Motor {
    return Motor{
        .port = mport,
        // we're using 5.5W motors, which have a set RPM of 200
        // source: https://kb.vex.com/hc/en-us/articles/10002101702932-Understanding-V5-Smart-Motor-5-5W-Performance
        .gearset = pros.motors.E_MOTOR_GEAR_200,
        .encoder_units = pros.motors.E_MOTOR_ENCODER_DEGREES,
    };
}

/// Reads the controller and updates the towers accordingly
/// 
/// Updates the port buffer upon motor disconnects.
pub fn controllerUpdate(port_buffer: *port.PortBuffer) void {
    if (controller.get_digital(controls.up))
        spin(tower_velocity, port_buffer)
    else if (controller.get_digital(controls.down))
        spin(-tower_velocity, port_buffer)
    else
        spin(0, port_buffer);
}

/// Initializes the tower
pub fn init() void {
    motors.top.init();
    motors.bottom.init();
}

/// Spins the tower based on an input velocity `(-1..=1)`, reporting disconnects to the port buffer
pub fn spin(velocity: f64, port_buffer: *port.PortBuffer) void {
    motors.top.setVelocity(velocity, port_buffer);
    motors.bottom.setVelocity(velocity, port_buffer);
}
