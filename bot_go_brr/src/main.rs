#![no_std]
#![no_main]

use vex_rt::prelude::*;

mod drive;
mod config;
pub mod bot;
mod controller;
mod utils;
mod button;

entry!(bot::Bot);
