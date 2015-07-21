use error::DiceParseError;
use result::{ DiceResult, DiceResultGenerator };
use regex::Regex;

lazy_static! {
    static ref DICE_CMD_PATTERN: Regex = Regex::new(r"^\d+(d\d+)?$").unwrap();
}

/// Dice describes a set of dice of the same type that can be "rolled" all at once, i.e. "2d6"
#[derive(Debug, Eq, PartialEq)]
pub struct Dice {
    pub count: u32,
    pub range: u32,
}

impl Dice {
    pub fn new(count: u32, range: u32) -> Dice {
        Dice {
            count: count,
            range: range,
        }
    }

    pub fn gen_result<G: DiceResultGenerator>(&self, drg: &mut G) -> DiceResult {
        drg.gen_result(&self)
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
