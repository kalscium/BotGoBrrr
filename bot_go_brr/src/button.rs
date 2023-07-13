use crate::utils::quit;

#[derive(Debug)]
pub enum ButtonArg {
    Null,
    Quit,
    A, // change later
}

impl ButtonArg {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Null => "Null",
            Self::A => "A",
            Self::Quit => "Quit",
        }
    }

    pub const fn duplicate(&self) -> Self {
        match self {
            ButtonArg::Null => ButtonArg::Null,
            ButtonArg::A => ButtonArg::A,
            ButtonArg::Quit => ButtonArg::Quit,
        }
    }

    pub fn stop() { // Stops all button activities
    }

    pub fn execute(&self) {
        match self {
            Self::Quit => quit(&0u128, "Hit quit button!"),
            Self::A => (),
            Self::Null => Self::stop(),
        }
    }
}