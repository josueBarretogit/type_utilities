#![warn(clippy::pedantic)]

#[cfg(feature = "strings")]
pub mod strings;
 
#[cfg(feature = "vec")]
pub mod vec;

#[cfg(feature = "bool")]
pub mod bool;
