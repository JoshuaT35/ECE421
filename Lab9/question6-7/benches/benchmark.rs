use rand::Rng;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn selection_sort(list: &mut Vec<i64>) {
    // for each list element
    for i in 0..list.len() {
        if let Some((smallest_item_idx, _)) = list[i..]
            .iter()
            .enumerate()
            .min_by_key(|&(_, &val)| val)
        {
            let smallest_item_idx = i + smallest_item_idx;
            list.swap(i, smallest_item_idx);
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
