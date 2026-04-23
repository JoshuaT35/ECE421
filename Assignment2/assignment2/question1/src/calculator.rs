pub fn add(num1: f64, num2: f64) -> f64 {
    return num1 + num2;
}

pub fn subtract(num1: f64, num2: f64) -> f64 {
    return num1 - num2;
}

pub fn multiply(num1: f64, num2: f64) -> f64 {
    return num1 * num2;
}

// example: uses Ok and Err
pub fn divide(num1: f64, num2: f64) -> Result<f64, String> {
    if num2 == 0.0 {
        return Err("Cannot divide by 0".to_string());
    }
    return Ok(num1 / num2);
}

// example: uses panic
pub fn get_square_root(num1: f64) -> f64 {
    if num1 < 0.0 {
        panic!("Cannot get square root of negative number");
    }
    return num1.sqrt();
}

// example: uses panic
pub fn get_roots(num1: f64, num2: f64, num3: f64) -> Vec<f64> {
    // check if discrminant is negative (no solution)
    let discriminant = num2.powf(2.0) - (4.0*num1*num3);
    if discriminant < 0.0 {
        panic!("Discriminant is negative");
    }

    // get the roots
    let mut roots = Vec::new();
    let root1 = (-num2 + discriminant.sqrt())/(2.0*num1);
    let root2 = (-num2 - discriminant.sqrt())/(2.0*num1);
    
    // push both roots only if they are different. else push just 1
    roots.push(root1);
    if root1 != root2 {
        roots.push(root2);
        // sort the vector in increasing order for easier testing
        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    return roots;
}
