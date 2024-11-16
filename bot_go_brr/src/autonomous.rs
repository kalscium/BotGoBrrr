//! Autonomous routine for the robot

use logic::{info, inst::AutonRoutine};
use safe_vex::rtos;
use crate::{belt, config, doinker, drive, log, solenoid};

/// The autonomous routine entrypoint
pub fn autonomous() {
    info!("autonomous period started");

    // variables that get mutated
    let mut logfile = log::logfile_init(config::log::LOGFILE_AUTO_PATH); // filestream to the opcontrol logfile
    let mut now = rtos::millis(); // the current time
    let mut prev_vdr: (i32, i32) = (0, 0); // the previous voltages of the left and right drives

    // variables for odometry
    let mut prev_rot_y: f32 = 0.; // the previous measurement from the y rotation sensor
    let mut y_coord: f32 = 0.; // the current calculated y coordinate of the robot

    // autonomous routine
    #[cfg(not(feature="full-autonomous"))]
    let auton_routine = AutonRoutine::new(&include!("autonomous/match_auton.rs")); // iterator over the auton routine insts
    #[cfg(feature="full-autonomous")]
    let mut auton_routine = AutonRoutine::new(&include!("autonomous/full_auton.rs")); // iterator over the auton routine insts

    // autonomous loop
    for inst in auton_routine {
        // update odometry calculation
        logic::odom::account_for(
            drive::get_rotation_angle(config::auton::ODOM_Y_PORT),
            &mut prev_rot_y,
            &mut y_coord,
        );

        // get the current angle error
        let mut angle_error = logic::drive::low_angle_diff(
            i16::from(inst.req_angle) as f32,
            drive::get_yaw(),
        );

        // get the current y coord error
        let mut y_coord_error = i16::from(inst.req_odom_y) as f32 - y_coord;

        // keep correcting as long as the error is too large (outside of precision limit)
        while
            maths::absf(angle_error) > config::auton::ANGLE_PRECISION
            || maths::absf(y_coord_error) > config::auton::ODOM_PRECISION
        {
            // correct for the errors
            let (ldr, rdr) = logic::drive::inst_control(
                i16::from(inst.req_angle) as f32,
                i16::from(inst.req_odom_y) as f32,
                drive::get_yaw(),
                y_coord,
                &mut prev_vdr,
            );

            // drive the correction
            drive::voltage_left(ldr);
            drive::voltage_right(rdr);

            // flush logs
            log::logic_flush(&mut logfile);

            // wait a tick cycle inbetween corrections
            rtos::task_delay_until(&mut now, config::TICK_SPEED);

            // update odometry calculation
            logic::odom::account_for(
                drive::get_rotation_angle(config::auton::ODOM_Y_PORT),
                &mut prev_rot_y,
                &mut y_coord,
            );

            // update the errors
            angle_error = logic::drive::low_angle_diff(
                i16::from(inst.req_angle) as f32,
                drive::get_yaw(),
            );
            y_coord_error = i16::from(inst.req_odom_y) as f32 - y_coord;
        }

        // once it meets the requirements, then execute it's actions
        belt::inst_control(inst.act_belt_active, inst.act_belt_up);
        doinker::inst_control(inst.act_doinker_active, inst.act_doinker_up);
        solenoid::inst_control(inst.act_solenoid_active);

        // logs
        info!("precision requirements achieved");
        info!("correction time: {}", now - rtos::millis());
        info!("belt active: {}, belt up: {}", inst.act_belt_active, inst.act_belt_up);
        info!("solenoid active: {}", inst.act_solenoid_active);

        // flush logs
        log::logic_flush(&mut logfile);

        // wait a tick cycle
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}
