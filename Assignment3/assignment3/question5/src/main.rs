use std::mem;

// question 5: Box function
fn vector_size_box<T>(box_var: &Box<T>) -> usize {
    return mem::size_of_val(&**box_var);
}

fn main() {
    let vec1 = vec![12, 32, 13];
    let vec2 = vec![44, 55, 16];

    {
        let vec1_iter = vec1.iter();

        // q5: wrap in Box
        let vec1_iter_box = Box::new(vec1_iter);
        println!("q5: vec1_iter_box size = {} bytes", vector_size_box(&vec1_iter_box));
    }
    {
        // combines both vec1 and vec2
        let vec_chained = vec1.iter().chain(vec2.iter());

        // q5: wrap in Box
        let vec_chained_box = Box::new(vec_chained);
        println!("q5: vec_chained_box size = {} bytes", vector_size_box(&vec_chained_box));
    }
    {
        // combines both vec1 and vec2
        let vec1_2=vec![vec1, vec2];
        let vec_flattened = vec1_2.iter().flatten(); 

        // q5: wrap in Box
        let vec_flattened_box = Box::new(vec_flattened);
        println!("q5: vec_flattened_box size = {} bytes", vector_size_box(&vec_flattened_box));
    }
}