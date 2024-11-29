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
    // move 72cm into the mogo infront
    act::y_coord(720., odom_y, logfile, odom_y_pid);

    // make sure the robot is straight and then activate the solenoid
    act::rotate(0., logfile, rot_pid);
    act::solenoid(true);

    // activate the belt for around 2 seconds
    act::belt(config::motors::BELT_VOLTS);
    act::wait(2000);

    // move to hit the pylon
    act::rotate(0., logfile, rot_pid);
    act::y_coord(720. + 580., odom_y, logfile, odom_y_pid); // move another 58cm

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
    act::wait(5000); // wait 5 seconds to calibrate imu

    // flush logs
    log::logic_flush(logfile);
}
