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

// uses the abs() function in C libc library
unsafe extern "C" {
    pub fn abs(i: i32) -> i32;
}

pub fn compute_manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    unsafe {
        let a_abs: i32 = abs(p2.x as i32 - p1.x as i32);
        let b_abs: i32 = abs(p2.y as i32 - p1.y as i32);
        (a_abs + b_abs)
    }
}

fn main() {
    // --- test manhattan distance function ---
    // parameters
    let p1: Point = Point::new(1, 1);
    let p2: Point = Point::new(2, 2);

    // result
    let result: i32 = compute_manhattan_distance(&p1, &p2);
    println!("manhattan distance between (1, 1) and (2, 2) is {}", result);
}
