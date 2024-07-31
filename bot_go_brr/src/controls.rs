use safe_vex::controller::Controller;
use crate::{bytecode::ByteCode, config, powf};

/// generates the drive instruction from the controller linearly
#[inline]
pub fn gen_drive_inst(controller: &Controller) -> [ByteCode; 2]  {
    // percentage joystick values
    let j1 = &controller.left_stick;

    // get the calculated voltages from the absolute x & y of the joystick
    let j1xv = powf(config::DMN, j1.x.abs() as f64)
        * if controller.l2 { config::drive::PRECISE_MULTIPLIER as f64 } else { 1.0 }; // precise turning
    let j1yv = powf(config::DMN, j1.y.abs() as f64)
        * if controller.l2 { config::drive::PRECISE_MULTIPLIER as f64 } else { 1.0 }; // precise turning

    // left drive & right drive
    let (ldr, rdr) = match (j1.x_larger(), j1.x.is_positive(), j1.y.is_positive()) {
        // move forward
        (false, _, true) => (j1yv, j1yv),
        // move backwards
        (false, _, false) => (-j1yv, -j1yv),

        _ => {
            let voltage = j1xv * config::drive::TURN_SPEED as f64;

            if j1.x.is_positive() { // turn right
                (voltage, -voltage)
            } else { // turn left
                (-voltage, voltage)
            }
        },
    };

    [
        ByteCode::LeftDrive { voltage: ldr as i32 },
        ByteCode::RightDrive { voltage: rdr as i32 },
    ]
}
