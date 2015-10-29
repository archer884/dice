use std::fmt;
use std::slice;

pub trait DiceResult<'a> {
    type Value: Clone;
    type Iterator: Iterator<Item = Self::Value>;
    fn values(&'a self) -> Self::Iterator;
    fn total(&self) -> Self::Value;
}

pub struct VecResult(Vec<u32>);

impl VecResult {
    pub fn new(values: Vec<u32>) -> VecResult {
        VecResult(values)
    }
}

pub struct VecResultIterator<'a>(slice::Iter<'a, u32>);

impl<'a> Iterator for VecResultIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|n| *n)
    }
}

impl<'a> DiceResult<'a> for VecResult {
    type Value = u32;
    type Iterator = VecResultIterator<'a>;

    fn values(&'a self) -> Self::Iterator {
        VecResultIterator(self.0.iter())
    }

    fn total(&self) -> u32 {
        self.0.iter().sum()
    }
}

impl fmt::Display for VecResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let results: Vec<_> = self.values().map(|n| n.to_string()).collect();

        write!(f, "{} ({})", results.join(", "), self.total())
    }
}
