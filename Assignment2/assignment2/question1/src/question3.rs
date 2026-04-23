use rand::prelude::*;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
pub fn test_random_positive_square_root() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    assert_eq!(calculator::get_square_root(x), x.sqrt());
}

#[test]
#[should_panic]
pub fn test_random_negative_square_root() {
    let mut rng = rand::thread_rng();
    let x: f64 = -1.0*rng.gen::<f64>();
    calculator::get_square_root(x);
}

#[test]
pub fn test_square_root_of_zero() {
    assert_eq!(calculator::get_square_root(0.0), 0.0);
}

#[test]
pub fn test_square_root_of_one() {
    assert_eq!(calculator::get_square_root(1.0), 1.0);
}