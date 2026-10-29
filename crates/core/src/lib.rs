#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub mod adapters;
pub mod address;
pub mod arithmetic;
pub mod errors;
pub mod msgs;

#[cfg(test)]
mod tests;
