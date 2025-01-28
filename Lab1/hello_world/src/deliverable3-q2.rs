use rand::prelude::*;
use apint::prelude::*;
use is_prime::*;

fn function(n: u32) -> ApInt {
    let mut rng = rand::thread_rng();

    loop {
        // get a number
        let mut candidate: ApInt = ApInt::from(rng.gen_range(0..n));
        // set its last bit to 1
        let _ = candidate.set_bit_at(0).unwrap();
        // convert to string
        let string_var: String = candidate.resize_to_u32().to_string();
        // check if it is prime
        if is_prime(&string_var) == true {
            return candidate;
        }
    }
}

fn main() {
    let number: ApInt = function(10);

    println!("Prime is {}", number.resize_to_u32().to_string());
}
