#[derive(Debug)]
pub enum ButtonArg {
    Null,
    A, // change later
}

impl ButtonArg {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Null => "Null",
            Self::A => "A",
        }
    }

    pub const fn duplicate(&self) -> Self {
        match self {
            ButtonArg::Null => ButtonArg::Null,
            ButtonArg::A => ButtonArg::A,
        }
    }

    pub fn stop() { // Stops all button activities
    }

    pub fn execute(&self) {
        match self {
            Self::A => (),
            Self::Null => Self::stop(),
        }
    }
}