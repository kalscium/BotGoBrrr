#![no_std]
#![recursion_limit = "1024"]

extern crate alloc;

pub mod bot;
pub mod drive_train;
pub mod config;
pub mod controls;
pub mod bytecode;
#[cfg(record)]
pub mod record;

#[inline]
fn append_slice<T: Clone>(vec: &mut alloc::vec::Vec<T>, slice: &[T]) {
    for x in slice {
        vec.push(x.clone());
    }
}
