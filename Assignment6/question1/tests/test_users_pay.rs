// import from lib.rs
use question1::*;

use serial_test::serial;

// test: pay with enough money
#[test]
#[serial]
fn test_pay_with_enough_money() {
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

    // get their balances before
    let test_user_from_balance_before: i64 = usersdb.get_user_balance(test_user_from_name).unwrap();
    let test_user_to_balance_before: i64 = usersdb.get_user_balance(test_user_to_name).unwrap();

    // test user pays
    usersdb.pay(test_user_from_name, test_user_to_name, test_t_amount).unwrap();

    // assert that transaction appears
    st = conn.prepare(
        "SELECT COUNT(*) FROM transactions WHERE u_from = :u_from AND u_to = :u_to;"
    ).unwrap();
    st.bind((":u_from", test_user_from_name)).unwrap();
    st.bind((":u_to", test_user_to_name)).unwrap();

    st.next().unwrap();
    let transaction_exists: i64 = st.read(0).unwrap();
    assert!(transaction_exists > 0);

    // assert that user balances updated correctly
    let test_user_from_balance_after: i64 = usersdb.get_user_balance(test_user_from_name).unwrap();
    let test_user_to_balance_after: i64 = usersdb.get_user_balance(test_user_to_name).unwrap();

    assert_eq!(test_user_from_balance_before-test_t_amount, test_user_from_balance_after);
    assert_eq!(test_user_to_balance_before+test_t_amount, test_user_to_balance_after);

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



// test: pay with no enough money
#[test]
#[serial]
#[should_panic(expected = "pay: user does not have enough money")]
fn test_pay_with_not_enough_money() {
    // test data
    let test_user_from_name: &str = "test_user_from_name";
    let test_user_from_pw: &str = "test_user_from_pw";
    let test_user_to_name: &str = "test_user_to_name";
    let test_user_to_pw: &str = "test_user_to_pw";
    let test_t_amount: i64 = 10000000000;

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
    usersdb.pay(test_user_from_name, test_user_to_name, test_t_amount).unwrap();

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