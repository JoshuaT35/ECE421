# ECE421 Lab 7

## How to run main/benchmarks

### To build
- Build all the projects by running `cargo build`.
- Or build a specific project `<project-name>` by running `cargo build --package <project-name>`.

### To run `main()`
- For question 1, run `cargo run --package question1`.
- For question 2, run `cargo run --package question2`.
- For question 3, run `cargo run --package question3`.
- For question 4, run `cargo run --package question4`.
- For question 5, run `cargo run --package question5`.
- For questions 7, 8, 9, run `cargo run --package question7-8-9`.
- For question 10, run `cargo run --package question10`.
- For question 11, run `cargo run --package question11`.

### To run bencharks
- For question 7, run `cargo bench --package question7-8-9`.


## Question 6 Answer
Output is:
`The average age of people older than 30 is 36.5`

## Question 8 answer
This is a result from running the benchmarks from question 7 (actual results may vary). There is a *sequential* and *parallel* benchmark.

**For vector size 10 000**
*sequential_avg_over_30_ppl_10_000*\
*time: [5.5269 µs 5.6847 µs 5.9066 µs]*\
*Found 6 outliers among 100 measurements (6.00%)*\
*2 (2.00%) high mild*\
*4 (4.00%) high severe*

*parallel_avg_over_30_ppl_10_000*\
*time: [236.38 µs 245.39 µs 256.22 µs]*\
*Found 7 outliers among 100 measurements (7.00%)*\
*2 (2.00%) high mild*\
*5 (5.00%) high severe*

In this case, *par_iter* seems to have increased the average time taken to compute the program.

## Question 9 answer
This is a result from running the benchmarks from question 7 (actual results may vary). Each vector size has a *sequential* and *parallel* benchmark.

**For vector size 1 000**
sequential_avg_over_30_ppl_1_000
time: [632.12 ns 665.07 ns 700.52 ns]
Found 5 outliers among 100 measurements (5.00%)
2 (2.00%) high mild
3 (3.00%) high severe

parallel_avg_over_30_ppl_1_000
time: [172.74 µs 177.05 µs 182.04 µs]
Found 2 outliers among 100 measurements (2.00%)
1 (1.00%) high mild
1 (1.00%) high severe

**For vector size 10 000**
*sequential_avg_over_30_ppl_10_000*\
*time: [5.5269 µs 5.6847 µs 5.9066 µs]*\
*Found 6 outliers among 100 measurements (6.00%)*\
*2 (2.00%) high mild*\
*4 (4.00%) high severe*

*parallel_avg_over_30_ppl_10_000*\
*time: [236.38 µs 245.39 µs 256.22 µs]*\
*Found 7 outliers among 100 measurements (7.00%)*\
*2 (2.00%) high mild*\
*5 (5.00%) high severe*

**For vector size 100 000**
*sequential_avg_over_30_ppl_100_000*\
*time: [59.181 µs 61.956 µs 64.801 µs]*\
*Found 2 outliers among 100 measurements (2.00%)*\
*2 (2.00%) high severe*

*parallel_avg_over_30_ppl_100_000*\
*time: [299.88 µs 309.41 µs 321.66 µs]*\
*Found 4 outliers among 100 measurements (4.00%)*\
*3 (3.00%) high mild*\
*1 (1.00%) high severe*

**For vector size 1 000 000**
*sequential_avg_over_30_ppl_1_000_000*\
*time: [563.32 µs 576.64 µs 594.38 µs]*\
*Found 12 outliers among 100 measurements (12.00%)*\
*6 (6.00%) high mild*\
*6 (6.00%) high severe*

*parallel_avg_over_30_ppl_1_000_000*\
*time: [603.94 µs 622.06 µs 642.92 µs]*\
*Found 15 outliers among 100 measurements (15.00%)*\
*10 (10.00%) high mild*\
*5 (5.00%) high severe*

At all vector sizes, the sequential average is faster than the parallel average.

## Question 11 answer
*into_par_iter* from the `Rayon` library. Furthermore, we also had to wrap the vector which stored pixel data in `Arc<Mutex<>>` since there were race conditions in updating the pixel data.
