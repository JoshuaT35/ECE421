// import from lib.rs
use DBProject::*;

use sqlite::State;

// test to add a payment that does not exist
#[test]
fn test_add_pay_not_exist() {
    // create a struct for the "users" database
    let usersdb = UserBase::new(
        "data/users.db".to_string(),
    );

    // test user from data
    let test_user_name_from: &str = "test_user_name_from";
    let test_user_pw_from: &str = "test_user_pw_from";

    // test user to data
    let test_user_name_to: &str = "test_user_name_to";
    let test_user_pw_to: &str = "test_user_pw_to";
    
    // --- START: delete any rows in the table which match test_user_data ---

    // first delete any row in the table with that name to ensure uniqueness
    let conn_users = sqlite::open(usersdb.get_fname()).unwrap();
    let mut conn_users_stmt = conn_users.prepare(
        "DELETE FROM users WHERE u_name=? OR u_name=?"
    ).unwrap();

    // bind the test user name to the first ?
    conn_users_stmt.bind((1, test_user_name_from)).unwrap();
    conn_users_stmt.bind((2, test_user_name_to)).unwrap();

    // execute the delete
    conn_users_stmt.next().unwrap();

    // --- END: delete any rows in the table which match test_user_data ---

    // --- START: add user_from and user_to to the users database ---

    // add test users data to the database
    usersdb.add_user(test_user_name_from, test_user_pw_from).unwrap();
    usersdb.add_user(test_user_name_to, test_user_pw_to).unwrap();

    // --- END: add user_from and user_to to the users database ---
    

    // --- START: add pay data to the transaction table ---
    
    // test data
    let test_user_from: &str = test_user_name_from;
    let test_user_to: &str = test_user_name_to;
    let test_t_amount: i64 = 100;

    // --- START: delete any rows in the table which match test_user_data ---

    // first delete any row in the table with that name to ensure uniqueness
    let conn_transactions = sqlite::open(usersdb.get_fname()).unwrap();
    let mut conn_transactions_stmt = conn_transactions.prepare(
        "DELETE FROM transactions WHERE u_from=? AND u_to=? AND t_amount=?"
    ).unwrap();

    // bind the test data to the first ?
    conn_transactions_stmt.bind((1, test_user_from)).unwrap();
    conn_transactions_stmt.bind((2, test_user_to)).unwrap();
    conn_transactions_stmt.bind((3, test_t_amount)).unwrap();

    // execute the delete
    conn_transactions_stmt.next().unwrap();

    // --- END: delete any rows in the table which match test_user_data ---

    // --- START: add pay data to the transaction table ---

    // add test data to the database
    usersdb.pay(test_user_from, test_user_to, test_t_amount).unwrap();

    // get transaction data
    conn_transactions_stmt = conn_transactions.prepare(
        "SELECT t1.u_from, t1.u_to, t1.t_amount FROM transactions t1 WHERE t1.u_from=? AND t1.u_to=? AND t1.t_amount=?"
    ).unwrap();

    // bind the test data to the ?
    conn_transactions_stmt.bind((1, test_user_from)).unwrap();
    conn_transactions_stmt.bind((2, test_user_to)).unwrap();
    conn_transactions_stmt.bind((3, test_t_amount)).unwrap();

    // on https://crates.io/crates/sqlite
    // get the first row only (next)
    if let Ok(State::Row) = conn_transactions_stmt.next() {
        // get the data
        let actual_user_from: String = conn_transactions_stmt.read::<String, _>("u_from").unwrap();
        let actual_user_to: String = conn_transactions_stmt.read::<String, _>("u_to").unwrap();
        let actual_t_amount: i64 = conn_transactions_stmt.read::<String, _>("t_amount").unwrap().parse().unwrap();

        // assert that data is correct
        assert_eq!(test_user_from, actual_user_from);
        assert_eq!(test_user_to, actual_user_to);
        assert_eq!(test_t_amount, actual_t_amount);
    }
    else {
        panic!("test_pay - no users found when selecting from transactions table");
    }

    // --- END: add pay data to the transaction table ---
}