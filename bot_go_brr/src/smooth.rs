use crate::controller::StickState;
use crate::drive::DriveArg;
use crate::button::ButtonArg;

pub struct Smooth {
    is_forward: bool,
    action: StickState,
}

impl Smooth {
    pub fn new() -> Self {
        Self {
            action: StickState::None,
            is_forward: true,
        }
    }

    #[inline]
    pub fn gen_arg(&mut self, state: StickState, button: ButtonArg) -> DriveArg {
        self.action = state;
        self.execute()(button, false)
    }

    pub fn execute(&mut self) -> fn(ButtonArg, bool) -> DriveArg {
        use StickState::*;
        use DriveArg::*;
        match self.action {
            None => Stop,
            East => match self.is_forward {
                true => Right,
                false => Left,
            },
            West => match self.is_forward {
                true => Left,
                false => Right,
            },
            North => {
                self.is_forward = true;
                Forward
            },
            South => {
                self.is_forward = false;
                Backward
            },
            NorthEast(n) => {
                self.is_forward = true;
                if n { Forward }
                else { Right }
            },
            NorthWest(n) => {
                self.is_forward = true;
                if n { Forward }
                else { Left }
            },
            SouthEast(n) => {
                self.is_forward = false;
                if n { Backward }
                else { Left }
            },
            SouthWest(n) => {
                self.is_forward = false;
                if n { Backward }
                else { Right }
            },
        }
    }
}