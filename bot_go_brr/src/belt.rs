//! Functions for controlling the belt of the robot

use logic::{info, warn};
use safe_vex::{controller::{self, Controller}, motor};
use crate::config;

/// Spins the belt based on user input and returns if it's spinning or not and the direction of that spin
pub fn user_control() {
    // grab the boolean activations of the bytecode 
    let up = controller::get_digital(Controller::Master, config::controls::BELT_UP);
    let down = controller::get_digital(Controller::Master, config::controls::BELT_DOWN);

    // if the up button is hit, then spin the belt up
    if let Ok(true) = up {
        return inst_control(config::motors::BELT_VOLTS);
    }
    
    // if the down button is hit, then spin the belt downwards
    if let Ok(true) = down {
        return inst_control(-config::motors::BELT_VOLTS);
    }

    // if there are no belt buttons being hit, make the belt stop
    inst_control(0);
}

/// Spins the belt based on the voltage in millivolts
pub fn inst_control(voltage: i32) {
    info!("belt voltage: {voltage}");

    // spin the belt motor and log any errors
    if let Err(err) = motor::move_voltage(
        config::motors::BELT.port,
        config::motors::BELT.reverse,
        voltage,
    ) {
        warn!("`PROSErr` occured while setting motor voltage for belt at port {}: {err:?}", config::motors::BELT.port as u8);
    }

    // also spin the intake motor and log any errors
    if let Err(err) = motor::move_voltage(
        config::motors::INTAKE.port,
        config::motors::INTAKE.reverse,
        voltage,
    ) {
        warn!("`PROSErr` occured while setting motor voltage for belt at port {}: {err:?}", config::motors::INTAKE.port as u8);
    }
}
