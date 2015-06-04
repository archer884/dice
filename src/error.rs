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
