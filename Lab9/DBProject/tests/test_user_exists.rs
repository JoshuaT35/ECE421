// import from lib.rs
use DBProject::*;

// test for adding a user that already exists
#[test]
#[should_panic]
fn test_add_user_already_exists() {
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

    // add test data to the database
    usersdb.add_user(test_user_name, test_user_pw).unwrap();
    
    // add user again (should panic)
    usersdb.add_user(test_user_name, test_user_pw).unwrap();
}