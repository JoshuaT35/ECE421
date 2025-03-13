// import from lib.rs
use question1::*;

use serial_test::serial;

// test: transaction history
#[test]
#[serial]
fn test_transaction_history() {
    // test data
    let test_user_from_name: &str = "test_user_from_name";
    let test_user_from_pw: &str = "test_user_from_pw";
    let test_user_to_name: &str = "test_user_to_name";
    let test_user_to_pw: &str = "test_user_to_pw";
    let test_t_amount: i64 = 100;

    // create a struct for the "users" database
    let usersdb = UserBase::new(
        "data/users.db".to_string(),
    );

    // --- delete test data from the database ---

    // open connection to the database
    let conn = sqlite::open(usersdb.get_fname()).unwrap();
    conn.execute("PRAGMA foreign_keys = ON;").unwrap();

    // delete from balances
    let mut st = conn.prepare(
        "DELETE FROM balances WHERE u_name = :u_from OR u_name = :u_to;"
    ).unwrap();
    st.bind((":u_from", test_user_from_name)).unwrap();
    st.bind((":u_to", test_user_to_name)).unwrap();
    st.next().unwrap();

    // delete from transactions
    st = conn.prepare(
        "DELETE FROM transactions WHERE u_from = :u_from AND u_to = :u_to;"
    ).unwrap();
    st.bind((":u_from", test_user_from_name)).unwrap();
    st.bind((":u_to", test_user_to_name)).unwrap();
    st.next().unwrap();

    // delete from users
    st = conn.prepare(
        "DELETE FROM users WHERE u_name = :u_from OR u_name = :u_to;"
    ).unwrap();
    st.bind((":u_from", test_user_from_name)).unwrap();
    st.bind((":u_to", test_user_to_name)).unwrap();
    st.next().unwrap();

    // insert test users into the database
    usersdb.add_user(test_user_from_name, test_user_from_pw).unwrap();
    usersdb.add_user(test_user_to_name, test_user_to_pw).unwrap();

    // test user pays
    // 1. <from> pays <to> $<t_mount>
    usersdb.pay(test_user_from_name, test_user_to_name, test_t_amount).unwrap();

    // print out transaction history of from
    usersdb.get_transactions_history(test_user_from_name).unwrap();

    // assert that 2 transactions made
    st = conn.prepare(
        "SELECT COUNT(*) FROM transactions WHERE u_from = :u_from AND u_to = :u_to;"
    ).unwrap();
    st.bind((":u_from", test_user_from_name)).unwrap();
    st.bind((":u_to", test_user_to_name)).unwrap();

    st.next().unwrap();
    let transaction_made_by_from: i64 = st.read(0).unwrap();
    assert_eq!(transaction_made_by_from, 1);

    // --- delete test data from the database ---

    // delete from balances
    st = conn.prepare(
        "DELETE FROM balances WHERE u_name = :u_from OR u_name = :u_to;"
    ).unwrap();
    st.bind((":u_from", test_user_from_name)).unwrap();
    st.bind((":u_to", test_user_to_name)).unwrap();
    st.next().unwrap();

    // delete from transactions
    st = conn.prepare(
        "DELETE FROM transactions WHERE u_from = :u_from AND u_to = :u_to;"
    ).unwrap();
    st.bind((":u_from", test_user_from_name)).unwrap();
    st.bind((":u_to", test_user_to_name)).unwrap();
    st.next().unwrap();

    // delete from users
    st = conn.prepare(
        "DELETE FROM users WHERE u_name = :u_from OR u_name = :u_to;"
    ).unwrap();
    st.bind((":u_from", test_user_from_name)).unwrap();
    st.bind((":u_to", test_user_to_name)).unwrap();
    st.next().unwrap();
}
