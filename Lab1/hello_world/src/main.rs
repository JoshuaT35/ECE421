// Replace the code here with any code in the other .rs files.

use is_prime::*;

// function to get all prime numbers below limit
fn get_primes(limit: usize) -> Vec<usize> {
    let mut primes = Vec::new();

    for i in 2..limit {
        if is_prime(&i.to_string()) {
            primes.push(i);
        }
    }
    return primes;
}

// get longest prime sequence
fn get_longest_prime_sequence(limit: usize) -> (usize, usize, Vec<usize>) {
    // get all primes
    let primes_vec = get_primes(limit);

    // variables to return
    let mut max_length = 0;
    let mut max_sum = 0;
    let mut best_sequence = Vec::new();

    // treaing each prime as a staring number
    for i in 0..primes_vec.len() {
        // set current variables
        let mut current_sum = 0;
        let mut current_sequence = Vec::new();

        // for each prime from index i, onwards
        for j in i..primes_vec.len() {
            // add prime to the current counting sum
            current_sum += primes_vec[j];
            current_sequence.push(primes_vec[j]);

            // if we go overboard, then break
            if current_sum >= limit {
                break;
            }

            // from here, it is now valid
            // if our current sequence is longer than the last recorded length, save the current sequence
            if current_sequence.len() > max_length && is_prime(&current_sum.to_string()) {
                max_length = current_sequence.len();
                max_sum = current_sum;
                best_sequence = current_sequence.clone();
            }
        }
    }
    return (max_length, max_sum, best_sequence);
}

fn main() {
    let limit = 1000;
    let (x, y, sequence) = get_longest_prime_sequence(limit);

    println!("Longest length of consecutive primes: {}", x);
    println!("Sum of these primes: {}", y);
    println!("List of consecutive primes: {:?}", sequence);
}
