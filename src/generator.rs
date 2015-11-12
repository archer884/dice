use dice::Dice;
use rand::Rng;

pub trait Generator<'a> {
    type Result;
    type Iterator: Iterator<Item = Self::Result>;
    fn generate(&'a mut self, dice: &'a Dice) -> Self::Iterator;
}

impl<'a, T: Rng> Generator<'a> for T {
    type Result = u32;
    type Iterator = Box<Iterator<Item = u32> + 'a>;

    fn generate(&'a mut self, dice: &'a Dice) -> Self::Iterator {
        Box::new((0..dice.count).map(move |_| self.gen_range(0, dice.range) + 1))
    }
}
