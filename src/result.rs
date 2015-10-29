use std::fmt;
use std::slice;

pub trait DiceResult {
    type RollValue;
    fn values(&self) -> slice::Iter<Self::RollValue>;
    fn total(&self) -> Self::RollValue;
}

pub struct VecResult(Vec<u32>);

impl VecResult {
    pub fn new(values: Vec<u32>) -> VecResult {
        VecResult(values)
    }
}

impl DiceResult for VecResult {
    type RollValue = u32;

    fn values(&self) -> slice::Iter<Self::RollValue> {
        self.0.iter()
    }

    fn total(&self) -> u32 {
        self.0.iter().sum()
    }
}

impl fmt::Display for VecResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_strings: Vec<_> = self.values().map(|n| n.to_string()).collect();

        write!(f, "{} ({})", as_strings.join(", "), self.total())
    }
}
