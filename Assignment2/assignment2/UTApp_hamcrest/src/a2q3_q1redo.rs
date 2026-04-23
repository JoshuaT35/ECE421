use super::*;

use rand::prelude::*;
use hamcrest::prelude::*;

#[test]
pub fn basic_add() {
    hamcrest::assert_that!(calculator::add(1.0, 2.0), is(equal_to(3.0)));
}
#[test]
pub fn add_negative_number() {
	hamcrest::assert_that!(calculator::add(-1.0, 2.0), is(equal_to(1.0)));
}
#[test]
pub fn add_random_numbers() {
	let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let y: f64 = rng.gen();
    hamcrest::assert_that!(calculator::add(x, y), is(equal_to(x+y)));
}

#[test]
pub fn basic_subtract() {
	hamcrest::assert_that!(calculator::subtract(4.0, 2.0), is(equal_to(2.0)));
}

#[test]
pub fn subtract_negative_number() {
    hamcrest::assert_that!(calculator::subtract(-3.0, 2.0), is(equal_to(-5.0)));
}

#[test]
pub fn subtract_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let y: f64 = rng.gen();
    hamcrest::assert_that!(calculator::subtract(x, y), is(equal_to(x-y)));
}
