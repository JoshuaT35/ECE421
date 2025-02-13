# ECE 421 Assignment 3

# How to run
To run a question, type in the terminal:
`cargo build`
`cargo run --package <question-package-name>`
Where `<question-package-name>` can be `question1`, `question2`...

# Answers to written questions

## Q6
Both question 4 and 5 get the size of the different variables created, `vec1_iter`, `vec_chained`, and `vec_flattened`.
`vec1_iter` has the smallest size at 16 bytes since it is an iterator to `vector 1` only.
`vec_chained` has the next smallest size at 32 bytes since it is an combined iterator, made up of an iterator to `vector 1` and iterator to `vector 2`.
`vec_flattened` has the largest size at 48 bytes. This is because it is a vector combined from both `vector 1` and `vector 2`, instead of being an iterator.

The difference between question 4 and question 5 is the type of input passed into the function used to caculate the size of the variables created. Question 4 passes the variable directly, while question 5 passes a pointer (`Box`) to the variable.

## Q7
Polymorphism is the ability in programming to represent multiple different variable types as a single type, simplifying interfaces which may work with many of these variables.

Rust supports polymorphism principles. For example, we can use Rust's generic `T` type to reprsent multiple variable types such as `u8` and `u32` in other pieces of code.

`traits` in Rust are also a form of polymorphism. `traits` allow us to define a general interface that specific structs should follow.

## Q8
The `equal` function is called 4 times.

## Q9
The `equal` fucntion is called 0 times, and there are a few reasons why.

1. In line 11, the `compare` function has no return value.
2. The `equal` functions in lines 6 and 7 do not have their return value saved.
3. The values of `x` and `y` are not modified.

Since the `compare` and `equal` functions have virtually no effect on any of the program's variables, the compiler optimizes the function by not calling the `equal` function.
