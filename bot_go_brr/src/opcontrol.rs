//! Opcontrol routine for the robot

use logic::{debug, info};
use safe_vex::rtos;
use crate::{belt, config, drive, log, solenoid};
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

    // opcontrol loop
    loop {
        debug!("opctrl tick: {tick}");

        // execute the belt
        let _belt_inst = belt::user_control();

        // execute the solenoid
        let _solenoid_inst = solenoid::user_control(tick, &mut solenoid_tick, &mut solenoid_active);

        // execute the drivetrain
        let _thrust = drive::user_control(&mut prev_vdr);

        // record the three values
        record.record(_thrust, _belt_inst, _solenoid_inst);

        // flush logs
        log::logic_flush(&mut logfile);

        // update the tick and wait for the next loop cycle
        tick += 1;
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}
