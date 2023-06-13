use vex_rt::prelude::*;
use crate::drive::Arg;
use crate::config::Config;

pub struct Stick {
    abs_x: u8,
    abs_y: u8,
    pos_x: bool,
    pos_y: bool,
}

impl Stick {
    pub fn new(x: i8, y: i8) -> Self {
        Self {
            abs_x: x.abs() as u8,
            abs_y: y.abs() as u8,
            pos_x: x > -1i8,
            pos_y: y > -1i8,
        }
    }

    pub fn above_threshold(&self) -> u8 { // 0 => none, 1 => x, 2 => y, 3 => both
        if self.abs_x > Config::CONTROLLER_STICK_THRESHOLD {
            if self.abs_y > Config::CONTROLLER_STICK_THRESHOLD { 3 }
            else { 1 }
        } else {
            if self.abs_y > Config::CONTROLLER_STICK_THRESHOLD { 2 }
            else { 0 }
        }
    }

    pub fn abs_arg(&self) -> Arg {
        let code: u8 = self.above_threshold();
        match code {
            0 => Arg::Stop,
            1 => {
                if self.pos_x { Arg::Right }
                else { Arg::Left }
            },
            2 => {
                if self.pos_y { Arg::Forward }
                else { Arg::Backward }
            },
            3 => Arg::Stall,
            _ => panic!("should logically not panic"),
        }
    }
}

pub struct Packet {
    left_stick: Stick,
    right_stick: Stick,
}

impl Packet {
    pub fn new(controller: &Controller) -> Packet {
        Packet {
            left_stick: Stick::new(
                controller.left_stick.get_x().unwrap(),
                controller.left_stick.get_y().unwrap(),
            ),
            right_stick: Stick::new(
                controller.right_stick.get_x().unwrap(),
                controller.right_stick.get_y().unwrap(),
            ),
        }
    }

    pub fn gen_arg(&self) -> Arg {
        let left: Arg = self.left_stick.abs_arg();
        let right: Arg = self.right_stick.abs_arg(); // Change later to relative arguments to gyro
        Arg::add(left, right)
    }
}