#![feature(core, plugin, slice_patterns)]
#![plugin(regex_macros)]
extern crate rand;
extern crate regex;

mod dice;
mod error;
mod result;

#[cfg(test)]
mod tests;

pub use dice::Dice;
pub use error::DiceParseError;
pub use result::{ DiceResult, DiceResultGenerator, GenFn };
