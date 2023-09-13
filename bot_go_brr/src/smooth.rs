use crate::controller::StickState;
use crate::drive::DriveArg;
use crate::button::ButtonArg;
use crate::config::Config;

pub struct Smooth {
    modifier: StickState,
    action: StickState,
    ticks: u16,
}

impl Smooth {
    pub fn new() -> Self {
        Self {
            action: StickState::None,
            modifier: StickState::None,
            ticks: 0,
        }
    }

    pub fn reset(&mut self, state: StickState) {
        if self.ticks >= Config::MIN_TICK_SMOOTH as u16 { self.modifier = self.action; }
        self.action = state;
        self.ticks = 0;
    }

    pub fn gen_arg(&mut self, state: StickState, button: ButtonArg) -> DriveArg {
        if state == self.action { self.ticks += 1 }
        else { self.reset(state) }
        self.execute()(button, false)
    }

    pub fn execute(&self) -> fn(ButtonArg, bool) -> DriveArg {
        use StickState::*;
        match self.action {
            None => DriveArg::Stop,
            East => DriveArg::Right,
            West => DriveArg::Left,
            North => DriveArg::Forward,
            South => DriveArg::Backward,
            NorthEast(n) => match self.modifier {
                North => DriveArg::Right,
                NorthWest(_) => DriveArg::Right,
                _ if n => DriveArg::Forward,
                _ => DriveArg::Right,
            },
            NorthWest(n) => match self.modifier {
                North => DriveArg::Left,
                NorthEast(_) => DriveArg::Left,
                _ if n => DriveArg::Forward,
                _ => DriveArg::Left,
            },
            SouthEast(n) => match self.modifier {
                South => DriveArg::Left,
                SouthWest(_) => DriveArg::Left,
                _ if n => DriveArg::Backward,
                _ => DriveArg::Right,
            },
            SouthWest(n) => match self.modifier {
                South => DriveArg::Right,
                SouthEast(_) => DriveArg::Right,
                _ if n => DriveArg::Backward,
                _ => DriveArg::Left,
            },
        }
    }
}