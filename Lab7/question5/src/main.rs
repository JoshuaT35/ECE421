use std::{
    io,
    io::{Write},
    cmp,
};

pub struct Point {
    x: i8,
    y: i8,
}

impl Point {
    pub fn new(x: i8, y: i8) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
}

// uses functions in C libc library
unsafe extern "C" {
    pub fn abs(i: i32) -> i32;
    pub fn pow(base: f64, exp: f64) -> f64;
    pub fn sqrt(x: f64) -> f64;
}

pub fn compute_euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    unsafe {
        let x_square: f64 = pow(f64::from(p1.x - p2.x), 2.0);
        let y_square: f64 = pow(f64::from(p1.y - p2.y), 2.0);
        return sqrt(x_square + y_square);
    }
}

pub fn compute_manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    unsafe {
        let a_abs: i32 = abs(p2.x as i32 - p1.x as i32);
        let b_abs: i32 = abs(p2.y as i32 - p1.y as i32);
        a_abs + b_abs
    }
}

pub fn compute_chebyshev_distance_C(p1: &Point, p2: &Point) -> i32 {
    unsafe {
        let num1: i32 = abs(p2.x as i32 - p1.x as i32);
        let num2: i32 = abs(p2.y as i32 - p1.y as i32);
        cmp::max(num1, num2)
    }
}

// get num input from io
pub fn get_num_input_io(buffer: &mut String) -> i8 {
    loop {
        buffer.clear();
        io::stdout().flush().unwrap();
        io::stdin().read_line(buffer).unwrap();
        match buffer.trim().parse::<i8>() {
            Ok(num) => {
                return num;
            },
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
            },
        }
    }
}

fn main() {
    let mut line = String::new();

    // Get first point (x1, y1)
    print!("Enter point 1 x-value: ");
    let x1: i8 = get_num_input_io(&mut line);

    print!("Enter point 1 y-value: ");
    let y1: i8 = get_num_input_io(&mut line);

    // Get second point (x2, y2)
    print!("Enter point 2 x-value: ");
    let x2: i8 = get_num_input_io(&mut line);

    print!("Enter point 2 y-value: ");
    let y2: i8 = get_num_input_io(&mut line);

    // create points
    let p1: Point = Point::new(x1, y1);
    let p2: Point = Point::new(x2, y2);

    // ask them which distance to calculate
    println!("Please select a distance to calculate:");
    println!("1. Euclidean");
    println!("2. Manhattan");
    println!("3. Chebyshev");

    // get response
    loop {
        let response: i8 = get_num_input_io(&mut line);
        match response {
            1 => {
                let result: f64 = compute_euclidean_distance(&p1, &p2);
                println!(
                    "euclidean distance between ({}, {}) and ({}, {}) is {}",
                    p1.x, p1.y, p2.x, p2.y, result
                );
                break;
            },
            2 => {
                let result: i32 = compute_manhattan_distance(&p1, &p2);
                println!(
                    "manhattan distance between ({}, {}) and ({}, {}) is {}",
                    p1.x, p1.y, p2.x, p2.y, result
                );
                break;
            },
            3 => {
                let result: i32 = compute_chebyshev_distance_C(&p1, &p2);
                println!(
                    "chebyshev distance between ({}, {}) and ({}, {}) is {}",
                    p1.x, p1.y, p2.x, p2.y, result
                );
                break;
            },
            _ => {
                println!("Invalid response.");
            }
        }
    }
}
