use rand::prelude::*;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
pub fn basic_multiply() {
    assert_eq!(calculator::multiply(1.0, 2.0), 2.0);
}

#[test]
pub fn multiply_negative_number() {
    assert_eq!(calculator::multiply(-1.0, 2.0), -2.0);
}

#[test]
pub fn multiply_random_numbers() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    assert_eq!(calculator::multiply(x, y), x*y);
}

#[test]
pub fn basic_divide() {
    assert_eq!(calculator::divide(4.0, 2.0).unwrap(), 2.0);
}

#[test]
pub fn divide_negative_number() {
    assert_eq!(calculator::divide(4.0, -2.0).unwrap(), -2.0);
}

#[test]
pub fn divide_random_numbers() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    match calculator::divide(x, y) {
        Ok(result) => assert_eq!(result, x/y),
        Err(error) => assert_eq!(error, "Cannot divide by 0"),
    }
}
