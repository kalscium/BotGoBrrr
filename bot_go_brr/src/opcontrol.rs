//! Opcontrol routine for the robot

use logic::{debug, info};
use safe_vex::rtos;
use crate::{belt, config, drive, log::{self, LogFile}, solenoid};

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
    let mut angle_integral: f32 = 0.0; // the integral for the robot's rotational corrections

    // (optional) record file for auton
    #[cfg(feature="record")]
    let mut record = Record::new_ignore(config::auton::RECORD_PATH);

    // opcontrol loop
    loop {
        cycle(
            &mut logfile,
            #[cfg(feature="record")]
            &mut record,
            tick,
            &mut solenoid_active,
            &mut solenoid_tick,
            &mut prev_vdr,
            &mut angle_integral,
        );
        tick += 1;
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}

/// An individual opcontrol cycle
fn cycle(
    logfile: &mut LogFile,
    #[cfg(feature="record")]
    record: &mut Record,
    tick: u32,
    solenoid_active: &mut bool,
    solenoid_tick: &mut u32,
    prev_vdr: &mut (i32, i32),
    angle_integral: &mut f32,
) {
    debug!("opctrl tick: {tick}");

    // execute the belt
    let _belt_inst = belt::user_control();

    // execute the solenoid
    let _solenoid_inst = solenoid::user_control(tick, solenoid_tick, solenoid_active);

    // execute the drivetrain
    let _thrust = drive::user_control(prev_vdr, angle_integral);

    // record the three values
    #[cfg(feature="record")]
    record.record(_thrust, _belt_inst, _solenoid_inst);

    // flush logs
    log::logic_flush(logfile);
}
