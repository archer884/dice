use ::Dice;

pub struct DiceResult(Vec<u32>);

impl DiceResult {
    pub fn new(values: Vec<u32>) -> DiceResult {
        DiceResult(values)
    }

    pub fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, u32> {
        self.0.iter()
    }

    pub fn values(&self) -> &[u32] {
        &self.0
    }

    pub fn total(&self) -> u32 {
        self.0.iter().sum()
    }
}

impl ::std::fmt::Display for DiceResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let as_strings: Vec<_> = self.iter().map(|n| n.to_string()).collect();
        write!(f, "{} ({})", as_strings.connect(", "), self.total())
    }
}

pub trait DiceResultGenerator {
    fn gen_result(&mut self, dice: &Dice) -> DiceResult;
}

impl<T: ::rand::Rng> DiceResultGenerator for T {
    fn gen_result(&mut self, dice: &Dice) -> DiceResult {
        DiceResult((0..dice.count).map(|_| self.gen_range(0, dice.range) + 1).collect())
    }
}
