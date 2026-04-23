// https://crates.io/crates/hamcrest

use crate::a2q2::*;
use hamcrest::prelude::*;

// test: Player properties are of type String
#[test]
fn test_name_properties_string() {
    // create struct
    let player = Player {
        id: 10,
        first_name: "Firstname".to_string(),
        last_name: "lastname".to_string(),
    };

    // assert that names are Strings
    hamcrest::assert_that!(player.first_name, is(type_of::<String>()));
    hamcrest::assert_that!(player.last_name, is(type_of::<String>()));
}

// test: Player's with same IDs have the same first and last name
#[test]
fn test_name_properties_same_properties() {
    // create player structs
    let player1 = Player {
        id: 10,
        first_name: "Firstname".to_string(),
        last_name: "lastname".to_string(),
    };

    let player2 = Player {
        id: 10,
        first_name: "Firstname".to_string(),
        last_name: "lastname".to_string(),
    };

    // assert that properties are the same
    hamcrest::assert_that!(player1.id, is(equal_to(player2.id)));
    hamcrest::assert_that!(player1.first_name, is(equal_to(player2.first_name)));
    hamcrest::assert_that!(player1.last_name, is(equal_to(player2.last_name)));
}
