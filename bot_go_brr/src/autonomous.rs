//! Autonomous routine for the robot

use logic::{info, inst::AutonRoutine};
use safe_vex::rtos;
use crate::{belt, config, drive, log, solenoid};

/// The autonomous routine entrypoint
pub fn autonomous() {
    info!("autonomous period started");

    // variables that get mutated
    let mut logfile = log::logfile_init(config::log::LOGFILE_OP_PATH); // filestream to the opcontrol logfile
    let mut now = rtos::millis(); // the current time
    let mut angle_integral: f32 = 0.0; // the integral for the robot's rotational corrections

    // autonomous routine
    #[cfg(not(feature="full-autonomous"))]
    let auton_routine = AutonRoutine::new(&include!("autonomous/match_auton.rs")); // iterator over the auton routine insts
    #[cfg(feature="full-autonomous")]
    let mut auton_routine = AutonRoutine::new(&include!("autonomous/full_auton.rs")); // iterator over the auton routine insts

    // autonomous loop
    for inst in auton_routine {
        loop {
            // get the current angle error
            let angle_error = maths::absf(logic::drive::low_angle_diff(
                i16::from(inst.req_angle) as f32,
                drive::get_yaw(),
            ));

            // if the absolute error is smaller than the angle precision requirement then run the inst's actions and advance to the next instruction
            if angle_error <= config::auton::ANGLE_PRECISION {
                // once it meets the requirements, then execute it's actions
                belt::inst_control(inst.act_belt_active, inst.act_belt_up);
                solenoid::inst_control(inst.act_solenoid_active);

                // flush logs
                info!("belt active: {}, belt up: {}", inst.act_belt_active, inst.act_belt_up);
                info!("solenoid active: {}", inst.act_solenoid_active);
                log::logic_flush(&mut logfile);

                // wait a tick cycle (also sort of an inst action)
                rtos::task_delay_until(&mut now, config::TICK_SPEED);

                break; // advance to the next inst
            }

            // correct for the required angle

            // get the correction ldr and rdr (for the angle)
            let (ldr, rdr) = logic::drive::inst_control(
                i16::from(inst.thrust) as i32,
                i16::from(inst.req_angle) as f32,
                drive::get_yaw(),
                config::TICK_SPEED as f32 / 1000.0,
                &mut angle_integral,
            );

            // drive the correction
            drive::voltage_left(ldr);
            drive::voltage_right(rdr);

            // flush logs
            log::logic_flush(&mut logfile);

            // wait a tick cycle inbetween corrections
            rtos::task_delay_until(&mut now, config::TICK_SPEED);
        }
    }
}
