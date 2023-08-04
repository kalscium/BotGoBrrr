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

pub struct PacketRaw {
    left_stick: Stick,
    right_stick: Stick,
    button_a: bool,
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
        })
    }

    pub fn gen_arg(&self) -> DriveArg {
        let this = if let Packet::Connected(this) = self { this }
            else { return DriveArg::Stop(ButtonArg::Null, false) };
        let left: DriveArg = this.left_stick.abs_arg(self.gen_button(), false);
        let right: DriveArg = this.right_stick.abs_arg(self.gen_button(), true);
        DriveArg::add(left, right)
    }

    pub fn gen_button(&self) -> ButtonArg {
        let this = if let Packet::Connected(this) = self { this }
            else { return ButtonArg::Null };
        if this.button_a { ButtonArg::A }
        else { ButtonArg::Null }
    }
}