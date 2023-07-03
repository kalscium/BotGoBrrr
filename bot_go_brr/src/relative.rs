extern crate alloc;

use alloc::string::ToString;
use crate::{
    drive::DriveArg,
    config::Config,
    button::ButtonArg,
};

pub enum RelAlign {
    N,
    E,
    W,
    S,
    NE,
    SE,
    SW,
    NW,
}

impl RelAlign {
    // to align, subtract value from dif ( if north and right aligned )
    pub fn value(&self) -> i16 { // All values are aligned to the right
        match self {
            Self::N => 0,
            Self::NE => 45,
            Self::E => 90,
            Self::SE => 135,
            Self::S => 180,
            Self::SW => -135,
            Self::W => -90,
            Self::NW => -45,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::N => "North",
            Self::NE => "NorthEast",
            Self::E => "East",
            Self::SE => "SouthEast",
            Self::S => "South",
            Self::SW => "SouthWest",
            Self::W => "West",
            Self::NW => "NorthWest",
        }
    }
}

pub struct Rel {
    dif: u8, // max inclusive 180
    left: bool,
    align: RelAlign,
}

impl Rel {
    pub fn new() -> Self {
        Self {
            dif: 0,
            left: true,
            align: RelAlign::N,
        }
    }

    fn overflow(left: bool, dif: i16) -> (bool, u8) { // run when overflow
        let new_left: bool = left ^ true; // flips it ( true if false, false if true )
        let new_dif: i16 = 180 * 2 - dif; // value set to 180 - overflow
        (new_left, new_dif as u8) // package safe values
    }

    fn underflow(left: bool, dif: i16) -> (bool, u8) { // run when underflow
        let new_left: bool = left ^ true;
        let new_dif: i16 = -dif; // value set to pos( underflow )
        (new_left, new_dif as u8) // packages safe values
    }

    fn safe_set(&mut self, value: i16) {
        if value > 180 {
            (self.left, self.dif) = Self::overflow(self.left, value);
        } else if value < 0 {
            (self.left, self.dif) = Self::underflow(self.left, value);
        } else {
            self.dif = value as u8;
        }
    }

    pub fn add(&mut self, left: bool, mut value: i16) {
        if self.left ^ left { value = -value }; // align value argument
        self.safe_set(self.dif as i16 + value);
    }

    pub fn sub(&mut self, left: bool, mut value: i16) {
        if self.left ^ left { value = -value }; // align value argument
        self.safe_set(self.dif as i16 - value);
    }

    pub fn record(&mut self, arg: &DriveArg) { // Run every tick
        let value: i16 = self.dif as i16 + match arg {
            DriveArg::Left(_) if self.left => Config::DEGREES_PER_TICK,
            DriveArg::Left(_) => -(Config::DEGREES_PER_TICK),
            DriveArg::Right(_) if self.left => -(Config::DEGREES_PER_TICK),
            DriveArg::Right(_) => Config::DEGREES_PER_TICK,
            _ => 0,
        }; self.safe_set(value);
    }

    pub fn align(&mut self, align: RelAlign) -> &Self {
        // Realign to north ( default )
        self.add(false, self.align.value());
        self.align = RelAlign::N;

        // Align to desired
        self.sub(false, align.value());
        self.align = align;

        self
    }

    pub fn get_arg(&self, button: ButtonArg) -> DriveArg {
        if self.dif <= Config::ROTATION_THRESHOLD { return DriveArg::Forward(button) } // If dif to desired is within threshold then move forward
        if self.left { DriveArg::Right(button) } // if too left correct by turning right
        else { DriveArg::Left(button) } // if too right correct by turning left
    }

    pub fn log(&self, tick: &u128) {
        use crate::utils::Log::*;
        Base(
            tick,
            "Rel Rotation",
            &List(
                &Wrap("(", &List(
                    &Title("left"), ": ",
                    &String(self.left.to_string()),
                ), ")"), " ",
                &List(
                    &String(self.dif.to_string()), " > ",
                    &Title(self.align.to_string()),
                ),
            ),
        ).log();
    }
}