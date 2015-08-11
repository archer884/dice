use dice::Dice;
use result::VecResult;

pub trait DiceResultGenerator {
    type DiceResult;

    fn gen_result(&mut self, dice: &Dice) -> Self::DiceResult;
}

impl<T: ::rand::Rng> DiceResultGenerator for T {
    type DiceResult = VecResult;

    fn gen_result(&mut self, dice: &Dice) -> Self::DiceResult {
        VecResult::new((0..dice.count).map(|_| self.gen_range(0, dice.range) + 1).collect())
    }
}

pub struct GenFn<F: FnMut(u32) -> u32>(pub F);

impl<F: FnMut(u32) -> u32> DiceResultGenerator for GenFn<F> {
    type DiceResult = VecResult;

    fn gen_result(&mut self, dice: &Dice) -> Self::DiceResult {
        VecResult::new((0..dice.count).map(|_| self.0(dice.range)).collect())
    }
}
