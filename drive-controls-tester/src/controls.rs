use crate::config;

#[derive(Debug, Clone, Copy)]
pub struct JoyStick {
    pub x: i8,
    pub y: i8,
}

/// generates the drive instruction from the controller smoothly
#[inline]
pub fn gen_drive_inst(j1: JoyStick, reversed: bool, precise: bool) -> (i32, i32)  {
    // get the calculated voltage for the x of j1
    let mut j1xv = (1024.0 * (config::DMN as f64).powf(j1.x.abs() as f64) - 1024.0)
        * if j1.x.is_negative() { -1.0 } else { 1.0 } // un-absolute the numbers
        * config::drive::TURN_SPEED as f64; // reduce turning speed

    // get the calculated absolute voltage for the y of j1
    let mut j1yv = (1024.0 * (config::DMN as f64).powf(j1.y.abs() as f64) - 1024.0)
        * if j1.y.is_negative() { -1.0 } else { 1.0 }; // un-absolute the numbers

    // reduce the voltages / speeds of the motors if precise driving is on
    if precise {
        j1xv *= config::drive::PRECISE_MULTIPLIER as f64;
        j1yv *= config::drive::PRECISE_MULTIPLIER as f64;
    }

    // calculate the left and right drives according to arcade controls
    let (mut ldr, mut rdr) = (
        (j1yv + j1xv).clamp(-12000.0, 12000.0),
        (j1yv - j1xv).clamp(-12000.0, 12000.0),
    );

    // swap the left and right drives and flip the sign if the robot is driving in reversed mode
    if reversed {
        core::mem::swap(&mut ldr, &mut rdr);
        ldr = -ldr;
        rdr = -rdr;
    }

    // return the left and right drives
    (
        ldr as i32,
        rdr as i32,
    )
}
