//! Opcontrol routine for the robot

use safe_vex::rtos;
use crate::{bytecode, config, controls};

/// The opcontrol routine entrypoint
pub fn opcontrol() {
    // variables that get mutated
    let mut now = rtos::millis(); // the current time
    let mut tick: u32 = 0; // the current tick
    let mut solenoid_active = false; // if the solenoid is active or not
    let mut solenoid_tick = tick; // the last time the solenoid's activity was changed

    // opcontrol loop
    loop {
        cycle(tick, &mut solenoid_active, &mut solenoid_tick);
        tick += 1;
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}

/// An individual opcontrol cycle
fn cycle(tick: u32, solenoid_active: &mut bool, solenoid_tick: &mut u32) {
    // get belt instruction
    let belt_inst = controls::belt();

    // get solenoid instruction
    let solenoid_inst = controls::solenoid(tick, solenoid_active, solenoid_tick);

    // get drive instruction
    let drive_inst = controls::drive();

    // execute all generated instructions
    bytecode::execute(belt_inst);
    bytecode::execute(solenoid_inst);
}
