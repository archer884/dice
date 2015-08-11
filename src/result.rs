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
        let as_strings: Vec<_> = self.values().iter().map(|n| n.to_string()).collect();
        write!(f, "{} ({})", as_strings.join(", "), self.total())
    }
}
