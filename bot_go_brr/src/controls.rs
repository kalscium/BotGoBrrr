use safe_vex::controller::Controller;
use crate::{bytecode::ByteCode, config, drive_train::DriveTrain};

/// generates the drive instruction from the controller linearly
#[inline]
pub fn gen_drive_inst(drive_train: &DriveTrain, last_joystick: (i8, i8), controller: &Controller) -> ([ByteCode; 2], (i8, i8)) {
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

    // get the current voltage of the motors
    let (voltage_left, voltage_right) = (
        drive_train.l1.get_ref().and_then(|x| x.get_voltage().ok()).unwrap_or(0),
        drive_train.r1.get_ref().and_then(|x| x.get_voltage().ok()).unwrap_or(0),
    );

    let (left, right) = (
        voltage_left + ((target1 - voltage_left as f32) * (1.0 - (- config::EXPONENT_SPEED * d1))) as i32,
        voltage_right + ((target2 - voltage_right as f32) * (1.0 - (- config::EXPONENT_SPEED * d2))) as i32,
    );

    ([
        ByteCode::LeftDrive { voltage: left },
        ByteCode::RightDrive { voltage: right },
    ], (j1, j2))
}
