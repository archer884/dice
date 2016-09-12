#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

mod dice;
mod error;
mod generator;

pub use dice::Dice;
pub use error::DiceParseError;
pub use generator::Generator;

#[cfg(test)]
mod tests {
    use super::Dice;

    #[test]
    /// `parse()` handles input of the form `2d6`
    fn parse_handles_long_form() {
        let cmd = Dice::new(2, 6);
        assert!(cmd == "2d6".parse().unwrap());
    }

    #[test]
    /// `parse()` handles input of the form `6`
    fn parse_handles_short_form() {
        let cmd = Dice::new(1, 6);
        assert!(cmd == "6".parse().unwrap());
    }
}
