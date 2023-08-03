#![no_std]
#![no_main]

use vex_rt::prelude::*;

mod drive;
mod config;
mod bot;
mod controller;
mod utils;
mod button;
mod algor;
mod record;
mod smooth;
entry!(bot::Bot);
