//! Opcontrol routine for the robot

use safe_vex::rtos;
use crate::{bytecode, config, controls, drive};

/// The opcontrol routine entrypoint
pub fn opcontrol() {
    // variables that get mutated
    let mut now = rtos::millis(); // the current time
    let mut tick: u32 = 0; // the current tick
    let mut solenoid_active = false; // if the solenoid is active or not
    let mut solenoid_tick = tick; // the last time the solenoid's activity was changed
    let mut rot_pid = drive_controls::new_pid(); // the PID for rotational corrections

    // opcontrol loop
    loop {
        cycle(tick, &mut solenoid_active, &mut solenoid_tick, &mut rot_pid);
        tick += 1;
        rtos::task_delay_until(&mut now, config::TICK_SPEED);
    }
}

/// An individual opcontrol cycle
fn cycle(
    tick: u32,
    solenoid_active: &mut bool,
    solenoid_tick: &mut u32,
    rot_pid: &mut drive_controls::pid::Pid<f32>
) {
    // get belt instruction
    let belt_inst = controls::belt();

    // get solenoid instruction
    let solenoid_inst = controls::solenoid(tick, solenoid_active, solenoid_tick);

    // get drive instruction
    let drive_inst = match controls::drive() {
        bytecode::ByteCode::Drive { x, y, desired_angle } => (x, y, desired_angle),
        _ => unreachable!("drive controls function must output drive bytecode"),
    };

    // execute all generated instructions
    drive::drive(drive_inst.0, drive_inst.1, drive_inst.2, rot_pid);
    bytecode::execute(belt_inst);
    bytecode::execute(solenoid_inst);
}
