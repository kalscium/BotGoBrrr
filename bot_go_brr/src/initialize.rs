//! Initialization routine for the robot

use safe_vex::adi::{self, AdiConfig};
use crate::config;

/// The initialization routine entrypoint
pub fn initialize() {
    // configure the solenoid
    adi::set_config(config::solenoid::PORT, AdiConfig::DigitalOut);
}
