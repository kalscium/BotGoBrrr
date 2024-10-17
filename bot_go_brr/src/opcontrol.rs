//! Opcontrol routine for the robot

use safe_vex::rtos;
use crate::{bytecode, config, controls};

/// The opcontrol routine entrypoint
pub fn opcontrol() {
    // variables that get mutated
    let mut now = rtos::millis();
    let mut tick: u32 = 0;

    // opcontrol loop
    loop {
        cycle(&mut tick);
        tick += 1;
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}

/// An individual opcontrol cycle
fn cycle(_tick: &mut u32) {
    // get belt instruction
    let belt_inst = controls::belt();

    // execute all generated instructions
    bytecode::execute(belt_inst);
}
