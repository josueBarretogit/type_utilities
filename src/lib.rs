#![warn(clippy::pedantic)]

//! # `Type_utilities`
//!
//! This crate adds more methods / utilities to the built-in  data types
//!
//! Every type is divided in modules, some types that are similar such as `String` and `&str` or
//! `vec` and `[T]` are all packaged under the same module (in this case `strings` and `vec`
//! respectively)
//!
//! Every module is optional, meaning if you only want the `bool` module then you can do
//! that by only including `bool` in the features section
//!

#[cfg(feature = "strings")]
pub mod strings;

#[cfg(feature = "vec")]
pub mod vec;

#[cfg(feature = "bool")]
pub mod bool;

#[cfg(feature = "option")]
pub mod option;

#[cfg(feature = "result")]
pub mod result;

pub mod hashmap;
