#![warn(clippy::pedantic)]

pub mod strings;
 
#[cfg(feature = "vec")]
pub mod vec;

#[cfg(feature = "vec")]
pub mod bool;
