//! Initialization routine for the robot

use safe_vex::{adi::{self, AdiConfig}, imu};
use crate::config;

/// The initialization routine entrypoint
pub fn initialize() {
    // configure the solenoid
    adi::set_config(config::solenoid::PORT, AdiConfig::DigitalOut);
    adi::set_config(safe_vex::port::AdiPort::H, AdiConfig::DigitalIn);

    // calibrate the interial sensor
    if imu::reset(config::IMU_PORT).is_err() {
        safe_vex::io::println!("silent error: failed to calibrate inertial sensor"); // don't crash
    }
}
