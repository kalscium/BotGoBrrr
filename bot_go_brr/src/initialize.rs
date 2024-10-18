//! Initialization routine for the robot

use safe_vex::{adi::{self, AdiConfig}, imu};
use crate::config;

/// The initialization routine entrypoint
pub fn initialize() {
    // configure the solenoid
    adi::set_config(config::solenoid::PORT, AdiConfig::DigitalOut);

    // calibrate the interial sensor
    if let Err(_) = imu::reset(config::IMU_PORT) {
        safe_vex::io::println!("silent error: failed to calibrate inertial sensor"); // don't crash
    }
}
