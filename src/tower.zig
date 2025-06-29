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
    // The A B C motors of the tower are ordered from top to bottom
    // as they appear on the robot, while D is the motor that's in the back.
    pub const a = towerMotor(20);
    pub const b = towerMotor(18);
    pub const c = towerMotor(17);
    pub const d = towerMotor(11);
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
    if (controller.get_digital(controls.up)) {
        if (controller.get_digital(controls.down)) // if both are hit at the same time, score middle
            spinScoreMiddle(tower_velocity, port_buffer)
        else
            spinScoreHigh(tower_velocity, port_buffer);
    } else if (controller.get_digital(controls.down))
        spinScoreHigh(-tower_velocity, port_buffer)
    else
        spin(0, port_buffer);
}

/// Initializes the tower
pub fn init() void {
    motors.a.init();
    motors.b.init();
    motors.c.init();
    motors.d.init();
}

/// Spins the tower so that it scores the high-goal at an input velocity of `-1..=1`, reporting disconnects to the port buffer
pub fn spinScoreHigh(velocity: f64, port_buffer: *port.PortBuffer) void {
    motors.a.setVelocity(velocity, port_buffer);
    motors.b.setVelocity(-velocity, port_buffer);
    motors.c.setVelocity(-velocity, port_buffer);
    motors.d.setVelocity(-velocity, port_buffer);
}

/// Spins the tower so that it scores the middle-goal at an input velocity of `-1..=1`, reporting disconnects to the port buffer
pub fn spinScoreMiddle(velocity: f64, port_buffer: *port.PortBuffer) void {
    motors.a.setVelocity(0, port_buffer);
    motors.b.setVelocity(velocity, port_buffer);
    motors.c.setVelocity(-velocity, port_buffer);
    motors.d.setVelocity(0, port_buffer);
}

/// Spins all the motors of the tower based on an input velocity `(-1..=1)`, reporting disconnects to the port buffer
pub fn spin(velocity: f64, port_buffer: *port.PortBuffer) void {
    motors.a.setVelocity(velocity, port_buffer);
    motors.b.setVelocity(velocity, port_buffer);
    motors.c.setVelocity(velocity, port_buffer);
    motors.d.setVelocity(velocity, port_buffer);
}
