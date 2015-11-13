use dice::Dice;
use rand::Rng;

pub trait Generator {
    type Output: IntoIterator;
    fn generate(&mut self, dice: &Dice) -> Self::Output;
}

impl<T: Rng> Generator for T {
    type Output = Vec<u32>;

    fn generate(&mut self, dice: &Dice) -> Self::Output {
        (0..dice.count).map(|_| self.gen_range(0, dice.range) + 1).collect()
    }
}
