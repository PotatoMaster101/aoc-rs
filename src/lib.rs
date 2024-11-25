#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod area;
pub mod input;
pub mod grid;
pub mod math;
pub mod pos;
