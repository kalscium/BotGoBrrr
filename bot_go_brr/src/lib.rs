#![no_std]
#![recursion_limit = "1024"]

extern crate alloc;

pub mod bot;
pub mod drive_train;
pub mod config;
pub mod controls;
pub mod bytecode;
// #[cfg(feature = "record")]
pub mod record;

#[inline]
fn append_slice<T: Clone>(vec: &mut alloc::vec::Vec<T>, slice: &[T]) {
    for x in slice {
        vec.push(x.clone());
    }
}

#[inline]
fn reverse_in_place<T>(mut vec: alloc::vec::Vec<T>) -> alloc::vec::Vec<T> {
    vec.reverse();
    vec
}
