use super::drive::DriveArg;
use super::button::ButtonArg;

pub struct Position {
    array_idx: u8,
    idx: u16,
}

impl Position {
    pub const fn new() -> Self {
        Self {
            array_idx: 0,
            idx: 0,
        }
    }

    pub fn advance(&mut self, len: u16) { // Run every tick
        if self.idx < len -1 { self.idx += 1; return; }
        self.array_idx += 1;
        self.idx = 0;
    }

    pub fn get(&mut self, args: &[ArgWrapper]) -> Option<DriveArg> { // Run every tick
        if self.array_idx >= args.len() as u8 { return None }
        let wrapped_arg: &ArgWrapper = &args[self.array_idx as usize];
        self.advance(wrapped_arg.1);
        Some(wrapped_arg.0.duplicate())
    }
}

pub struct ArgWrapper(DriveArg, u16);

pub struct Algor {
    wrapped_args: &'static [ArgWrapper],
}

// Algorithms
impl Algor {
    pub const AUTONOMOUS: Algor = Algor::new(&[
        ArgWrapper(DriveArg::Forward(ButtonArg::Null, false), 12),
    ]);
}

impl Algor {
    pub const fn new(wrapped_args: &'static [ArgWrapper]) -> Self {
        Self {
            wrapped_args,
        }
    }

    pub fn get(&self, pos: &mut Position) -> Option<DriveArg> { // Run every tick
        pos.get(self.wrapped_args)
    }
}
