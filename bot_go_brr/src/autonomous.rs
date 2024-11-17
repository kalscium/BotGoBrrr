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

    // autonomous routine
    #[cfg(not(feature="full-autonomous"))]
    let auton_routine = AutonRoutine::new(&include!("autonomous/match_auton.rs")); // iterator over the auton routine insts
    #[cfg(feature="full-autonomous")]
    let mut auton_routine = AutonRoutine::new(&include!("autonomous/full_auton.rs")); // iterator over the auton routine insts

    // autonomous loop
    for inst in auton_routine {
        // get the current angle error
        let mut angle_error = logic::drive::low_angle_diff(
            i16::from(inst.req_angle) as f32,
            drive::get_yaw(),
        );

        // keep correcting as long as the error is too large (outside of precision limit)
        while
            maths::absf(angle_error) > config::auton::ANGLE_PRECISION
        {
            // correct for the errors
            let (ldr, rdr) = logic::drive::inst_control(
                i16::from(inst.req_angle) as f32,
                drive::get_yaw(),
                &mut prev_vdr,
            );

            // drive the correction
            drive::voltage_left(ldr);
            drive::voltage_right(rdr);

            // flush logs
            log::logic_flush(&mut logfile);

            // wait a tick cycle inbetween corrections
            rtos::task_delay_until(&mut now, config::TICK_SPEED);

            // update the errors
            angle_error = logic::drive::low_angle_diff(
                i16::from(inst.req_angle) as f32,
                drive::get_yaw(),
            );
        }

        // once it meets the requirements, then execute it's actions
        belt::inst_control(inst.act_belt_active, inst.act_belt_up);
        doinker::inst_control(inst.act_doinker_active, inst.act_doinker_up);
        solenoid::inst_control(inst.act_solenoid_active);

        // drive based on thrust
        drive::voltage_left(i16::from(inst.thrust) as i32);
        drive::voltage_right(i16::from(inst.thrust) as i32);

        // logs
        info!("precision requirements achieved");
        info!("correction time: {}", now - rtos::millis());
        info!("belt active: {}, belt up: {}", inst.act_belt_active, inst.act_belt_up);
        info!("solenoid active: {}", inst.act_solenoid_active);
        info!("thrust: {}", inst.thrust);

        // flush logs
        log::logic_flush(&mut logfile);

        // wait a tick cycle
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}
