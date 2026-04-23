// use std::cell::RefCell;
// use std::rc::Rc;

fn main() {
    // --- RefCell: dereferencing a value (borrow mut can be used as well) ---
    // let refcellptr: RefCell<i32> = RefCell::new(1);
    // println!("value is {:?}", refcellptr.borrow());

    // --- RefCell: cloning value ---
    // let refcellptr: RefCell<i32> = RefCell::new(2);
    // let a: RefCell<i32> = refcellptr.clone();
    // println!("value is {:?}", a.borrow());

    // --- RefCell: editing the value ---
    // let refcellptr: RefCell<i32> = RefCell::new(3);
    // {
    //     let mut a = refcellptr.borrow_mut();
    //     *a += 3; // Modify the value
    // } // The mutable borrow ends here, must be set since a borrow cannot be called after a 

    // let b = refcellptr.borrow();
    // println!("Updated value: {}", *b);

    // --- RefCell: multiple borrows (play around with borrow and mut borrow) ---
    // let refcellptr: RefCell<i32> = RefCell::new(4);
    // let mut a = refcellptr.borrow();
    // let mut b = refcellptr.borrow_mut();
    // conclusion: multiple borrows allowed ONLY if ALL borrows are borrow() [immutable]


    // --- Rc: retrieving value ---
    // let rcptr: Rc<i32> = Rc::new(1);
    // let a: i32 = *rcptr;
    // println!("value is {:?}", a);
    // println!("value is {:?}", rcptr);

    // --- Rc: cloning value (note that the * does not change the value being used when :? is used) ---
    // let rcptr: Rc<i32> = Rc::new(2);
    // let a: Rc<i32> = rcptr.clone();
    // println!("value is {:?}", a);
    // println!("value is {:?}", *rcptr);

    // --- Rc: getting number of references to the value (including the initial Rc pointer) ---
    // let rcptr: Rc<i32> = Rc::new(3);
    // println!("no. of references before creating a = {}", Rc::strong_count(&rcptr));
    // let a: Rc<i32> = rcptr.clone();
    // println!("no. of references after creating a = {}", Rc::strong_count(&rcptr));
    // println!("no. of references after creating a = {}", Rc::strong_count(&a));

    // --- Rc Notes ---
    // 1. Rc does not allow us to change the value we are pointing to. In the above cases, we cannot
    //     dereference either `rcptr` or `a` and change the value. All clones are mutable.


    // using rc and refcell to create multiple pointers to a value, and then editing that value using one of the pointers
    // let ptr: Rc<RefCell<i32>> = Rc::new(RefCell::new(10));
    // // cloning Rc pointer since Rc allows multiple references to exist
    // let a = ptr.clone();
    // let b = ptr.clone();

    // // borrowing b mutable to edit the value
    // {
    //     // * to dereference Rc to get inner RefCell, borrow_mut() performed on RefCell
    //     let mut borrowed_b = (*b).borrow_mut();
    //     // edit the value
    //     *borrowed_b = 20;
    // }

    // // print the new value
    // println!("ptr is {:?}", (*ptr).borrow());

}
