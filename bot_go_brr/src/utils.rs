use core::slice::Iter;
use vex_rt::prelude::*;

pub fn log(tick: &u128, title: &str, body: &str) {
    println!("\x1b[35;1m[\x1b[0m{0}\x1b[35;1m] \x1b[0m{1}\x1b[35;1m => \x1b[0m{2}", *tick, title, body);
}

pub fn list_ports(ports: Iter<&SmartPort>) {
    println!("\x1b[35;1m===( Port Mapping )===\x1b[0m");

    let mut i: u8 = 0;
    for port in ports {
        i += 1;
        println!("    \x1b[35;1m- Port (\x1b[0m{0}\x1b[35;1m) => \x1b[0m{1:?}", i, port.plugged_type());
    }
}