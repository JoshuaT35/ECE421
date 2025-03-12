use rayon::prelude::*;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

struct Person {
    age: u32,
}

fn generate_people(num_people: u32) -> Vec<Person> {
    let mut v: Vec<Person> = Vec::new();
    for i in 1..num_people {
        v.push(Person { age: i });
    }
    return v;
}

fn sequential_avg_over_30(people: &[Person]) -> f32 {
    let num_over_30 = people.iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = people.iter().map(|x| x.age).filter(|&x| x > 30).sum();
    sum_over_30 as f32 / num_over_30
}

fn parallel_avg_over_30(people: &[Person]) -> f32 {
    let num_over_30 = people.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = people.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();
    sum_over_30 as f32 / num_over_30
}

fn criterion_benchmark(c: &mut Criterion) {
    // generate people
    let people1000: Vec<Person> = generate_people(1000);
    let people10000: Vec<Person> = generate_people(10000);
    let people100000: Vec<Person> = generate_people(100000);
    let people1000000: Vec<Person> = generate_people(1000000);
    
    // sequential and parallel for 1 000 people
    c.bench_function("sequential_avg_over_30_ppl_1_000", |b| {
        b.iter(|| sequential_avg_over_30(black_box(&people1000)))
    });
    c.bench_function("parallel_avg_over_30_ppl_1_000", |b| {
        b.iter(|| parallel_avg_over_30(black_box(&people1000)))
    });

    // sequential and parallel for 10 000 people
    c.bench_function("sequential_avg_over_30_ppl_10_000", |b| {
        b.iter(|| sequential_avg_over_30(black_box(&people10000)))
    });
    c.bench_function("parallel_avg_over_30_ppl_10_000", |b| {
        b.iter(|| parallel_avg_over_30(black_box(&people10000)))
    });

    // sequential and parallel for 100 000 people
    c.bench_function("sequential_avg_over_30_ppl_100_000", |b| {
        b.iter(|| sequential_avg_over_30(black_box(&people100000)))
    });
    c.bench_function("parallel_avg_over_30_ppl_100_000", |b| {
        b.iter(|| parallel_avg_over_30(black_box(&people100000)))
    });

    // sequential and parallel for 1 000 000 people
    c.bench_function("sequential_avg_over_30_ppl_1_000_000", |b| {
        b.iter(|| sequential_avg_over_30(black_box(&people1000000)))
    });
    c.bench_function("parallel_avg_over_30_ppl_1_000_000", |b| {
        b.iter(|| parallel_avg_over_30(black_box(&people1000000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);