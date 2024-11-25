#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod geo;
pub mod input;
pub mod math;
