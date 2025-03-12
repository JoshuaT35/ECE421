# ECE421 Lab 9

## How to run main/benchmarks/tests

### To build
- Build all the projects by running `cargo build`.
- Build a specific project `<project-name>` by running `cargo build --package <project-name>`.

### To run `main()`
- For question 1, run `cargo run --package question1`.
- For question 2, run `cargo run --package question2`.
- For question 3, run `cargo run --package question3`.

### To run bencharks
- For question 5, run `cargo bench --package question4-5`.
- For question 7, run `cargo bench --package question6-7`.

### To run tests (for Question 10)
- Test if a new user is successfully added: `cargo test --package DBProject --test test_user_not_exist`
- Test if a user already exists: `cargo test --package DBProject --test test_user_exists`
- Test if a new transaction is successfully added: `cargo test --package DBProject --test test_transaction_not_exist`
- Test if a transaction already exists: `cargo test --package DBProject --test test_transaction_exists`

NOTE: since tests may access a database in parallel, each test was split into its own files.

## Question 2 Answer
The machine code for question 2 can be found in `question1-2-3/machine_code.txt`.

## Question 3 Answer
The machine code for question 3 can be found in `question1-2-3/machine_code_with_o_flag.txt`.
The addition of the optimization `-O` flag reduced the number of machine code lines from 617 to 50. This drastically improved the performance of the program.

## Question 5 answer
This is a result from running the benchmarks for question 5 (actual result may vary).

*selection_sort*\
*time: [77.387 ms 81.195 ms 85.473 ms]*\
*Found 7 outliers among 100 measurements (7.00%)*\
*3 (3.00%) high mild*\
*4 (4.00%) high severe*

According to this, the time it takes for the `selection_sort()` function to complete ranges from 77.387ms (worst) - 85.473ms (best), with the median being 81.195ms. Furthermore, 7 samples out of 100 run were outliers.

## Question 7 answer
This is a result from running the benchmarks for question 7 (actual result may vary).

*selection_sort*\
*time: [68.113 ms 72.120 ms 76.800 ms]*\
*Found 8 outliers among 100 measurements (8.00%)*\
*3 (3.00%) high mild*\
*5 (5.00%) high severe*

According to this, the time it takes for the `selection_sort()` function to complete ranges from 68.113ms (worst) - 76.800ms (best), with the median being 72.120ms. Furthermore, 8 samples out of 100 were outliers.

## Question 8 answer
There were more outliers in `Question 5` benchmarks than in `Question 5` benchmarks, which indicates that iterations in `Question 7` were less consistent (and less predictable) in the time it took for them to run than in `Question 5`.

However, the time it takes for a sample to run in `Question 7` benchmarks is generally shorter than in `Question 8`. This indicates that the code benchmarked in `Question 7` is faster overall than in `Question 5`.

## Question 9 answer
A *zero cost abstraction* is an abstraction applied to current code that does not increase the run-time cost of an application. These abstractions usually have other effects, such as increase performance or enhancing code readability. Examples would be using functions in-place of repeated code.

In Question 8, *zero cost abstractions* were applied by using iterators and other built-in Rust functions, enhancing the code's performance.

## Notes
### To open table in DBProject
1. navigate to directory with the database: `DBProject/data/`
2. Run `sqlite3` in the terminal
3. Open the database using `.open users.db`
