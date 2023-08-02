use vex_rt::prelude::*;
use crate::drive::DriveArg;
use crate::button::ButtonArg;
use crate::config::Config;

pub enum StickState {
    None,
    North,
    NorthEast(bool), // North Larger?
    East,
    SouthEast(bool), // South Larger?
    South,
    SouthWest(bool), // South Larger?
    West,
    NorthWest(bool), // North Larger?
}

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

    pub fn gen_state(&self) -> StickState {
        match (self.abs_y > Config::CONTROLLER_STICK_THRESHOLD, self.abs_x > Config::CONTROLLER_STICK_THRESHOLD) {
            (false, false) => StickState::None,
            (true, false) => if self.pos_y { StickState::North } else { StickState::South },
            (false, true) => if self.pos_x { StickState::East } else { StickState::West },
            (true, true) => (match (self.pos_y, self.pos_x) {
               (true, true) => StickState::NorthEast,
               (true, false) => StickState::NorthWest,
               (false, true) => StickState::SouthEast,
               (false, false) => StickState::SouthWest,
            }(self.abs_y > self.abs_x))
        }
    }

    pub fn abs_arg(&self, button: ButtonArg, right:bool) -> DriveArg {
        (match self.gen_state() {
            StickState::None => DriveArg::Stop,
            StickState::North => DriveArg::Forward,
            StickState::East => DriveArg::Right,
            StickState::South => DriveArg::Backward,
            StickState::West => DriveArg::Left,
            _ => DriveArg::Stall,
        })(button, right)
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