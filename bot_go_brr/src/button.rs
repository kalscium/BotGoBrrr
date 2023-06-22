#[derive(Debug)]
pub enum ButtonArg {
    Null,
    X, // change later
}

impl ButtonArg {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Null => "Null",
            Self::X => "X",
        }
    }

    pub fn check(&self, other: Self) -> bool {
        if let other = self { true } else { false } // Check for same type of misc arg
    }
}