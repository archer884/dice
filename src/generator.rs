use dice::Dice;
use rand::Rng;

pub trait Generator<'a> {
    type Output: IntoIterator;
    fn generate(&'a mut self, dice: &'a Dice) -> Self::Output;
}

impl<'a, T: Rng> Generator<'a> for T {
    type Output = Vec<u32>;

    fn generate(&'a mut self, dice: &'a Dice) -> Self::Output {
        (0..dice.count).map(move |_| self.gen_range(0, dice.range) + 1).collect()
    }
}
