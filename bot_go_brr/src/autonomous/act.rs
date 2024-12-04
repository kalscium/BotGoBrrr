//! Common actions and functions used during autonomous

use logic::info;
use logic::odom::OdomState;
use logic::pid::PIDState;
use safe_vex::rtos;
use crate::log;
use crate::{drive, config, log::LogFile};

/// Waits for the specified milliseconds
pub fn wait(ms: u32, odom: &mut OdomState) {
    info!("routine: waiting {ms}ms");

    let mut now = rtos::millis();
    for _ in 0..ms/config::TICK_SPEED {
        // update odometry
        logic::odom::account_for(
            drive::get_rotation_angle(config::auton::ODOM_LY_PORT),
            drive::get_rotation_angle(config::auton::ODOM_RY_PORT),
            odom,
        );

        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}

pub use crate::solenoid::inst_control as solenoid;
pub use crate::belt::inst_control as belt;
pub use crate::doinker::inst_control as doinker;

/// Drives the robot's drivetrain at set voltages
pub fn drive(left: i32, right: i32) {
    drive::voltage_left(left);
    drive::voltage_right(right);
}

/// Corrects to a rotation and location
pub fn goto(
    yaw: f32,
    y_coord: f32,
    odom: &mut OdomState,
    logfile: &mut LogFile,
) {
    info!("going to y coord {y_coord}mm at yaw {yaw}°");
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
        maths::absf(y_coord - odom.y_coord) > config::auton::ODOM_PRECISION
    {
        // check if the yaw was off while it was correcting for y coord
        if maths::absf(logic::drive::low_angle_diff(yaw, drive::get_yaw())) > config::auton::ANGLE_PRECISION {
            // correct for the yaw before continuing to correct for the y coord
            correct_yaw(yaw, logfile);
        }

        // get correction y value
        let correct_y = logic::drive::y_coord_correct(
            y_coord,
            odom.y_coord,
            config::TICK_SPEED as f32 / 1000., // convert ms to s
            &config::auton::Y_PID,
            &mut pid,
        );

        // run it through arcade
        let (ldr, rdr) = logic::drive::arcade(0, correct_y as i32);

        // drive
        drive(ldr, rdr);

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

    // make sure it still has the right yaw
    correct_yaw(yaw, logfile);

    // make sure the robot's motors have stopped spinning
    drive(0, 0);
    
    info!("corrected for a y coord of {y_coord}mm at an angle of {yaw}°");
}

/// Correct for a specified yaw angle / rotation
pub fn correct_yaw(
    target: f32,
    logfile: &mut LogFile,
) {
    info!("correcting for yaw of {target}°");
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
        drive(ldr, rdr);

        // add code to update odom if you run into problems

        // flush logs
        log::logic_flush(logfile);

        // wait for tick
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }

    // make sure the robot's motors have stopped spinning
    drive(0, 0);
    
    info!("corrected for a yaw of {target}°");
}

/// Correct for a specified y coordinate
pub fn correct_y_coord(
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
        drive(ldr, rdr);

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

    // make sure the robot's motors have stopped spinning
    drive(0, 0);
    
    info!("corrected for a y coord of {target}mm");
}
