use dice::Dice;

pub trait DiceResult {
    type RollValue;

    fn values(&self) -> &[Self::RollValue];
    fn total(&self) -> u32;
}

pub struct VecResult(Vec<u32>);

impl VecResult {
    pub fn new(values: Vec<u32>) -> VecResult {
        VecResult(values)
    }

    pub fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, u32> {
        self.0.iter()
    }
}

impl DiceResult for VecResult {
    type RollValue = u32;

    fn values(&self) -> &[Self::RollValue] {
        &self.0
    }

    fn total(&self) -> u32 {
        self.0.iter().sum()
    }
}

impl ::std::fmt::Display for VecResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let as_strings: Vec<_> = self.iter().map(|n| n.to_string()).collect();
        write!(f, "{} ({})", as_strings.join(", "), self.total())
    }
}

pub trait DiceResultGenerator {
    type DiceResult;

    fn gen_result(&mut self, dice: &Dice) -> Self::DiceResult;
}

impl<T: ::rand::Rng> DiceResultGenerator for T {
    type DiceResult = VecResult;

    fn gen_result(&mut self, dice: &Dice) -> Self::DiceResult {
        VecResult((0..dice.count).map(|_| self.gen_range(0, dice.range) + 1).collect())
    }
}

pub struct GenFn<F: FnMut(u32) -> u32>(pub F);

impl<F: FnMut(u32) -> u32> DiceResultGenerator for GenFn<F> {
    type DiceResult = VecResult;

    fn gen_result(&mut self, dice: &Dice) -> Self::DiceResult {
        VecResult((0..dice.count).map(|_| self.0(dice.range)).collect())
    }
}
