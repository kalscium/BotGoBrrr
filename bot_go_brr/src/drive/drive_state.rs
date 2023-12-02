extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use safe_vex::controller::joystick::JoyStick;
use crate::config::Config;

/// The current state of the voltage for each of the drive-train's motors
#[derive(Debug, Clone, Copy)]
pub struct DriveState {
    /// Top-let motor of the robot
    pub l1: i32,
    /// Bottom-let motor of the robot
    pub l2: i32,
    /// Top-right motor of the robot
    pub r1: i32,
    /// Bottom-right motor of the robot
    pub r2: i32,
    /// Arm of the robot
    pub arm: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum DriveArg {
    /// Forwards by a multiplier
    Forward(u8, bool),
    /// Forwards by a multiplier
    Backward(u8, bool),
    /// Turn left
    TLeft(u8, bool),
    /// Turn right
    TRight(u8, bool),
    /// Strafe left
    SLeft,
    /// Strafe right
    SRight,
    /// Move arm (up or down)
    Arm(bool),
}

impl DriveArg {
    #[inline]
    pub fn new(left_stick: JoyStick, right_stick: JoyStick, l2: bool, r2: bool, up: bool, down: bool) -> Box<[DriveArg]> {
        use DriveArg as D;

        let left_stick = left_stick.clamp(Config::CONTROLLER_STICK_MIN);
        let right_stick = right_stick.clamp(Config::CONTROLLER_STICK_MIN);
        let mut args = Vec::new();
        let mut movement_arg = false;

        let precise = if right_stick.x_larger() { right_stick.x != 0 } else { right_stick.y != 0 };

        let joy_stick = if precise { right_stick } else { left_stick };
        let stick = if joy_stick.x_larger() { joy_stick.x } else { joy_stick.y };
        if stick != 0 {args.push({
            movement_arg = true; // you can't do two different movements at the same time

            match (joy_stick.x_larger(), stick.is_positive()) {
                (true, true) => D::TRight(stick as u8, precise), // Turn right
                (true, false) => D::TLeft(stick.unsigned_abs(), precise), // Turn left
                (false, true) => D::Forward(stick as u8, precise), // Move forwards
                (false, false) => D::Backward(stick.unsigned_abs(), precise), // Move backwards
            }
        })};

        if !movement_arg {
            if l2 {
                args.push(D::SLeft);
            } else if r2 {
                args.push(D::SRight);
            }
        }

        if up {
            args.push(D::Arm(true));
        } else if down {
            args.push(D::Arm(false));
        }

        args.into_boxed_slice()
    }
}

impl DriveState {
    #[inline]
    pub fn new(args: &[DriveArg]) -> Self {
        let mut state = Self {
            l1: 0,
            l2: 0,
            r1: 0,
            r2: 0,
            arm: 0,
        };

        use DriveArg as D;
        for arg in args {match arg {
            D::Forward(x, precise) => {
                let voltage = calc_joy_voltage(*x, Config::DRIVE_FORWARD_SPEED, *precise);
                state.l1 = voltage;
                state.l2 = voltage;
                state.r1 = voltage;
                state.r2 = voltage;
            },

            D::Backward(x, precise) => {
                let voltage = calc_joy_voltage(*x, Config::DRIVE_BACKWARD_SPEED, *precise);
                state.l1 = -voltage;
                state.l2 = -voltage;
                state.r1 = -voltage;
                state.r2 = -voltage;
            },

            D::TLeft(x, precise) => {
                let voltage = calc_joy_voltage(*x, Config::DRIVE_TURN_SPEED, *precise);
                state.l1 = -voltage;
                state.l2 = -voltage;
                state.r1 = voltage;
                state.r2 = voltage;
            },

            D::TRight(x, precise) => {
                let voltage = calc_joy_voltage(*x, Config::DRIVE_TURN_SPEED, *precise);
                state.l1 = voltage;
                state.l2 = voltage;
                state.r1 = -voltage;
                state.r2 = -voltage;
            },

            D::SLeft => {
                let voltage = (i32::MAX as f64 * (100f64 / Config::DRIVE_STRAFE_SPEED.clamp(0, 100) as f64)) as i32;
                state.l1 = voltage;
                state.l2 = -voltage;
                state.r1 = -voltage;
                state.r2 = voltage;
            },

            D::SRight => {
                let voltage = (i32::MAX as f64 * (100f64 / Config::DRIVE_STRAFE_SPEED.clamp(0, 100) as f64)) as i32;
                state.l1 = -voltage;
                state.l2 = voltage;
                state.r1 = voltage;
                state.r2 = -voltage;
            },

            D::Arm(up) => {
                let voltage = (i32::MAX as f64 * (100f64 / Config::ARM_SPEED.clamp(0, 100) as f64)) as i32;
                state.arm = if *up { voltage } else { -voltage };
            },
        }}

        state
    }
}

/// Calculates the voltage to use for the motors connected to the joysticks
#[inline]
pub fn calc_joy_voltage(stick: u8, percent: u8, precise: bool) -> i32 {
    if precise {
        return (i32::MAX as f64
            * (percent.clamp(0, 100) as f64 / 100f64)
        ) as i32;
    }

    // Daniel's magic number
    const OFFSET_POWER: (f64, u16) = {
        let mut offset = 2147483648f64;
        let mut power = 0;
        while offset > 1f64 {
            offset /= Config::EXPO_MULTIPLIER;
            power += 1;
        } (offset, power)
    };
    const SEGMENT: f32 = OFFSET_POWER.1 as f32 / 128f32;

    ((powi(Config::EXPO_MULTIPLIER, (stick as f32 * SEGMENT) as u16) * OFFSET_POWER.0) // to calculate the exponential increase
        * (percent.clamp(0, 100) as f64 / 100f64) // to normalize the voltage to the percentage (and prevent overflow)
    ).clamp(i32::MIN as f64, i32::MAX as f64) as i32
}

#[inline]
fn powi(x: f64, i: u16) -> f64 {
    let mut out = 1f64;
    for _ in 0..i {
        out *= x;
    } out
}