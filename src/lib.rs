#![feature(iter_arith, slice_patterns)]

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

mod dice;
mod error;
mod result;

#[cfg(test)]
mod tests;

pub use dice::Dice;
pub use error::DiceParseError;
pub use result::{ DiceResult, DiceResultGenerator, GenFn, VecResult };
