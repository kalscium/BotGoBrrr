//! Functions for dealing with the scoring 'tower' motors

const pros = @import("pros");
const Motor = @import("Motor.zig");
const def = @import("def.zig");
const port = @import("port.zig");
const controller = @import("controller.zig");

/// The velocity of the tower spinning up
pub const tower_velocity: f64 = 1.0;
/// The velocity the tower when out-taking (b-down)
pub const tower_outtake_vel: f64 = 1.0;

/// The motor configs
pub const motors = struct {
    // The two motors in the tower
    pub const hood = towerMotor(-5, pros.motors.E_MOTOR_GEAR_200);
    pub const top = towerMotor(4, pros.motors.E_MOTOR_GEAR_200);
    pub const mid = towerMotor(6, pros.motors.E_MOTOR_GEAR_200);
    pub const bottom = towerMotor(18, pros.motors.E_MOTOR_GEAR_200);
};

/// The ADI port of the little will dropping pneumatics
pub const little_will_port = 'A';

/// The tower controller controls
pub const controls = struct {
    /// The button for spinning the tower up
    pub const top_forwards: c_int = pros.misc.E_CONTROLLER_DIGITAL_L2;
    /// The button for spinning the tower up
    pub const bottom_forwards: c_int = pros.misc.E_CONTROLLER_DIGITAL_R1;
    /// The button for spinning the tower up
    pub const top_backwards: c_int = pros.misc.E_CONTROLLER_DIGITAL_L1;
    /// The button for spinning the tower up
    pub const bottom_backwards: c_int = pros.misc.E_CONTROLLER_DIGITAL_L1;
    /// The button for dropping little will
    pub const down_will: c_int = pros.misc.E_CONTROLLER_DIGITAL_B;
    /// The button for dropping little will
    pub const up_will: c_int = pros.misc.E_CONTROLLER_DIGITAL_X;
};

/// Tower motor default configs (port is negative for reversed)
pub fn towerMotor(comptime mport: comptime_int, gearset: pros.motors.motor_gearset_e_t) Motor {
    return Motor{
        .port = mport,
        // we're using 5.5W motors, which have a set RPM of 200
        // source: https://kb.vex.com/hc/en-us/articles/10002101702932-Understanding-V5-Smart-Motor-5-5W-Performance
        .gearset = gearset,
        .encoder_units = pros.motors.E_MOTOR_ENCODER_DEGREES,
    };
}

/// Reads the controller and updates the towers accordingly
/// 
/// Updates the port buffer upon motor disconnects.
pub fn controllerUpdate(port_buffer: *port.PortBuffer) void {
    if (controller.get_digital(controls.top_forwards) and controller.get_digital(controls.top_backwards)) {
        spin(tower_velocity, port_buffer);
    } else {
        if (controller.get_digital(controls.top_forwards))
            storeBlocks(tower_velocity, port_buffer)
        else if (controller.get_digital(controls.top_backwards)) 
            spin(-tower_outtake_vel, port_buffer)
        else 
            spin(0.0, port_buffer);
    }

    if (controller.get_digital(controls.up_will))
        _ = pros.adi.adi_digital_write(little_will_port, true);
    if (controller.get_digital(controls.down_will))
        _ = pros.adi.adi_digital_write(little_will_port, false);
}

/// Initializes the tower
pub fn init() void {
    motors.hood.init();
    motors.top.init();
    motors.mid.init();
    motors.bottom.init();
    _ = pros.adi.adi_port_set_config(little_will_port, pros.adi.E_ADI_DIGITAL_OUT);
}

/// Spins all the motors of the tower based on an input velocity `(-1..=1)` to store (not score) blocks, reporting disconnects to the port buffer
pub fn storeBlocks(velocity: f64, port_buffer: *port.PortBuffer) void {
    motors.hood.setVelocity(-velocity, port_buffer);
    motors.top.setVelocity(velocity, port_buffer);
    motors.mid.setVelocity(velocity, port_buffer);
    motors.bottom.setVelocity(velocity, port_buffer);
}

/// Spins all the motors of the tower based on an input velocity `(-1..=1)`, reporting disconnects to the port buffer
pub fn spin(velocity: f64, port_buffer: *port.PortBuffer) void {
    motors.hood.setVelocity(velocity, port_buffer);
    motors.top.setVelocity(velocity, port_buffer);
    motors.mid.setVelocity(velocity, port_buffer);
    motors.bottom.setVelocity(velocity, port_buffer);
}
