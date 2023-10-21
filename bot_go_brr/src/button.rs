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
    pub fn to_string(self) -> &'static str {
        use ButtonArg::*;
        match self {
            Up => "Up",
            Down => "Down",
            Null => "Null",
            A => "A",
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
        loop {
            if let Ok(x) = unsafe {
                Motor::new(
                    port,
                    ratio,
                    vex_rt::prelude::EncoderUnits::Degrees,
                    reverse,
                )
            } { return x }
        }
    }

    pub fn stop(&mut self) { // Stops all button activities
        let _ = self.arm.move_voltage(0); // Stop the arm
    }

    pub fn move_arm(&mut self, up: bool) {
        if up {
            let _ = self.arm.move_voltage(Drive::cal_volt(Config::ARM_SPEED));
        } else {
            let _ = self.arm.move_voltage(-Drive::cal_volt(Config::ARM_SPEED));
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