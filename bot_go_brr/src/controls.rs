use safe_vex::controller::Controller;
use crate::{bytecode::ByteCode, config, powf};

/// generates the drive instruction from the controller linearly
#[inline]
pub fn gen_drive_inst(controller: &Controller) -> [ByteCode; 2]  {
    // percentage joystick values
    let j1 = &controller.left_stick;

    // get the calculated voltages from the absolute x & y of the joystick
    let j1xv = powf(config::DMN, j1.x.abs() as f64);
    let j1yv = powf(config::DMN, j1.y.abs() as f64);

    // left drive & right drive
    let (ldr, rdr) = match (j1.x_larger(), j1.x.is_positive(), j1.y.is_positive()) {
        // move forward
        (false, _, true) => (j1yv, j1yv),
        // move backwards
        (false, _, false) => (-j1yv, -j1yv),

        // turn right
        (true, true, _) => (j1xv, -j1xv),
        // turn left
        (true, false, _) => (-j1xv, j1xv),
    };

    [
        ByteCode::LeftDrive { voltage: ldr as i32 },
        ByteCode::RightDrive { voltage: rdr as i32 },
    ]
}
