use dice::Dice;
use rand::Rng;

pub trait Generator {
    type Result;
    fn generate(&mut self, dice: &Dice) -> Self::Result;
}

impl<T: Rng> Generator for T {
    type Result = Vec<u32>;

    fn generate(&mut self, dice: &Dice) -> Self::Result {
        (0..dice.count).map(|_| self.gen_range(0, dice.range) + 1).collect()
    }
}
