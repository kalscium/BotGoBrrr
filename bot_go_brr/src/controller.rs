use vex_rt::prelude::*;
use crate::drive::DriveArg;
use crate::button::ButtonArg;
use crate::config::Config;
use crate::relative::{Rel, RelAlign};

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

    pub fn get_rel_align(&self) -> Option<RelAlign> {
        let code: u8 = self.above_threshold();
        match code {
            0 => None,

            1 if self.pos_x => Some(RelAlign::E),
            1 => Some(RelAlign::W),

            2 if self.pos_y => Some(RelAlign::N),
            2 => Some(RelAlign::S),

            // if both
            3 => match (&self.pos_x, &self.pos_y) {
                (true, true) => Some(RelAlign::NE),
                (false, true) => Some(RelAlign::NW),
                (true, false) => Some(RelAlign::SE),
                (false, false) => Some(RelAlign::SW),
            },
            _ => panic!("should logically not panic"),
        }
    }

    pub fn abs_arg(&self, button: ButtonArg) -> DriveArg {
        let code: u8 = self.above_threshold();
        match code {
            0 => DriveArg::Stop(button),

            1 if self.pos_x => DriveArg::Right(button),
            1 => DriveArg::Left(button), // <else>

            2 if self.pos_y => DriveArg::Forward(button),
            2 => DriveArg::Backward(button), // <else>

            3 => DriveArg::Stall(button),
            _ => panic!("should logically not panic"),
        }
    }

    pub fn rel_arg(&self, button: ButtonArg, rel: &mut Rel) -> DriveArg {
        // align relative
        if let Some(rel_align) = self.get_rel_align() { rel.align(rel_align) }
        else { return DriveArg::Stop(button) };
        
        rel.get_arg(button) // return relative driveargs
    }
}

pub struct Packet {
    left_stick: Stick,
    right_stick: Stick,
    button_a: bool,
    button_b: bool,
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
            button_b: controller.b.is_pressed().unwrap(),
        }
    }

    pub fn gen_arg(&self, rel: &mut Rel) -> DriveArg {
        let left: DriveArg = self.left_stick.abs_arg(self.gen_button());
        let right: DriveArg = self.right_stick.rel_arg(self.gen_button(), rel);
        DriveArg::add(left, right)
    }

    pub fn gen_button(&self) -> ButtonArg {
        if self.button_a { ButtonArg::A }
        else if self.button_b { ButtonArg::Quit }
        else { ButtonArg::Null }
    }
}