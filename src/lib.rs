#![feature(core, plugin, slice_patterns)]
#![plugin(regex_macros)]
extern crate rand;
extern crate regex;
mod dice_tests;
use rand::Rng;
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

pub struct DiceResult(Vec<u32>);

impl DiceResult {
    pub fn new(values: Vec<u32>) -> DiceResult {
        DiceResult(values)
    }

    pub fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, u32> {
        self.0.iter()
    }

    pub fn total(&self) -> u32 {
        self.0.iter().sum()
    }
}

impl ::std::fmt::Display for DiceResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let as_strings: Vec<_> = self.iter().map(|n| n.to_string()).collect();
        write!(f, "{} ({})", as_strings.connect(", "), self.total())
    }
}

/// Describes a set of dice of the same type that can be "rolled" all at once, i.e. "2d6"
#[derive(Debug, Eq, PartialEq)]
pub struct Dice {
    count: u32,
    range: u32,
}

impl Dice {
    pub fn new(count: u32, range: u32) -> Dice {
        Dice {
            count: count,
            range: range,
        }
    }

    pub fn gen_result(&self) -> DiceResult {
        let mut prng = ::rand::weak_rng();

        DiceResult((0..self.count).map(|_| prng.gen_range(0, self.range) + 1).collect())
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
