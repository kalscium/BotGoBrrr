//! Functions for dealing with the scoring 'tower' motors

const pros = @import("pros");
const Motor = @import("Motor.zig");
const def = @import("def.zig");
const port = @import("port.zig");
const controller = @import("controller.zig");

/// The velocity of the tower spinning up
pub const tower_velocity: f64 = 1.0;
/// The velocity the tower when spinning down
pub const tower_velocity_down: f64 = 0.5;

/// The motor configs
pub const motors = struct {
    // The A B motors of the tower are ordered from top to bottom as
    // they appear
    pub const a = towerMotor(-18);
    pub const b = towerMotor(-6);
};

/// The ADI port of the little will dropping pneumatics
pub const little_will_port = 'A';

/// The tower controller controls
pub const controls = struct {
    /// The button for spinning the tower up
    pub const up: c_int = pros.misc.E_CONTROLLER_DIGITAL_L2;
    /// The button for spinning the tower up
    pub const down: c_int = pros.misc.E_CONTROLLER_DIGITAL_L1;
    /// The button for dropping little will
    pub const down_will: c_int = pros.misc.E_CONTROLLER_DIGITAL_DOWN;
    /// The button for dropping little will
    pub const up_will: c_int = pros.misc.E_CONTROLLER_DIGITAL_UP;
};

/// Tower motor default configs (port is negative for reversed)
pub fn towerMotor(comptime mport: comptime_int) Motor {
    return Motor{
        .port = mport,
        // we're using 5.5W motors, which have a set RPM of 200
        // source: https://kb.vex.com/hc/en-us/articles/10002101702932-Understanding-V5-Smart-Motor-5-5W-Performance
        .gearset = pros.motors.E_MOTOR_GEAR_600,
        .encoder_units = pros.motors.E_MOTOR_ENCODER_DEGREES,
    };
}

/// Reads the controller and updates the towers accordingly
/// 
/// Updates the port buffer upon motor disconnects.
pub fn controllerUpdate(port_buffer: *port.PortBuffer) void {
    if (controller.get_digital(controls.up)) {
        if (controller.get_digital(controls.down)) // if both are hit at the same time, score down at full speed
            scoreMid(tower_velocity, port_buffer)
        else
            spin(tower_velocity, port_buffer);
    } else if (controller.get_digital(controls.down))
        spin(-tower_velocity, port_buffer)
    else
        spin(0, port_buffer);

    if (controller.get_digital(controls.up_will))
        _ = pros.adi.adi_digital_write(little_will_port, true);
    if (controller.get_digital(controls.down_will))
        _ = pros.adi.adi_digital_write(little_will_port, false);
}

/// Initializes the tower
pub fn init() void {
    motors.a.init();
    motors.b.init();
    _ = pros.adi.adi_port_set_config(little_will_port, pros.adi.E_ADI_DIGITAL_OUT);
}

/// Spins all the motors of the tower based on an input velocity `(-1..=1)` to score mid, reporting disconnects to the port buffer
pub fn scoreMid(velocity: f64, port_buffer: *port.PortBuffer) void {
    motors.a.setVelocity(-velocity, port_buffer);
    motors.b.setVelocity(velocity, port_buffer);
}

/// Spins all the motors of the tower based on an input velocity `(-1..=1)`, reporting disconnects to the port buffer
pub fn spin(velocity: f64, port_buffer: *port.PortBuffer) void {
    motors.a.setVelocity(velocity, port_buffer);
    motors.b.setVelocity(velocity, port_buffer);
}
