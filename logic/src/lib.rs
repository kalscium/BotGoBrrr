//! A platform independant library for holding all the logic of the robot

#![no_std]

extern crate alloc;

pub mod log;
pub mod magic;
pub mod drive;
pub mod odom;
pub mod pid;
