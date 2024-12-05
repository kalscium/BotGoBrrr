//! Opcontrol routine for the robot

use logic::{debug, info, odom::OdomState, pid::PIDState};
use safe_vex::rtos;
use crate::{belt, config, doinker, drive, log, solenoid};

/// The opcontrol routine entrypoint
pub fn opcontrol() {
    info!("opcontrol period started");

    // variables that get mutated
    let mut logfile = log::logfile_init(config::log::LOGFILE_OP_PATH); // filestream to the opcontrol logfile
    let mut now = rtos::millis(); // the current time
    let mut tick: u32 = 0; // the current tick
    let mut solenoid_active = false; // if the solenoid is active or not
    let mut solenoid_tick = tick; // the last time the solenoid's activity was changed

    // variables for odometry
    let mut odom = OdomState {
        prev_ly: drive::get_rotation_angle(config::auton::ODOM_LY_PORT),
        prev_ry: drive::get_rotation_angle(config::auton::ODOM_RY_PORT),
        y_coord: 0.,
    };

    // variables for the pid
    let mut rot_pid = PIDState::default();
    let mut y_pid = PIDState::default();
    
    // opcontrol loop
    loop {
        debug!("opctrl tick: {tick}");
        debug!("time: {}s", rtos::millis() as f32 / 1000.);

        // update the odometry calculations
        logic::odom::account_for(
            drive::get_rotation_angle(config::auton::ODOM_LY_PORT),
            drive::get_rotation_angle(config::auton::ODOM_RY_PORT),
            &mut odom,
        );
        
        // execute the belt
        belt::user_control();

        // execute the doinker
        doinker::user_control();

        // execute the solenoid
        solenoid::user_control(tick, &mut solenoid_tick, &mut solenoid_active);

        // execute the drivetrain
        drive::user_control(
            config::TICK_SPEED as f32 / 1000.,
            odom.y_coord,
            &config::auton::ROT_PID,
            &mut rot_pid,
            &config::auton::Y_PID,
            &mut y_pid,
        );

        // log how long the cycle took
        info!("cycle time: {}", (rtos::millis() - now) as f32 / 1000.);

        // flush logs
        log::logic_flush(&mut logfile);

        // update the tick and wait for the next loop cycle
        tick += 1;
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}
