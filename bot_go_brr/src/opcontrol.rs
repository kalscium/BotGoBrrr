//! Opcontrol routine for the robot

use logic::{debug, info};
use safe_vex::rtos;
use crate::{belt, config, doinker, drive, log, solenoid};
use crate::record::Record;

/// The opcontrol routine entrypoint
pub fn opcontrol() {
    info!("opcontrol period started");

    // variables that get mutated
    let mut logfile = log::logfile_init(config::log::LOGFILE_OP_PATH); // filestream to the opcontrol logfile
    let mut now = rtos::millis(); // the current time
    let mut tick: u32 = 0; // the current tick
    let mut solenoid_active = false; // if the solenoid is active or not
    let mut solenoid_tick = tick; // the last time the solenoid's activity was changed
    let mut record = Record::new_ignore(config::auton::RECORD_PATH); // record file for auton
    let mut prev_vdr: (i32, i32) = (0, 0); // the previous voltages for the left and right drives

    // variables for odometry
    let mut prev_rot_y: f32 = 0.; // the previous measurement from the y rotation sensor
    let mut y_coord: f32 = 0.; // the current calculated y coordinate of the robot

    // opcontrol loop
    loop {
        debug!("opctrl tick: {tick}");

        // update the odometry calculations
        logic::odom::account_for(
            drive::get_rotation_angle(config::auton::ODOM_Y_PORT),
            &mut prev_rot_y,
            &mut y_coord,
        ); info!("y coord: {y_coord}");

        // execute the belt
        let belt_inst = belt::user_control();

        // execute the doinker
        let doinker_inst = doinker::user_control();

        // execute the solenoid
        let solenoid_inst = solenoid::user_control(tick, &mut solenoid_tick, &mut solenoid_active);

        // execute the drivetrain
        drive::user_control(&mut prev_vdr);

        // record the three values
        record.record(y_coord, belt_inst, doinker_inst, solenoid_inst);

        // log how long the cycle took
        info!("cycle time: {}", now - rtos::millis());

        // flush logs
        log::logic_flush(&mut logfile);

        // update the tick and wait for the next loop cycle
        tick += 1;
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}
