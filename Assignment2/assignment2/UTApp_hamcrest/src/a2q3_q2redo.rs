use super::*;

use rand::prelude::*;
use hamcrest::prelude::*;

// is(close_to()) used to allow room for error

#[test]
pub fn basic_multiply() {
    hamcrest::assert_that!(calculator::multiply(1.0, 2.0), is(close_to(2.0, 0.00001)));
}

#[test]
pub fn multiply_negative_number() {
    hamcrest::assert_that!(calculator::multiply(-1.0, 2.0), is(close_to(-2.0, 0.00001)));
}

#[test]
pub fn multiply_random_numbers() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    hamcrest::assert_that!(calculator::multiply(x, y), is(close_to(x*y, 0.00001)));
}

#[test]
pub fn basic_divide() {
    hamcrest::assert_that!(calculator::divide(4.0, 2.0).unwrap(), is(close_to(2.0, 0.00001)));
}

#[test]
pub fn divide_negative_number() {
    hamcrest::assert_that!(calculator::divide(4.0, -2.0).unwrap(), is(close_to(-2.0, 0.00001)));
}

#[test]
pub fn divide_random_numbers() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    match calculator::divide(x, y) {
        Ok(result) => hamcrest::assert_that!(result, is(close_to(x/y, 0.00001))),
        Err(error) => hamcrest::assert_that!(&error, is(equal_to("Cannot divide by 0"))),
    }
}
