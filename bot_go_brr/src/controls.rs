use safe_vex::controller::Controller;
use crate::{config, drive_train::DriveInst};

/// generates the drive instruction from the controller linearly
#[inline]
pub fn gen_drive_inst(last_inst: DriveInst, last_joystick: (i8, i8), controller: &Controller) -> (DriveInst, (i8, i8)) {
    // percentage joystick values
    let (j1, j2) = (
        controller
            .left_stick
            .y,
        controller
            .right_stick
            .y,
    );

    // calculate the target voltages
    let (target1, target2) = (
        if j1.is_positive() { 12000f32 } else { -12000f32 },
        if j1.is_positive() { 12000f32 } else { -12000f32 },
    );

    // calculate the joystick difference
    let (d1, d2) = (
        (j1.abs() as f32 - last_joystick.0.abs() as f32) / j1 as f32,
        (j2.abs() as f32 - last_joystick.1.abs() as f32) / j2 as f32,
    );

    let (left, right) = (
        last_inst.left + ((target1 - last_inst.left as f32) * (1.0 - (- config::EXPONENT_SPEED * d1))) as i32,
        last_inst.right + ((target2 - last_inst.right as f32) * (1.0 - (- config::EXPONENT_SPEED * d2))) as i32,
    );

    (DriveInst {
        left,
        right,
    }, (j1, j2))
}
