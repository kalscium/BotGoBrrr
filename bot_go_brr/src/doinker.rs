//! Functinos for controlling the doinker of the robot

use logic::{info, warn};
use safe_vex::{controller::{self, Controller}, motor};
use crate::config;

/// Spins the doinker based on user input and returns if it's spinning or not and the direction of that spin
pub fn user_control() {
    // grab the boolean activations of the bytecode 
    let up = controller::get_digital(Controller::Master, config::controls::DOINKER_UP);
    let down = controller::get_digital(Controller::Master, config::controls::DOINKER_DOWN);

    // if the up button is hit, then spin the doinker up
    if let Ok(true) = up {
        return inst_control(config::motors::DOINKER_VOLTS);
    }
    
    // if the down button is hit, then return that the doinker should spin downwards
    if let Ok(true) = down {
        return inst_control(-config::motors::DOINKER_VOLTS);
    }

    // if there are no doinker buttons being hit, make the doinker stop
    inst_control(0);
}

/// Spins the doinker based upon the provided voltage in millivolts
pub fn inst_control(voltage: i32) {
    info!("doinker voltage: {voltage}");

    // spin the doinker motor and log any errors
    if let Err(err) = motor::move_voltage(
        config::motors::DOINKER.port,
        config::motors::DOINKER.reverse,
        voltage,
    ) {
        warn!("`PROSErr` occured while setting motor voltage for doinker at port {}: {err:?}", config::motors::DOINKER.port as u8);
    }
}
