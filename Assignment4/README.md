# ECE421 Assignment 4

## How to run main/test

- For question 1 code, run `cargo package --run question1`.
- For question 2 code, run `cargo package --run question2`.
- For question 3 code, run `cargo package --run question3`.
- For question 5 code, run `cargo package --run question5`.

## Question 2 Answers

The `cons` function in Rust appends a `value` to the front of a Rust `list`.
The first parameter, `car`, is the v`alue` to be appended to the front.
The second parameter, `cdr`, is the `list` that the `value` will be appended to.

## Question 3 Answers

The previous implementation (given in the assignment pdf) has an error at the line:
`task.id = 100`
Since the `task` struct instance was not declared using the `mut` keyword, changing the values of its fields is not allowed, hence  the error.

## Question 4 Answers

(a)
The program creates two DoubleNode structs, `node_a` and `node_b`. The program makes the `next` field of `node_a` point to `node_b`, and the `prev` field of `node_b` point to `node_a`.

(b)
Th data structure `DoubleNode` intends to implement a double-linked list, where each `DoubleNode` points to the `DoubleNode`s before and after it. `DoubleNode`s at the end of the list point to `None`.

(c)
`Rc<RefCell<Option<DoubleNode>>>` uses an `Rc` pointer, which keeps track of how many `Rc` pointers point to the same data. If this number reaches 0, then the memory is automatically deallocated. This way, we can keep track of how many `Rc`-like pointers to a `DoubleNode` exist.

`Weak<RefCell<Option<DoubleNode>>>` uses `Weak`, which is a version of `Rc` that has weak-ownership of the data. This means that a `Weak` pointer does not contribute to the number of `Rc`-like pointers that point to the data, and is thus not used in the counter for automatic deallocation. However, it means that a `DoubleNode` can be manually deallocated even if other `DoubleNodes` point to it.

(d)
`*a.borrow_mut()` dereferences both the `Rc` and `Refcell` pointers, extracting `Some(node_a)`.

`Some(node_a)` is then matched with `Some(ref mut x)`, setting `x` as a mutable reference to `node_a`.

Thus, `node_a` fields can be manipulated. `(*x).prev = Rc::clone(&b);` sets the `prev` field of `node_a` to be an `Rc` pointer that points to `b`. Because of this, `node_a` now has a pointer reference to `node_b` (by cloning the pointer of `b`).
