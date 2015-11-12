use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DiceParseError {
    InvalidExpression,
}

impl Error for DiceParseError {
    fn description(&self) -> &str {
        match self {
            &DiceParseError::InvalidExpression => "Invalid dice expression",
        }
    }
}

impl fmt::Display for DiceParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}
