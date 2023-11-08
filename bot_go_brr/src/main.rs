#![no_std]
#![no_main]

extern crate alloc;

use vex_rt::prelude::*;

mod drive;
mod config;
mod bot;
mod controller;
mod utils;
mod button;
mod algor;
mod advlog;
entry!(bot::Bot);
