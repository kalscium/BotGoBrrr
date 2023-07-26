use vex_rt::prelude::*;
use crate::drive::DriveArg;
use crate::button::ButtonArg;
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
            abs_x: x.unsigned_abs(),
            abs_y: y.unsigned_abs(),
            pos_x: x > -1i8,
            pos_y: y > -1i8,
        }
    }

    pub fn above_threshold(&self) -> u8 { // 0 => none, 1 => x, 2 => y, 3 => both
        if self.abs_x > Config::CONTROLLER_STICK_THRESHOLD {
            if self.abs_y > Config::CONTROLLER_STICK_THRESHOLD { 3 }
            else { 1 }
        } else if self.abs_y > Config::CONTROLLER_STICK_THRESHOLD { 2 }
        else { 0 }
    }

    pub fn abs_arg(&self, button: ButtonArg, right: bool) -> DriveArg {
        let code: u8 = self.above_threshold();
        match code {
            0 => DriveArg::Stop(button, false),

            1 if self.pos_x => DriveArg::Right(button, right),
            1 => DriveArg::Left(button, right), // <else>

            2 if self.pos_y => DriveArg::Forward(button, right),
            2 => DriveArg::Backward(button, right), // <else>

            3 => DriveArg::Stall(button, false),
            _ => panic!("should logically not panic"),
        }
    }
}

pub struct Packet {
    left_stick: Stick,
    right_stick: Stick,
    button_a: bool,
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
            button_a: controller.a.is_pressed().unwrap(),
        }
    }

    pub fn gen_arg(&self) -> DriveArg {
        let left: DriveArg = self.left_stick.abs_arg(self.gen_button(), false);
        let right: DriveArg = self.right_stick.abs_arg(self.gen_button(), true);
        DriveArg::add(left, right)
    }

    pub fn gen_button(&self) -> ButtonArg {
        if self.button_a { ButtonArg::A }
        else { ButtonArg::Null }
    }
}