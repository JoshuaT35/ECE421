use std::cmp;

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

pub fn compute_chebyshev_distance(p1: &Point, p2: &Point) -> i32 {
    let num1: i32 = (p2.x as i32 - p1.x as i32).abs();
    let num2: i32 = (p2.y as i32 - p1.y as i32).abs();
    cmp::max(num1, num2)
}

fn main() {
    // --- test chebyshev distance function ---
    // parameters
    let p1: Point = Point::new(3, 4);
    let p2: Point = Point::new(8, 10);

    // result
    let result: i32 = compute_chebyshev_distance(&p1, &p2);
    println!(
        "chebyshev distance between ({}, {}) and ({}, {}) is {}",
        p1.x, p1.y, p2.x, p2.y, result
    );
}
