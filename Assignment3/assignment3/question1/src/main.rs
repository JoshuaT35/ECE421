struct Bag<T> {
    items: [T; 3],
}

fn main() {
    let b1 = Bag {
        items: [1u8, 2u8, 3u8],
    };
    let b2 = Bag {
        items: [1u32, 2u32, 3u32],
    };
}
