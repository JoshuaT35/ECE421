use std::mem;

struct Bag<T> {
    items: [T; 3],
}

fn bag_size_u8(bag: &Bag<u8>) -> usize {
    return mem::size_of_val(bag);
}
fn bag_size_u32(bag: &Bag<u32>) -> usize {
    return mem::size_of_val(bag);
}

fn main() {
    let b1 = Bag {
        items: [1u8, 2u8, 3u8],
    };
    let b2 = Bag {
        items: [1u32, 2u32, 3u32],
    };

    let bagsize1 = bag_size_u8(&b1);
    let bagsize2 = bag_size_u32(&b2);
    println!("size of First Bag = {} bytes", bagsize1);
    println!("size of Second Bag = {} bytes", bagsize2);
}
