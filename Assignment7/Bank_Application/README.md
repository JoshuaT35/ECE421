# ECE421 Assignment 7 Question 1 - AKA BankApplication

## Author
Name: Joshua Tablan\
CCID: tablan\
student ID: 1726177

### To build
- Run `cargo build`.

### To run `main()`
- Run `cargo run`.

## Notes
1. All accounts in the bank are initialized to have $0.
2. When money is transferred, the account the money comes from is set to 0 if it has insufficient funds (instead of returning an error).
3. When money is transferred, the account the money goes to always receives the money.