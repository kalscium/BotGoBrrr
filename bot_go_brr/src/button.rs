use crate::utils::quit;

#[derive(Debug)]
pub enum ButtonArg {
    Null,
    Quit,
    X, // change later
}

impl ButtonArg {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Null => "Null",
            Self::X => "X",
            Self::Quit => "Quit",
        }
    }

    pub const fn duplicate(&self) -> Self {
        match self {
            ButtonArg::Null => ButtonArg::Null,
            ButtonArg::X => ButtonArg::X,
            ButtonArg::Quit => ButtonArg::Quit,
        }
    }

    pub fn stop() {
    }

    pub fn execute(&self) {
        match self {
            Self::Quit => quit(&0u128, "Hit quit button!"),
            Self::X => (),
            Self::Null => Self::stop(),
        }
    }
}