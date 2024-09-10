use safe_vex::controller::Controller;
use crate::{bytecode::ByteCode, config, powf};

/// generates the drive instruction from the controller smoothly
#[inline]
pub fn gen_drive_inst(controller: &Controller, forwards: &mut bool) -> [ByteCode; 2]  {
    // get joystick values and reverse values
    let j1 = &controller.left_stick;
    let reverse = controller.l1;

    if j1.x.abs() as u8 <= config::STICK_RESET_THRESHOLD {
        *forwards = false;
    } else if j1.y.is_positive() {
        *forwards = true;
    }

    // get the calculated voltage for the x of j1
    let j1xv = (1024.0 * powf(config::DMN as f64, j1.x.abs() as f64) - 1024.0)
        * if j1.x.is_negative() { -1.0 } else { 1.0 } // un-absolute the numbers
        * if controller.l2 { config::drive::PRECISE_MULTIPLIER as f64 } else { 1.0 } // precise turning
        * config::drive::TURN_SPEED as f64; // reduce turning speed

    // get the calculated voltage for the y of j1
    let j1yv = (1024.0 * powf(config::DMN as f64, j1.y.abs() as f64) - 1024.0)
        * if j1.y.is_negative() { -1.0 } else { 1.0 } // un-absolute the numbers
        * if controller.l2 { config::drive::PRECISE_MULTIPLIER as f64 } else { 1.0 }; // precise driving

    // calculate the left and right drives according to arcade controls
    let (mut ldr, mut rdr) = (
        (j1yv - j1xv).clamp(-12000.0, 12000.0),
        (j1yv + j1xv).clamp(-12000.0, 12000.0),
    );

    // swap the right and left drives if you are driving backwards or reversed while driving forwards
    if j1.y.is_positive() && reverse || !*forwards && j1.y.is_negative() {
        core::mem::swap(&mut ldr, &mut rdr);
    }

    // if the robot is driving forwards but the joystick is pointing down then flip the sign
    if *forwards && j1.x == 0 {
        ldr = -ldr;
        rdr = -rdr;
    }

    // return the left and right drives
    [
        ByteCode::LeftDrive { voltage: ldr as i32 },
        ByteCode::RightDrive { voltage: rdr as i32 },
    ]
}
