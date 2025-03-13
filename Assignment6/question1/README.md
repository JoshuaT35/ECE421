# ECE421 Assignment 6 - Question 1

## Author
Name: Joshua Tablan\
CCID: tablan\
student ID: 1726177

## How to run tests

### To build
- Build by running `cargo build`.

### To run tests
- Run `cargo test`.
Tests are run serially.

## Notes
1. Created table `balances` which tracks a user and the user's balance. Used the command:\
`CREATE TABLE balances(u_name text PRIMARY KEY, balance INTEGER NOT NULL, FOREIGN KEY (u_name) REFERENCES users(u_name));`
2. In tests, we assume that the test data should not appear in the database. For example, let's say a test user is called `test_user`. `test_user` is first deleted from the database before running tests. Hence, if a valid user signs up as `test_user`, then that user's profile will be delete when the tests are run.
