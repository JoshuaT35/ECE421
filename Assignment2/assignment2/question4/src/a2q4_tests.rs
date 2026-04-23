use super::*;

use hamcrest::prelude::*;
use factorial::Factorial;
use std::panic;

// test: ensure a is not smaller than b
#[test]
fn test_a_not_smaller_than_b() {
    // a is larger than b
    let mut a: f64 = 4.0;
    let mut b: f64 = 2.0;
    let mut expected_result: f64 = ((a as u64).factorial() / (((a-b) as u64).factorial() * (b as u64).factorial())) as f64;
    let mut actual_result: f64 = a2q4::combination_func(a, b) as f64;

    hamcrest::assert_that!(actual_result, is(close_to(expected_result, 0.00001)));

    // a is equal to b
    a = 2.0;
    b = 2.0;
    expected_result = ((a as u64).factorial() / (((a-b) as u64).factorial() * (b as u64).factorial())) as f64;
    actual_result = a2q4::combination_func(a, b) as f64;
    
    hamcrest::assert_that!(actual_result, is(close_to(expected_result, 0.00001)));

    // a is smaller than b
    a = 1.0;
    b = 2.0;

    // catch result (panic expected)
    let result = panic::catch_unwind(|| {
        a2q4::combination_func(a, b);
    });

    // use hamcrest to assert that panic was obtained
    hamcrest::assert_that!(result.is_err(), is(true));
}

// test: inputs must be positive
#[test]
#[should_panic]
fn test_positive_inputs() {
    // a is negative
    let mut a: f64 = -4.0;
    let mut b: f64 = 2.0;
    a2q4::combination_func(a, b);

    // b is negative
    let mut a: f64 = 4.0;
    let mut b: f64 = -2.0;
    a2q4::combination_func(a, b);

    // both are negative
    let mut a: f64 = -4.0;
    let mut b: f64 = -2.0;
    a2q4::combination_func(a, b);
}