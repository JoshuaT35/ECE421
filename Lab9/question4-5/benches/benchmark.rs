use rand::Rng;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn selection_sort(list: &mut Vec<i64>) {
    // for each list element
    for i in 0..list.len() {
        // get current item pointed to by i
        let mut smallest_item_idx = i;
        let mut current_min = list[i];

        // go through all other elements
        for j in (i+1)..list.len() {
            // get index of smallest item in other elements
            if list[j] < current_min {
                current_min = list[j];
                smallest_item_idx = j;
            }
        }

        // swap item pointed at i, with smallest item in other elements
        if i != smallest_item_idx {
            let temp = list[i];
            list[i] = list[smallest_item_idx];
            list[smallest_item_idx] = temp;
        }
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    // create random vector
    let mut rng = rand::rng();
    let mut l: Vec<i64> = (0..10000).map(|_| {rng.random_range(1..=10000)}).collect();

    c.bench_function("selection_sort", |b| b.iter(|| selection_sort(black_box(&mut l))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
