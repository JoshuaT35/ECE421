// import from lib.rs
use DBProject::*;

use sqlite::State;

// test to add a payment that already exists
#[test]
#[should_panic]
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

    // add test data to the database
    usersdb.pay(test_user_from, test_user_to, test_t_amount).unwrap();

    // add test data to the database again
    usersdb.pay(test_user_from, test_user_to, test_t_amount).unwrap();
}