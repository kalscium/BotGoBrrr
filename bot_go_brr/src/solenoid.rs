//! Functions for controlling the solenoid of the robot

use safe_vex::{controller::{self, Controller}, adi};
use crate::config;

/// Activates the solenoid based on user input, the current tick, the last tick it was toggled, and it's current activation state and returns if it's active or not
pub fn user_control(
    tick: u32,
    last_toggled: &mut u32,
    active: &mut bool,
) -> bool {
    // if there hasn't been at least `config::solenoid::DELAY` ticks then do nothing
    if tick - *last_toggled < config::solenoid::DELAY {
        return *active;
    }

    // otherwise check for the solenoid button being toggled
    let toggled = controller::get_digital(Controller::Master, config::controls::SOLENOID_TOGGLE);
    if let Ok(true) = toggled {
        // update the solenoid tick and active variables
        *last_toggled = tick;
        *active = !*active; // toggle the 'active' state

        // rumble the controller if the solenoid is active to alert driver
        if *active {
            let _ = controller::rumble(Controller::Master, "."); // ignore any errors
        }
    }

    // update the solenoid and return it's state
    inst_control(*active);
    *active
}

/// Activates the solenoid based on provided state
pub fn inst_control(active: bool) {
    unsafe {
        adi::digital_write(config::solenoid::PORT, active)
            .expect("solenoid should've been configured long before this point");
    }
}
