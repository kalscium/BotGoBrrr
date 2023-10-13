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

    pub fn gen_arg(&self, button: ButtonArg, precise: bool) -> DriveArg {
        (match (self.abs_y > Config::CONTROLLER_STICK_THRESHOLD, self.abs_x > Config::CONTROLLER_STICK_THRESHOLD) {
            (false, false) => DriveArg::Stop,
            (true, false) => if self.pos_y { DriveArg::Forward } else { DriveArg::Backward },
            (false, true) => if self.pos_x { DriveArg::Right } else { DriveArg::Left },
            (true, true) => match (self.pos_y, self.pos_x, self.abs_y > self.abs_x) {
                (true, _, true) => DriveArg::Forward,
                (false, _, true) => DriveArg::Backward,
                (_, false, false) => DriveArg::Left,
                (_, true, false) => DriveArg::Right,
            }
        })(button, precise)
    }
}

pub struct PacketRaw {
    left_stick: Stick,
    right_stick: Stick,
    button_a: bool,
    button_up: bool,
    button_down: bool,
}

pub enum Packet {
    Disconnected,
    Connected(PacketRaw),
}

macro_rules! safe_unwrap {
    ($item:expr) => {{
        if let Ok(x) = $item {
            x
        } else {
            return Packet::Disconnected;
        }
    }}
}

impl Packet {
    pub fn new(controller: &Controller) -> Packet {
        Packet::Connected(PacketRaw {
            left_stick: Stick::new(
                safe_unwrap!(controller.left_stick.get_x()),
                safe_unwrap!(controller.left_stick.get_y()),
            ),
            right_stick: Stick::new(
                safe_unwrap!(controller.right_stick.get_x()),
                safe_unwrap!(controller.right_stick.get_y()),
            ),
            
            button_a: safe_unwrap!(controller.a.is_pressed()),
            button_up: safe_unwrap!(controller.up.is_pressed()),
            button_down: safe_unwrap!(controller.down.is_pressed()),
        })
    }

    pub fn gen_arg(&self) -> DriveArg {
        let this = if let Packet::Connected(this) = self { this }
            else { return DriveArg::Stop(ButtonArg::Null, false) };
        let left: DriveArg = this.left_stick.gen_arg(self.gen_button(), false);
        let right: DriveArg = this.right_stick.gen_arg(self.gen_button(), true);
        DriveArg::add(left, right)
    }

    pub fn gen_button(&self) -> ButtonArg {
        let this = if let Packet::Connected(this) = self { this }
            else { return ButtonArg::Null };
        if this.button_a { ButtonArg::A }
        else if this.button_up { ButtonArg::Up }
        else if this.button_down { ButtonArg::Down }
        else { ButtonArg::Null }
    }
}