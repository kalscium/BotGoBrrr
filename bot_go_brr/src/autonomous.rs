//! Autonomous routine for the robot

use logic::info;
use safe_vex::rtos;
use crate::{belt, config, doinker, drive, log, solenoid};

/// The autonomous routine entrypoint
pub fn autonomous() {
    info!("autonomous period started");

    // variables that get mutated
    let mut logfile = log::logfile_init(config::log::LOGFILE_AUTO_PATH); // filestream to the opcontrol logfile
}
