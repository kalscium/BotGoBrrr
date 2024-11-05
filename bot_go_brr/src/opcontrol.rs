//! Opcontrol routine for the robot

use logic::{debug, info};
use safe_vex::rtos;
use crate::{belt, config, drive, log::{self, LogFile}, solenoid};

/// The opcontrol routine entrypoint
pub fn opcontrol() {
    info!("opcontrol period started");

    // variables that get mutated
    let mut logfile = log::logfile_init(config::log::LOGFILE_OP_PATH); // filestream to the opcontrol logfile
    let mut now = rtos::millis(); // the current time
    let mut tick: u32 = 0; // the current tick
    let mut solenoid_active = false; // if the solenoid is active or not
    let mut solenoid_tick = tick; // the last time the solenoid's activity was changed
    let mut angle_integral: f32 = 0.0; // the integral for the robot's rotational corrections

    // opcontrol loop
    loop {
        cycle(
            &mut logfile,
            tick,
            &mut solenoid_active,
            &mut solenoid_tick,
            &mut angle_integral,
        );
        tick += 1;
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}

/// An individual opcontrol cycle
fn cycle(
    logfile: &mut LogFile,
    tick: u32,
    solenoid_active: &mut bool,
    solenoid_tick: &mut u32,
    angle_integral: &mut f32,
) {
    debug!("opctrl tick: {tick}");

    // execute the belt
    belt::user_control();

    // execute the solenoid
    solenoid::user_control(tick, solenoid_tick, solenoid_active);

    // execute the drivetrain
    drive::user_control(angle_integral);

    // flush logs
    log::logic_flush(logfile);
}
