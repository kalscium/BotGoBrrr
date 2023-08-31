use vex_rt::prelude::{Motor, Gearset};

use crate::{config::Config, drive::Drive};

pub struct ButtonMan {
    arm: Motor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonArg {
    Null,
    Up,
    Down,
    A, // change later
}

impl ButtonArg {
    pub fn to_string(&self) -> &str {
        use ButtonArg::*;
        match self {
            Up => "Up",
            Down => "Down",
            Null => "Null",
            A => "A",
        }
    }

    pub const fn duplicate(&self) -> Self {
        use ButtonArg::*;
        match self {
            A => A,
            Up => Up,
            Down => Down,
            Null => Null,
        }
    }

    pub fn stop(man: &mut ButtonMan) { // Stops all button activities
        man.arm.move_voltage(0).unwrap();
    }

    pub fn execute(&self, man: &mut ButtonMan) {
        use ButtonArg::*;
        match self {
            Up => Self::move_arm(man, true),
            Down => Self::move_arm(man, false),
            A => (),
            Null => Self::stop(man),
        }
    }

    fn move_arm(man: &mut ButtonMan, up: bool) {
        if up {
            man.arm.move_voltage(Drive::cal_volt(Config::ARM_SPEED)).unwrap();
        } else {
            man.arm.move_voltage(-Drive::cal_volt(Config::ARM_SPEED)).unwrap();
        }
    }
}

impl ButtonMan {
    pub fn new() -> Self {
        Self {
            arm: Self::build_motor(Config::ARM_PORT, Config::ARM_RATIO, Config::ARM_REVERSE),
        }
    }

    pub fn build_motor(port: u8, ratio: Gearset, reverse: bool) -> Motor {
        unsafe {
            Motor::new(
                port,
                ratio,
                vex_rt::prelude::EncoderUnits::Rotations,
                reverse,
            )
        }.unwrap_or_else(|_| {
            panic!("Error: Could not configure / generate motor at port '{port}'!")
        })
    }
}