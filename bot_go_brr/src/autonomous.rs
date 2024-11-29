//! Autonomous routine for the robot

pub mod act;

use logic::{info, odom::OdomState, pid::PIDState};
use crate::{config, drive, log::{self, LogFile}};

/// The autonomous routine entrypoint
pub fn autonomous() {
    info!("autonomous period started");

    // variables that get mutated
    let mut logfile = log::logfile_init(config::log::LOGFILE_AUTO_PATH); // filestream to the opcontrol logfile

    // varables for pid
    let mut odom_y_pid = PIDState::default();
    let mut rot_pid = PIDState::default();

    // variables for odom
    let mut odom_y = OdomState {
        prev_ly: drive::get_rotation_angle(config::auton::ODOM_LY_PORT),
        prev_ry: drive::get_rotation_angle(config::auton::ODOM_RY_PORT),
        y_coord: 0.,
    };

    #[cfg(not(feature = "skills"))]
    match_auton(&mut logfile, &mut odom_y_pid, &mut rot_pid, &mut odom_y);

    #[cfg(feature = "skills")]
    skills_auton(&mut logfile, &mut odom_y_pid, &mut rot_pid, &mut odom_y);
}

/// The autonomous routine for the begining of matches
fn match_auton(
    logfile: &mut LogFile,
    odom_y_pid: &mut PIDState,
    rot_pid: &mut PIDState,
    odom_y: &mut OdomState,
) {
    // flush logs
    log::logic_flush(logfile);
}

/// The autonomous routine for autonomous skills runs
fn skills_auton(
    logfile: &mut LogFile,
    odom_y_pid: &mut PIDState,
    rot_pid: &mut PIDState,
    odom_y: &mut OdomState,
) {
    // flush logs
    log::logic_flush(logfile);
}
