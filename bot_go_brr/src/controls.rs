use safe_vex::controller::Controller;
use crate::bytecode::ByteCode;

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
        (j1 as f32 / i8::MAX as f32 * 12000.0) as i32,
        (j2 as f32 / i8::MAX as f32 * 12000.0) as i32,
    );

    [
        ByteCode::LeftDrive { voltage: left },
        ByteCode::RightDrive { voltage: right },
    ]
}
