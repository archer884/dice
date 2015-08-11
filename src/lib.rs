#![feature(iter_arith, slice_patterns)]

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

mod dice;
mod error;
mod generator;
mod result;

pub use dice::Dice;
pub use error::DiceParseError;
pub use generator::{ DiceResultGenerator, GenFn };
pub use result::{ DiceResult, VecResult };

#[cfg(test)]
mod tests {
    use super::{
        Dice,
        DiceResult,
        DiceResultGenerator,
        GenFn,
        VecResult
    };

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

    #[test]
    /// GenFn works correctly with dice commands
    fn genfn_works() {
        let cmd = Dice::new(10, 10);
        let mut seed = 1;
        let mut gen = GenFn(|_| {
            let ret = seed;
            seed += 1;
            ret
        });

        assert!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10] == cmd.gen_result(&mut gen).values());
    }

    #[test]
    /// dice result generators work correctly with dice commands
    fn drg_works() {
        let cmd = Dice::new(10, 10);
        assert!([1, 2, 1, 2, 1, 2, 1, 2, 1, 2] == cmd.gen_result(&mut TestDrg(1, 2)).values())
    }

    struct TestDrg(u32, u32);

    impl DiceResultGenerator for TestDrg {
        type DiceResult = VecResult;

        fn gen_result(&mut self, dice: &::dice::Dice) -> Self::DiceResult {
            VecResult::new((0..dice.count).map(|_| if self.0 == self.1 {
                self.0 = 1;
                self.1
            } else {
                self.0 += 1;
                self.0 - 1
            }).collect())
        }
    }
}
