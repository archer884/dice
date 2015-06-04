#[cfg(test)]
use super::*;

/// `parse()` handles input of the form `2d6`
#[test]
fn parse_handles_long_form() {
    let cmd = Dice::new(2, 6);
    assert!(cmd == "2d6".parse().unwrap());
}

#[test]
fn parse_handles_short_form() {
    let cmd = Dice::new(1, 6);
    assert!(cmd == "6".parse().unwrap());
}
