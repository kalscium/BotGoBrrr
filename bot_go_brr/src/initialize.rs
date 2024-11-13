//! Initialization routine for the robot

use logic::warn;
use safe_vex::{adi::{self, AdiConfig}, imu, rotation};
use crate::config;

/// The initialization routine entrypoint
pub fn initialize() {
    // configure the solenoid
    adi::set_config(config::solenoid::PORT, AdiConfig::DigitalOut);
    adi::set_config(safe_vex::port::AdiPort::H, AdiConfig::DigitalIn);

    // calibrate the interial sensor
    if let Err(err) = imu::reset(config::IMU_PORT) {
        warn!("`PROSErr` occured while calibrating intertial sensor at port {}: {err:?}", config::IMU_PORT as u8);
    }

    // calibrate the odom x rotation sensor
    if let Err(err) = rotation::reset(config::odom::PORT_X) {
        warn!("`PROSErr` occured while calibrating odom x rotation sensor at port {}: {err:?}", config::IMU_PORT as u8);
    }
}
