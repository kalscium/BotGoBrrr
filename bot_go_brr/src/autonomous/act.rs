//! Common actions and functions used during autonomous

use logic::info;
use logic::odom::OdomState;
use logic::pid::PIDState;
use safe_vex::rtos;
use crate::log;
use crate::{drive, config, log::LogFile};

/// Waits for the specified milliseconds
pub fn wait(ms: u32) {
    info!("routine: waiting {ms}ms");
    rtos::task_delay(ms);
}

pub use crate::solenoid::inst_control as solenoid;
pub use crate::belt::inst_control as belt;
pub use crate::doinker::inst_control as doinker;

/// Correct for a specified yaw angle / rotation
pub fn rotate(
    target: f32,
    logfile: &mut LogFile,
) {
    info!("correcting for yaw of {target} degrees");
    let mut pid = PIDState::default();
    let mut now = rtos::millis();

    while
        // the error is larger than the precision limit
        maths::absf(logic::drive::low_angle_diff(target, drive::get_yaw())) > config::auton::ANGLE_PRECISION
    {
        let yaw = drive::get_yaw();

        // get correction x value
        let correct_x = logic::drive::rot_correct(
            target,
            yaw,
            config::TICK_SPEED as f32 / 1000., // convert ms to s
            &config::auton::ROT_PID,
            &mut pid,
        );

        // run it through arcade
        let (ldr, rdr) = logic::drive::arcade(correct_x as i32, 0);

        // drive
        drive::voltage_left(ldr);
        drive::voltage_right(rdr);

        // add code to update odom if you run into problems

        // flush logs
        log::logic_flush(logfile);

        // wait for tick
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }

    info!("corrected for a yaw of {target} degrees");
}

/// Correct for a specified y coordinate
pub fn y_coord(
    target: f32,
    odom: &mut OdomState,
    logfile: &mut LogFile,
) {
    info!("correcting for y coordinate of {target}mm");
    let mut pid = PIDState::default();
    let mut now = rtos::millis();

    // update odom
    logic::odom::account_for(
        drive::get_rotation_angle(config::auton::ODOM_LY_PORT),
        drive::get_rotation_angle(config::auton::ODOM_RY_PORT),
        odom,
    );

    while
        // the error is larger than the precision limit
        maths::absf(target - odom.y_coord) > config::auton::ODOM_PRECISION
    {
        // get correction y value
        let correct_y = logic::drive::y_coord_correct(
            target,
            odom.y_coord,
            config::TICK_SPEED as f32 / 1000., // convert ms to s
            &config::auton::Y_PID,
            &mut pid,
        );

        // run it through arcade
        let (ldr, rdr) = logic::drive::arcade(0, correct_y as i32);

        // drive
        drive::voltage_left(ldr);
        drive::voltage_right(rdr);

        // update odom
        logic::odom::account_for(
            drive::get_rotation_angle(config::auton::ODOM_LY_PORT),
            drive::get_rotation_angle(config::auton::ODOM_RY_PORT),
            odom,
        );

        // flush logs
        log::logic_flush(logfile);

        // wait for tick
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }

    info!("corrected for a y coord of {target}mm");
}
