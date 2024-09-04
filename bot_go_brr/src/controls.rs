use safe_vex::controller::Controller;
use crate::{bytecode::ByteCode, config, powf};

/// generates the drive instruction from the controller linearly
#[inline]
pub fn gen_drive_inst(controller: &Controller) -> [ByteCode; 2]  {
    // get joystick values and reverse values
    let j1 = &controller.left_stick;
    let reversed = controller.l1;

    // get the calculated voltages from the absolute x & y of the joystick
    let j1xv = (1024.0 * powf(config::DMN as f64, j1.x.abs() as f64) - 1024.0)
        * if controller.l2 { config::drive::PRECISE_MULTIPLIER as f64 } else { 1.0 }; // precise turning
    let j1yv = (1024.0 * powf(config::DMN as f64, j1.y.abs() as f64) - 1024.0)
        * if controller.l2 { config::drive::PRECISE_MULTIPLIER as f64 } else { 1.0 }; // precise turning
    // left drive & right drive
    let (ldr, rdr) = match (j1.x_larger(), j1.x.is_positive(), j1.y.is_positive(), reversed) {
        // move forward
        (false, _, true, false) => (j1yv, j1yv), // normal
        (false, _, true, true) => (j1yv, j1yv), // while reversed (driving backwards)
        // move backwards
        (false, _, false, false) => (-j1yv, -j1yv), // normal
        (false, _, false, true) => (j1yv, j1yv), // while reversed (driving backwards)

        _ => {
            let voltage = j1xv * config::drive::TURN_SPEED as f64;

            if j1.x.is_positive() {
                (voltage, -voltage)
            } else {
                (-voltage, voltage)
            }
        },
    };

    [
        ByteCode::LeftDrive { voltage: ldr as i32 },
        ByteCode::RightDrive { voltage: rdr as i32 },
    ]
}
