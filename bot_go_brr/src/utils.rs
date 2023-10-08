extern crate alloc;

use core::slice::Iter;
use vex_rt::prelude::*;
use alloc::{fmt::{format, Display, self}, string::{ToString, String}};

pub enum Log<'a> {
    String(String), // basic string
    Base(&'a u32, &'a str, &'a Log<'a>), // Base of every log ( tick, title, body )
    Wrap(&'a str, &'a Log<'a>, &'a str), // wrapping eg. '['value']'
    List(&'a Log<'a>, &'a str, &'a Log<'a>), // List with separator
    Title(&'a str), // Blue title
    Void,
}

impl<'a> Display for Log<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Log::*;
        write!(
            f,
            "{}",
            match self {
                // basic
                String(x) => x.clone(),
                Title(x) => Self::blue(x),
                Void => alloc::string::String::new(),

                // not basic
                Wrap(a, log, b) => format(format_args!( "{0} {1} {2}", Self::pink(a), log, Self::pink(b) )),
                Base(tick, title, body) => format(format_args!( "{0} {1} {2} {3}", Wrap("[", &String(tick.to_string()), "]"), Self::blue(title), Self::pink("=>"), body)),
                List(x, sep, y) => format(format_args!( "{0}{1}{2}", x, Self::pink(sep), y)),
            }
        )
    }
}

impl<'a> Log<'a> {
    pub fn log(self) {
        println!("{}", self.to_string());
    }

    // colours
    fn colour(log: &str, colour: &str) -> String {
        format(format_args!( "{colour}{log}\x1b[0m" ))
    }

    fn blue(log: &str) -> String { Self::colour(log, "\x1b[34m") }
    fn pink(log: &str) -> String { Self::colour(log, "\x1b[35;1m") }
}

pub fn list_ports(ports: Iter<&SmartPort>) {
    use Log::*;
    Wrap("===(", &Title("Port Mapping"), ")===").log();

    for (i, port) in ports.enumerate() {
        List(
            &List(
                &List(&Void, "- ", &Title("Port")), "",
                &Wrap("(", &String((i + 1).to_string()), ")"),
            ), " => ",
            &String(format(format_args!("{:?}", port.plugged_type()))),
        ).log();
    }
}

#[macro_export]
macro_rules! niceif {
    (if $cond:expr, $one:expr, else $two:expr) => {
        if $cond {
            $one
        } else {
            $two
        }
    }
}