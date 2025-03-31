pub fn selection_sort(list: &mut Vec<i64>) {
    // for each list element
    for i in 0..list.len() {
        // find the smallest item in the list and its index relative to the list slice
        // i.e if list slice is indexes 1..., and smallest item is first item of slice
        // item is 0
        if let Some((smallest_item_idx, _)) = list[i..]
            .iter()
            .enumerate()
            .min_by_key(|&(_, &val)| val)
        {
            // add i for original list
            let smallest_item_idx: usize = i + smallest_item_idx;
            // place smallest item at the current index of the list we are tracking
            list.swap(i, smallest_item_idx);
        }
    }
}

fn main() {
    // example vector
    let mut unsorted: Vec<i64> = vec![3, 4, 1, 6, 8];

    // sort list
    selection_sort(&mut unsorted);

    // print all items in the list
    let unsorted_iter = unsorted.iter();
    print!("elements in list: ");
    for val in unsorted_iter {
        print!("{} ", val);
    }
    println!();
}
