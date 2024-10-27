//! Functions for controlling the belt of the robot

use safe_vex::{controller::{self, Controller}, motor};
use crate::config;

/// Spins the belt based on user input and returns if it's spinning or not and the direction of that spin
pub fn user_control() -> Option<bool> {
    // grab the boolean activations of the bytecode 
    let up = controller::get_digital(Controller::Master, config::controls::BELT_UP);
    let down = controller::get_digital(Controller::Master, config::controls::BELT_DOWN);

    // if the up button is hit, then spin the belt up
    if let Ok(true) = up {
        inst_control(true, true);
        return Some(true);
    }
    
    // if the down button is hit, then return that the belt should spin downwards
    if let Ok(true) = down {
        inst_control(true, false);
        return Some(false);
    }

    // if there are no belt buttons being hit, make the belt stop
    inst_control(false, false);
    None
}

/// Spins the belt based on the provided instruction (if it's spinning and the direction of spin)
pub fn inst_control(active: bool, spinning_up: bool) {
    // get the desired voltage
    let voltage = match (active, spinning_up) {
        (true, true) => config::motors::BELT_SPEED,
        (true, false) => -config::motors::BELT_SPEED,
        (false, _) => 0,
    };

    // spin the belt motor
    let _ = motor::move_voltage( // ignore any errors
        config::motors::BELT.port,
        config::motors::BELT.reverse,
        voltage,
    );
}
