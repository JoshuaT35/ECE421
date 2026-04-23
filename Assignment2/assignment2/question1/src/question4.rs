use rand::prelude::*;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
pub fn test_basic_roots() {
   let a: f64 = 1.0;
   let b: f64 = 0.0;
   let c: f64 = -1.0;
   let expected: Vec<f64> = vec![-1.0, 1.0];
   assert_eq!(calculator::get_roots(a, b, c), expected);
}

#[test]
pub fn test_single_root() {
    let a: f64 = 1.0;
    let b: f64 = 0.0;
    let c: f64 = 0.0;
    let discriminant: f64 = b.powf(2.0) - (4.0*a*c);
    let expected: Vec<f64> = vec![0.0];
    
    // assert answers are expected, and that discriminant is 0
    assert_eq!(discriminant, 0.0);
    assert_eq!(calculator::get_roots(a, b, c), expected);
}

#[test]
pub fn test_random_solvable_quadratic() {
    let mut rng = rand::thread_rng();

    // generate random numbers from range 0 to 20
    let mut a: f64 = rng.gen_range(0.0..20.0);
    let mut b: f64 = rng.gen_range(0.0..20.0);
    let mut c: f64 = rng.gen_range(0.0..20.0);

    // get the discriminant
    let mut discriminant: f64 = b.powf(2.0) - (4.0*a*c);

    // if discriminant is negative, regenerate values
    while discriminant < 0.0 {
        // regenerate random numbers from range 0 to 20
        a = rng.gen_range(0.0..20.0);
        b = rng.gen_range(0.0..20.0);
        c = rng.gen_range(0.0..20.0);

        // recalculate discriminant
        discriminant = b.powf(2.0) - (4.0*a*c);
    }

    // get the expected roots
    let mut expected_roots = Vec::new();
    let root1 = (-b + discriminant.sqrt())/(2.0*a);
    let root2 = (-b - discriminant.sqrt())/(2.0*a);

    // push both roots only if they are different. else push just 1
    expected_roots.push(root1);
    if root1 != root2 {
        expected_roots.push(root2);
        // sort the vector in increasing order for easier testing
        expected_roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    assert_eq!(calculator::get_roots(a, b, c), expected_roots);
}

#[test]
#[should_panic]
pub fn test_random_non_solvable_quadratic() {
    let mut rng = rand::thread_rng();

    // generate random numbers from range 0 to 20
    let mut a: f64 = rng.gen_range(0.0..20.0);
    let mut b: f64 = rng.gen_range(0.0..20.0);
    let mut c: f64 = rng.gen_range(0.0..20.0);

    // get the discriminant
    let mut discriminant: f64 = b.powf(2.0) - (4.0*a*c);

    // if discriminant is not negative (possible solutions), regenerate values
    while discriminant >= 0.0 {
        // regenerate random numbers from range 0 to 20
        a = rng.gen_range(0.0..20.0);
        b = rng.gen_range(0.0..20.0);
        c = rng.gen_range(0.0..20.0);

        // recalculate discriminant
        discriminant = b.powf(2.0) - (4.0*a*c);
    }

    // call function. should panic
    calculator::get_roots(a, b, c);
}
