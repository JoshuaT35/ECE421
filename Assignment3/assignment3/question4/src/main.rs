use std::mem;

fn vector_size<T>(vec: &T) -> usize {
    return mem::size_of_val(vec);
}

fn main() {
    let vec1 = vec![12, 32, 13];
    let vec2 = vec![44, 55, 16];

    {
        let vec1_iter = vec1.iter();

        // q4
        println!("vec1_iter size = {} bytes", vector_size(&vec1_iter));
    }
    {
        // combines both vec1 and vec2
        let vec_chained = vec1.iter().chain(vec2.iter());

        // q4
        println!("vec_chained size = {} bytes", vector_size(&vec_chained));
    }
    {
        // combines both vec1 and vec2
        let vec1_2=vec![vec1, vec2];
        let vec_flattened = vec1_2.iter().flatten(); 

        println!("vec_flattened size = {} bytes", vector_size(&vec_flattened));
    }
}
