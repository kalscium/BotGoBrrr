use crate::config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum DrivingState {
    /// If the robot is actively driving forwards
    Forwards = 1,
    /// If the robot is actively driving backwards
    Backwards = -1,
    /// If the robot is currently neutral
    Neutral = 0,
}

#[derive(Debug, Clone, Copy)]
pub struct JoyStick {
    pub x: i8,
    pub y: i8,
}

/// generates the drive instruction from the controller smoothly
#[inline]
pub fn gen_drive_inst(j1: JoyStick, reversed: bool, precise: bool, driving_state: &mut DrivingState) -> (i32, i32)  {
    // set the driving state to neutral if the stick is within the stick reset threshold
    if j1.x.unsigned_abs()<= config::STICK_RESET_THRESHOLD && j1.y.unsigned_abs() <= config::STICK_RESET_THRESHOLD { // required for the next part to work
        *driving_state = DrivingState::Neutral;
    }

    // change the driving state to the direction of the joystick if the driving-state is neutral
    if let DrivingState::Neutral = *driving_state {
        if j1.y == 0 { // do nothing
            *driving_state = DrivingState::Neutral;
        } else if j1.y <= -(config::STICK_RESET_THRESHOLD as i8 / 2) { // driving backwards now
            *driving_state = DrivingState::Backwards;
        } else { // driving forwards now
            *driving_state = DrivingState::Forwards;
        }
    }

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

    // swap the left and right drives if the robot is driving backwards
    if *driving_state == DrivingState::Backwards {
        core::mem::swap(&mut ldr, &mut rdr);
    }

    // flip the sign of ldr & rdr if the robot is driving a certain direction that goes against the controller joystick when stick-x is within the minimum stick reset threshold
    if (*driving_state as i8).is_positive() != j1.y.is_positive() && j1.x.unsigned_abs() <= config::STICK_RESET_THRESHOLD {
        ldr = -ldr;
        rdr = -rdr;
    }

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