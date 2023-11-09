use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use hashbrown::HashMap;
use vex_rt::io::println;
use crate::drive::DriveArg;

/// Used to format text with colour
/// # Examples
/// ```rust
/// use soulog::*;
/// colour_format![pink("["), none("Logger"), pink("] "), none("Example Log")];
/// // outputs: [Logger] Example Log
/// // but with colour
/// ```
#[macro_export]
macro_rules! colour_format { // Verbose ugly stuff I can't read
    ($(
        $(none($none:expr))?
        $(blue($blue:expr))?
        $(pink($pink:expr))?
        $(white($white:expr))?
        $(green($green:expr))?
        $(cyan($cyan:expr))?
        $(red($red:expr))?
        $(black($black:expr))?
        $(yellow($yellow:expr))?
    ),*) => {{
        let mut string = String::new();
        $(
            $(string.push_str("\x1b[0m"); string.push_str($none);)?
            $(string.push_str("\x1b[34m"); string.push_str($blue);)?
            $(string.push_str("\x1b[35m"); string.push_str($pink);)?
            $(string.push_str("\x1b[37m"); string.push_str($white);)?
            $(string.push_str("\x1b[32m"); string.push_str($green);)?
            $(string.push_str("\x1b[36m"); string.push_str($cyan);)?
            $(string.push_str("\x1b[31m"); string.push_str($red);)?
            $(string.push_str("\x1b[30m"); string.push_str($black);)?
            $(string.push_str("\x1b[33m"); string.push_str($yellow);)?
        )* string.push_str("\x1b[0m");
        string
    }}
}

/// `(String) Log => (abs_tick, held_for)`
pub struct Advlog(HashMap<DriveArg, Vec<(usize, u16)>>);

impl Advlog {
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn parse(&mut self, tick: usize, arg: DriveArg) {
        if let Some(x) = self.0.get_mut(&arg) {
            let x = x.last_mut().unwrap(); // vec cannot be empty
            if tick - x.0 == x.1 as usize { x.1 += 1; }
            else {
                self.0.get_mut(&arg)
                    .unwrap() // Has to exist
                    .push((tick, 1));
            }
        } else { self.0.insert(arg, vec![(tick, 1)]); }
    }

    // todo: Fix the weird issue where it gets repeat printed multiple times
    pub fn export(&self) {
        let mut last = 0usize;
        loop {
            let mut found = false;
            for (i, x) in self.0.iter() {
                for x in x.iter() {
                    if x.0 == last {
                        found = true;
                        last += x.1 as usize;
                        let (drive, button, precise) = i.to_strings();
                        let precise = if precise {
                            colour_format![blue("("), yellow("precise"), blue(") ")]
                        } else {
                            String::new()
                        };

                        println!("{}", colour_format![
                            none(&precise),
                            cyan(drive),
                            blue("("),
                            yellow(button),
                            blue(")"),
                            blue(" for "),
                            yellow(&x.1.to_string()),
                        ]);
                    }
                }
            } if !found { break };
        }
    }
}