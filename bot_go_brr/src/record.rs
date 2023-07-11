extern crate alloc;

use crate::drive::DriveArg;
use alloc::string::ToString;

pub struct Record {
    arg: DriveArg,
    held: u16,
}

impl Record {
    pub fn new(arg: DriveArg) -> Self {
        Self {
            arg,
            held: 0,
        }
    }

    pub fn clear(&mut self, arg: DriveArg) {
        self.arg = arg;
        self.held = 0;
    }
    
    pub fn record(&mut self, arg: DriveArg) -> DriveArg {
        if self.arg.to_strings() == arg.to_strings() { self.held += 1 }
        else {
            self.log();
            self.clear(arg.duplicate());
        }; arg
    }

    pub fn log(&self) {
        use crate::utils::Log::*;
        let (name, button) = self.arg.to_strings();
        List(
            &Wrap("[", &List(
                &Title("held"), ": ",
                &String(self.held.to_string()),
            ), "]"), " ",
            &List(
                &Title(name), "",
                &Wrap("(", &Title(button), ")"),
            ),
        ).log();
    }
}