pub fn selection_sort(list: &mut Vec<i64>) {
    // for each list element
    for i in 0..list.len() {
        if let Some((smallest_item_idx, _)) = list[i..]
            .iter()
            .enumerate()
            .min_by_key(|&(_, &val)| val)
        {
            let smallest_item_idx = i + smallest_item_idx;
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
