# ECE421 Assignment 6 - Question 2

## Author
Name: Joshua Tablan
CCID: tablan
student ID: 1726177

## How to run main/tests

### To build
- Build by running `cargo build`.

### To run `main()`
- Run `cargo run <new, transfer balance> <args>`.

## Notes
1. Created table `balances` which tracks a user and the user's balance. Used the command:\
CREATE TABLE balances(u_name text PRIMARY KEY, balance INTEGER NOT NULL, FOREIGN KEY (u_name) REFERENCES users(u_name));
