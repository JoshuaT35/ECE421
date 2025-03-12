// import from lib.rs
use DBProject::*;

use bcrypt::{
    verify,
};
use sqlite::State;

// test to add a user that does not exist
#[test]
fn test_add_user_not_exist() {
    // create a struct for the "users" database
    let usersdb = UserBase::new(
        "data/users.db".to_string(),
    );
    
    // test user data
    let test_user_name: &str = "test_user_name";
    let test_user_pw: &str = "test_user_pw";

    // --- START: delete any rows in the table which match test_user_data ---

    // first delete any row in the table with that name to ensure uniqueness
    let conn = sqlite::open(usersdb.get_fname()).unwrap();
    let mut stmt = conn.prepare(
        "DELETE FROM users WHERE u_name=?"
    ).unwrap();

    // bind the test user name to the first ?
    stmt.bind((1, test_user_name)).unwrap();

    // execute the delete
    stmt.next().unwrap();

    // --- END: delete any rows in the table which match test_user_data ---

    // --- START: add test data to the table ---

    // add test data to the database
    usersdb.add_user(test_user_name, test_user_pw).unwrap();

    // add user data inside
    stmt = conn.prepare(
        "SELECT u1.u_name, u1.p_word FROM users u1 WHERE u1.u_name=?"
    ).unwrap();

    // bind the test user name to the first ?
    stmt.bind((1, test_user_name)).unwrap();

    // --- END: add test data to the table ---

    // --- assert that test data was added ---

    // on https://crates.io/crates/sqlite
    // get the first row only (next)
    if let Ok(State::Row) = stmt.next() {
        let actual_user_name: String = stmt.read::<String, _>("u_name").unwrap();
        let actual_user_pw_hashed: String = stmt.read::<String, _>("p_word").unwrap();

        // verify password
        let hash_to_pw: bool = bcrypt::verify(test_user_pw, &actual_user_pw_hashed).unwrap();

        // assertions
        assert_eq!(test_user_name, actual_user_name);
        assert!(hash_to_pw);
    }
    else {
        panic!("test_add_user: No users found.");
    }
}
