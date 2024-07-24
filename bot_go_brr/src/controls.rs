use safe_vex::controller::Controller;
use crate::{bytecode::ByteCode, config, powf};

/// generates the drive instruction from the controller linearly
#[inline]
pub fn gen_drive_inst(controller: &Controller) -> [ByteCode; 2]  {
    // percentage joystick values
    let (j1, j2) = (
        controller
            .left_stick
            .y,
        controller
            .right_stick
            .y,
    );

    let (left, right) = (
        (powf(config::DMN, j1 as f64)) as i32,
        (powf(config::DMN, j2 as f64)) as i32,
    );

    [
        ByteCode::LeftDrive { voltage: left },
        ByteCode::RightDrive { voltage: right },
    ]
}
