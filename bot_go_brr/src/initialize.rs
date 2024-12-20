//! Initialization routine for the robot

use logic::warn;
use safe_vex::{adi::{self, AdiConfig}, imu};
use crate::config;

/// The initialization routine entrypoint
pub fn initialize() {
    // configure the solenoid
    adi::set_config(config::solenoid::PORT, AdiConfig::DigitalOut);
    adi::set_config(safe_vex::port::AdiPort::H, AdiConfig::DigitalIn);

    // calibrate the interial sensor
    if let Err(err) = imu::reset(config::auton::IMU_PORT) {
        warn!("`PROSErr` occured while calibrating intertial sensor at port {}: {err:?}", config::auton::IMU_PORT as u8);
    }
}
