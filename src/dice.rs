use std::str;

use error::DiceParseError;
use generator::Generator;
use regex::Regex;

lazy_static! {
    static ref DICE_CMD_PATTERN: Regex = Regex::new(r"^\d+(d\d+)?$").unwrap();
}

/// `Dice` describes a set of dice of the same type that can be "rolled" all at once, i.e. "2d6"
#[derive(Debug, Eq, PartialEq)]
pub struct Dice {
    pub count: u32,
    pub range: u32,
}

impl Dice {
    /// Creates a new `Dice` struct with the provided dice count and range values
    pub fn new(count: u32, range: u32) -> Dice {
        Dice {
            count: count,
            range: range,
        }
    }

    /// Generates the result of a dice roll for a given `Dice` value
    pub fn gen_result<T: Generator>(&self, generator: &mut T) -> <T as Generator>::Output {
        generator.generate(&self)
    }
}

impl str::FromStr for Dice {
    type Err = DiceParseError;

    fn from_str(s: &str) -> Result<Dice, Self::Err> {
        if !DICE_CMD_PATTERN.is_match(s) {
            return Err(DiceParseError::InvalidExpression)
        }

        let mut parts = s.split('d').filter_map(|n| n.parse().ok());
        match (parts.next(), parts.next()) {
            (Some(count), Some(range)) => Ok(Dice::new(count, range)),
            (Some(range), None) => Ok(Dice::new(1, range)),
            _ => Err(DiceParseError::InvalidExpression),
        }
    }
}
