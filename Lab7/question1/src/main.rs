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

pub fn compute_euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    let x_square: f64 = f64::from((p1.x - p2.x).pow(2));
    let y_square: f64 = f64::from((p1.y - p2.y).pow(2));
    return (x_square + y_square).sqrt();
}

fn main() {
    // --- test euclidean distance function ---
    // parameters
    let p1: Point = Point::new(1, 1);
    let p2: Point = Point::new(2, 2);

    // result
    let result: f64 = compute_euclidean_distance(&p1, &p2);
    println!("euclidean distance between (1, 1) and (2, 2) is {}", result);
}

// --- tests ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean() {
        // parameters
        let p1: Point = Point::new(1, 1);
        let p2: Point = Point::new(2, 2);

        // expected answer
        let expected: f64 = ((1.0 as f64 - 2.0).powf(2.0) + (1.0 as f64 - 2.0).powf(2.0)).sqrt();
        // let expected: f64 = (2 as f64).sqrt();

        // actual
        let actual: f64 = compute_euclidean_distance(&p1, &p2);

        assert_eq!(expected, actual);
    }
}
