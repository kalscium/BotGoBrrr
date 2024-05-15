use safe_vex::controller::Controller;
use crate::{config, drive_train::DriveInst};

/// generates the drive instruction from the controller linearly
#[inline]
pub fn gen_drive_inst(controller: &Controller) -> DriveInst {
    // percentage joystick values
    let (j1, j2) = (
        controller
            .left_stick
            .clone()
            .clamp(config::CONTROLLER_STICK_MIN)
            .y as f32 * 100f32 / i8::MAX as f32,
        controller
            .right_stick
            .clone()
            .clamp(config::CONTROLLER_STICK_MIN)
            .y as f32 * 100f32 / i8::MAX as f32,
    );

    DriveInst {
        left:  calculate_voltage(j1, 100),
        right: calculate_voltage(j2, 100),
    }
}

#[inline]
fn calculate_voltage(percent1: f32, percent2: u8) -> i32 {
    (
        12000f32
        * percent1 / 100f32
        * percent2 as f32 / 100f32
    ).clamp(-12000.0, 12000.0) as i32
}
