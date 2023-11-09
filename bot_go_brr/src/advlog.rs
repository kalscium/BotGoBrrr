use alloc::string::{String, ToString};
use alloc::vec::Vec;
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
pub struct Advlog(Vec<(DriveArg, u16)>);

impl Advlog {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn parse(&mut self, arg: DriveArg) {
        if let Some((x, i)) = self.0.last_mut() {
            if *x == arg {
                *i += 1;
                return; // so it doesn't execute under
            }
        } self.0.push((arg, 1));
    }

    pub fn export(&self) {
        for (x, i) in self.0.iter() {
            let (drive, button, precise) = x.to_strings();
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
                yellow(&i.to_string()),
            ]);
        }
    }
}