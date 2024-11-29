//! Autonomous routine for the robot

use logic::info;
use safe_vex::rtos;
use crate::{belt, config, doinker, drive, log, solenoid};

/// The autonomous routine entrypoint
pub fn autonomous() {
    info!("autonomous period started");

    // variables that get mutated
    let mut logfile = log::logfile_init(config::log::LOGFILE_AUTO_PATH); // filestream to the opcontrol logfile

    // variables for odometry
    let mut prev_rot_y: f32 = drive::get_rotation_angle(config::auton::ODOM_Y_PORT); // the previous measurement from the y rotation sensor
    let mut y_coord: f32 = 0.; // the current calculated y coordinate of the robot
}
