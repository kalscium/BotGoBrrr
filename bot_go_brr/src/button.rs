use vex_rt::prelude::{Motor, Gearset};
use crate::{config::Config, drive::Drive};

pub struct ButtonMan {
    arm: Motor,
    held: u16,
    last: ButtonArg,
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
        use ButtonArg as B;
        match self {
            B::A => B::A,
            B::Up => B::Up,
            B::Down => B::Down,
            B::Null => B::Null,
        }
    }
}

impl ButtonMan {
    pub fn new() -> Self {
        Self {
            arm: Self::build_motor(Config::ARM_PORT, Config::ARM_RATIO, Config::ARM_REVERSE),
            held: 0,
            last: ButtonArg::Null,
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

    pub fn stop(&mut self) { // Stops all button activities
        self.arm.move_voltage(0).unwrap(); // Stop the arm
    }

    pub fn move_arm(&mut self, up: bool) {
        if self.held >= Config::ARM_HOLD_LIMIT { self.stop(); return; }
        if up {
            self.arm.move_voltage(Drive::cal_volt(Config::ARM_SPEED)).unwrap();
        } else {
            self.arm.move_voltage(-Drive::cal_volt(Config::ARM_SPEED)).unwrap();
        }
    }

    pub fn execute(&mut self, arg: ButtonArg) {
        use ButtonArg as B;
        match arg {
            B::Null => self.stop(),
            B::Up => self.move_arm(true),
            B::Down => self.move_arm(false),
            B::A => (),
        }

        if arg == self.last { self.held += 1 }
        else { self.held = 0 }
    }
}