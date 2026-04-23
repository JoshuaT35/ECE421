use super::*;

use hamcrest::prelude::*;
use std::panic;

// test for negative income
#[test]
#[should_panic]
pub fn test_negative_income() {
    let income: f64 = -1000.0;
    a2q5::tax(income);
}

// test for non-integer income
#[test]
#[should_panic]
pub fn test_non_integer_income() {
    let income: f64 = -1000.50;
    a2q5::tax(income);
}
