use std::env;
use std::io::{self, Write};
use question3::*;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure at least one argument (the command)
    if args.len() < 2 {
        eprintln!("Usage: cargo run <command> [args]");
        return;
    }

    // Create the user database
    let usersdb = UserBase::new("data/users.db".to_string());

    // Match the command
    match args[1].as_str() {
        "new" => {
            if args.len() != 4 {
                eprintln!("Usage: cargo run new <username> <password>");
                return;
            }
            let username: &str = &args[2];
            let password: &str = &args[3];

            match usersdb.add_user(username, password) {
                Ok(()) => {
                    println!("Adding user {} with password {}…", username, password);
                    println!("Operation done successfully!");
                },
                Err(UBaseErr::DbErr(e)) => {
                    eprintln!("Error accessing database");
                },
                Err(UBaseErr::HashError(e)) => {
                    eprintln!("Error with password encryption");
                },
            }
            return;
        }

        "transfer" => {
            if args.len() != 5 {
                eprintln!("Usage: cargo run transfer <from_user> <to_user> <amount>");
                return;
            }
            let from_user = &args[2];
            let to_user = &args[3];
            let amount: i64 = args[4].parse().unwrap_or_else(|_| {
                eprintln!("Amount must be a number.");
                std::process::exit(1);
            });

            // Prompt for password
            print!("Please input your password: ");
            io::stdout().flush().unwrap();
            let mut password = String::new();
            io::stdin().read_line(&mut password).unwrap();
            let password = password.trim();

            // verify user
            match usersdb.verify_user(from_user, password) {
                Ok(valid) => {
                    if !valid {
                        eprintln!("Invalid password!");
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("Error verifying user");
                    return;
                }
            }

            println!("Sending money from {} to {}…", from_user, to_user);
            if let Err(e) = usersdb.pay(from_user, to_user, amount) {
                eprintln!("Error");
                return;
            }

            println!("Operation done successfully!");
        }

        "balance" => {
            if args.len() != 3 {
                eprintln!("Usage: cargo run balance <username>");
                return;
            }
            let username = &args[2];

            // Prompt for password
            print!("Please input your password: ");
            io::stdout().flush().unwrap();
            let mut password = String::new();
            io::stdin().read_line(&mut password).unwrap();
            let password = password.trim();

            // Verify user credentials
            match usersdb.verify_user(username, password) {
                Ok(valid) => {
                    if !valid {
                        eprintln!("Invalid password!");
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("Error verifying user:");
                    return;
                },
            }

            // Get and display balance
            match usersdb.get_user_balance(username) {
                Ok(balance) => {
                    println!("Balance is ${}", balance);
                    println!("Operation done successfully!");
                },
                Err(e) => {
                    eprintln!("Error retrieving balance");
                },
            }
        }

        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Available commands: new, transfer, balance");
        }
    }
}
