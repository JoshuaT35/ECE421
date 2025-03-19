# ECE421 Assignment 7 Question 2 - AKA Assignment7_Q2

## Author
Name: Joshua Tablan\
CCID: tablan\
student ID: 1726177

### To build
- Run `cargo build`.

### To run `main()`
- Run `cargo run`.

Question 2(a) answers:
The given code fails because ownership of `sample_data` is moved into the first thread spawned. Thus, subsequent threads cannot access `sample_data`.