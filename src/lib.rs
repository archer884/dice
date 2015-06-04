#![feature(plugin, slice_patterns)]
#![plugin(regex_macros)]
extern crate regex;
mod dice_tests;
use regex::Regex;

static DICE_CMD_PATTERN: Regex = regex!(r"\d(d\d+)?");

#[derive(Debug)]
pub enum DiceParseError {
    InvalidExpression,
}

impl ::std::error::Error for DiceParseError {
    fn description(&self) -> &str {
        match self {
            &DiceParseError::InvalidExpression => "Invalid dice expression",
        }
    }
}

impl ::std::fmt::Display for DiceParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(::std::error::Error::description(self))
    }
}

/// Describes a set of dice of the same type that can be "rolled" all at once, i.e. "2d6"
#[derive(Eq, PartialEq)]
pub struct Dice {
    count: u32,
    range: u32,
}

impl Dice {
    fn new(count: u32, range: u32) -> Dice {
        Dice {
            count: count,
            range: range,
        }
    }
}

impl ::std::str::FromStr for Dice {
    type Err = DiceParseError;

    fn from_str(s: &str) -> Result<Dice, Self::Err> {
        if !DICE_CMD_PATTERN.is_match(s) {
            return Err(DiceParseError::InvalidExpression)
        }

        let split: Vec<_> = s.split('d').filter_map(|n| n.parse().ok()).collect();
        match &split[..] {
            [ref count, ref range] => Ok(Dice::new(*count, *range)),
            [ref range] => Ok(Dice::new(1, *range)),
            _ => Err(DiceParseError::InvalidExpression),
        }
    }
}
