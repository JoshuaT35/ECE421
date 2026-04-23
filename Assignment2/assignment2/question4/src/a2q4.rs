use std::io;
use std::panic;
use factorial::Factorial;

pub fn combination_func(a: f64, b: f64) -> u64 {
    // if either is a negative number, panic
    if a < 0.0 || b < 0.0 {
        panic!("combination_func: negative input");
    }

    // if a is smaller than b, panic
    if a < b {
        panic!("combination_func: a is smaller than b");
    }

    let numerator: u64 = (a as u64).factorial();
    let denominator: u64 = ((a-b) as u64).factorial() * (b as u64).factorial();
    return numerator/denominator;
}

pub fn get_combination_func() -> u64 {
    let mut input1: String = String::new();
    let mut input2: String = String::new();
    
    // get input
    println!("Input values for a: ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    // get input
    println!("Input values for b: ");
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    // convert the input to numbers
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");

    let combination_func_result: u64 = combination_func(num1, num2);

    return combination_func_result;
}
