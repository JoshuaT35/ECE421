use rayon::join;
use std::cmp::PartialOrd;

fn concurrent_quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);

    let (left, right) = v.split_at_mut(mid);

    rayon::join(
        || concurrent_quick_sort(left),
        || concurrent_quick_sort(&mut right[1..]),
    );
}

fn partition<T: PartialOrd>(v: &mut [T]) -> usize {
    let last_index: usize = v.len() - 1;
    let mut i: isize = -1;

    for j in 0..last_index {
        if v[j] <= v[last_index] {
            i += 1;
            v.swap(i as usize, j);
        }
    }

    v.swap((i + 1) as usize, last_index);

    return (i + 1) as usize;
}


fn main() {
    let mut example_vec: Vec<i32> = vec![2, 7, 1, 6];

    concurrent_quick_sort(&mut example_vec);

    for val in example_vec.iter() {
        print!("{} ", val);
    }
    println!();
}
