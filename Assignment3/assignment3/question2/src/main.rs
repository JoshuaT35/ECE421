use std::mem;

struct Bag<T> {
    items: [T; 3],
}

fn bag_size<T>(bag: &Bag<T>) -> usize {
    return mem::size_of_val(bag);
}

fn main() {
    let b1 = Bag {
        items: [1u8, 2u8, 3u8],
    };
    let b2 = Bag {
        items: [1u32, 2u32, 3u32],
    };

    let bagsize1: usize = bag_size(&b1);
    let bagsize2: usize = bag_size(&b2);
    println!("size of First Bag = {} bytes", bagsize1);
    println!("size of Second Bag = {} bytes", bagsize2);
}