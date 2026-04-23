use super::*;

use rand::prelude::*;
use hamcrest::prelude::*;
use std::panic;

// is(close_to()) used to allow room for error

#[test]
pub fn test_random_positive_square_root() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    hamcrest::assert_that!(calculator::get_square_root(x), is(close_to(x.sqrt(), 0.00001)));
}

#[test]
pub fn test_random_negative_square_root() {
    let mut rng = rand::thread_rng();
    let x: f64 = -1.0*rng.gen::<f64>();

    // capture the panic
    let result = panic::catch_unwind(|| {
        calculator::get_square_root(x);
    });

    // use hamcrest to assert that it is a panic
    hamcrest::assert_that!(result.is_err(), is(true));
}

#[test]
pub fn test_square_root_of_zero() {
    hamcrest::assert_that!(calculator::get_square_root(0.0), is(close_to(0.0, 0.00001)));
}

#[test]
pub fn test_square_root_of_one() {
    hamcrest::assert_that!(calculator::get_square_root(1.0), is(close_to(1.0, 0.00001)));
}