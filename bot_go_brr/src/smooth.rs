use crate::controller::StickState;
use crate::drive::DriveArg;
use crate::button::ButtonArg;

impl StickState {
    #[inline]
    pub fn gen_arg(self, button: ButtonArg) -> DriveArg {
        self.execute()(button, false)
    }

    pub fn execute(self) -> fn(ButtonArg, bool) -> DriveArg {
        use StickState::*;
        use DriveArg::*;
        match self {
            None => Stop,
            East => Right,
            West => Left,
            North => Forward,
            South => Backward,
            NorthEast(n) => {
                if n { Forward }
                else { Right }
            },
            NorthWest(n) => {
                if n { Forward }
                else { Left }
            },
            SouthEast(n) => {
                if n { Backward }
                else { Left }
            },
            SouthWest(n) => {
                if n { Backward }
                else { Right }
            },
        }
    }
}