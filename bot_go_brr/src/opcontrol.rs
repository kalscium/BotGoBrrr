//! Opcontrol routine for the robot

use logic::{debug, info};
use safe_vex::rtos;
use crate::{belt, config, drive, log, solenoid};

#[cfg(feature="record")]
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
    let mut prev_vdr: (i32, i32) = (0, 0); // the previous voltages for the left and right drives
    let mut initial_yaw: f32 = 0.0; // the initial yaw of the robot before corrections

    // (optional) record file for auton
    #[cfg(feature="record")]
    let mut record = Record::new_ignore(config::auton::RECORD_PATH);

    debug!("testing odom!");
    let mut odom: f32 = 0.;
    let mut prev_odom: f32 = 0.;

    // opcontrol loop
    loop {
        debug!("opctrl tick: {tick}");

        // test odom
        logic::odom::account_for(drive::get_rotation_angle(config::odom::PORT_X), &mut prev_odom, &mut odom);
        info!("odom: {odom}mm");

        // execute the belt
        let _belt_inst = belt::user_control();

        // execute the solenoid
        let _solenoid_inst = solenoid::user_control(tick, &mut solenoid_tick, &mut solenoid_active);

        // execute the drivetrain
        let _thrust = drive::user_control(&mut initial_yaw, &mut prev_vdr);

        // record the three values
        #[cfg(feature="record")]
        record.record(_thrust, _belt_inst, _solenoid_inst);

        // flush logs
        log::logic_flush(&mut logfile);

        // update the tick and wait for the next loop cycle
        tick += 1;
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}
