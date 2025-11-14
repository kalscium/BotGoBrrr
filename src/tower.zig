//! Functions for dealing with the scoring 'tower' motors

const pros = @import("pros");
const Motor = @import("Motor.zig");
const def = @import("def.zig");
const port = @import("port.zig");
const controller = @import("controller.zig");

/// The velocity of the tower spinning up
pub const tower_velocity: f64 = 1.0;
/// The velocity of the tower when out-taking
pub const tower_outtake_vel: f64 = 1.0;
/// The velocity of the ttower when parking
pub const tower_park_vel: f64 = 0.08;

/// The motor configs
pub const motors = struct {
    pub const hood = towerMotor(-6, pros.motors.E_MOTOR_GEAR_200);
    pub const top = towerMotor(-5, pros.motors.E_MOTOR_GEAR_200);
    pub const mid = towerMotor(-5, pros.motors.E_MOTOR_GEAR_600);
    pub const bottom = towerMotor(18, pros.motors.E_MOTOR_GEAR_200);
};

/// The tower controller controls
pub const controls = struct {
    /// The button for spinning the tower up
    pub const forwards: c_int = pros.misc.E_CONTROLLER_DIGITAL_R2;
    /// The button for spinning the tower down
    pub const backwards: c_int = pros.misc.E_CONTROLLER_DIGITAL_R1;
    /// The button for toggling little will
    pub const toggle_will: c_int = pros.misc.E_CONTROLLER_DIGITAL_B;
    /// The button for toggling park
    pub const toggle_park: c_int = pros.misc.E_CONTROLLER_DIGITAL_Y;
};

/// The ADI port of the little will dropping pneumatics
pub const little_will_port = 'A';

/// The ADI port of the parking
pub const park_port = 'H';

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

/// The state of the tower
pub const TowerState = struct {
    /// If little will is toggled on
    little_will: bool = false,
    /// If the intake is toggled on
    intake: bool = false,
    /// If the parking is toggled on
    park: bool = false,
};

/// Reads the controller and updates the towers accordingly
/// 
/// Updates the port buffer upon motor disconnects.
pub fn controllerUpdate(state: *TowerState, port_buffer: *port.PortBuffer) void {
    // check for the intake toggle
    if (controller.get_digital_new_press_any(controls.forwards)) {
        state.intake = !state.intake;
        if (state.intake) // rumble when toggled on
            _ = pros.misc.controller_rumble(pros.misc.E_CONTROLLER_MASTER, ".");
    }

    // check for scoring
    if (controller.get_digital_any(controls.forwards) and controller.get_digital_any(controls.backwards)) {
        state.intake = false;
        spin(tower_velocity, port_buffer);
    } else if (controller.get_digital_any(controls.backwards)) { // out-take
        // if the parking button is held down, the spin slowly
        if (controller.get_digital_any(controls.toggle_park))
            spin(-tower_park_vel, port_buffer)
        else
            spin(-tower_outtake_vel, port_buffer);
        state.intake = false;
    } else if (state.intake) { // store
        storeBlocks(tower_velocity, port_buffer);
    } else {
        spin(0.0, port_buffer);
    }

    // check for park
    if (controller.get_digital_new_press_any(controls.toggle_park)) {
        state.park = !state.park;
        // rumble if down
        if (state.park)
            _ = pros.misc.controller_rumble(pros.misc.E_CONTROLLER_MASTER, ".-");
        _ = pros.adi.adi_digital_write(park_port, state.park);
    }
    
    // little will toggles
    if (controller.get_digital_new_press_any(controls.toggle_will)) {
        state.little_will = !state.little_will;
        // rumble if down
        if (state.little_will)
            _ = pros.misc.controller_rumble(pros.misc.E_CONTROLLER_MASTER, "-");
        _ = pros.adi.adi_digital_write(little_will_port, state.little_will);
    }
}

/// Initializes the tower
pub fn init() void {
    motors.hood.init();
    motors.top.init();
    motors.mid.init();
    motors.bottom.init();
    _ = pros.adi.adi_port_set_config(little_will_port, pros.adi.E_ADI_DIGITAL_OUT);
    _ = pros.adi.adi_port_set_config(park_port, pros.adi.E_ADI_DIGITAL_OUT);
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
