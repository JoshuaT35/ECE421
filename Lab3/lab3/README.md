# ECE421 Lab 3

## How to run

- For deliverable 1, change the name of `d1.rs` to `main.rs` and do `cargo run`.
- For deliverable 2, change the name of `d2.rs` to `main.rs` and do `cargo run`.
- For deliverable 3, change the name of `d3.rs` to `main.rs` and do `cargo run`.
- For deliverable 4, change the name of `d4.rs` to `main.rs` and do `cargo run`.

## Deliverable 3 Questions

To fix the `struct TreeNode`, we had to add a lifetime to it through `<'a>`. This is because the `data` variable of the struct is a reference to memory. The lifetime lets the compiler know the struct will only last as long as the memory that `data` refers to.

We also wrapped the `left_child` and `right_child` of the struct in `Option<Box<>>` since the current implementation caused infinite recursion by having TreeNode refer to itself.

## Deliverable 4 Questions

Empty is used in place of `Option<>`. It allows for a `Tree` with no data to take up minimal memory rather than create an empty `Node`.

The `struct` was better than the `enum` because it allows us to attach the `insert_node` function to the `struct`. This better structures the code.
