use vex_rt::prelude::*;

pub fn log(tick: &u128, title: &str, body: &str) {
    println!("[{0}] {1} => {2}", *tick, title, body);
}