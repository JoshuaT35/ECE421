pub fn selection_sort(list: &mut Vec<i64>) {
    // for each list element
    for i in 0..list.len() {
        // get current item pointed to by i
        let mut smallest_item_idx = i;
        let mut current_min = list[i];

        // go through all other elements
        for j in (i+1)..list.len() {
            // get index of smallest item in other elements
            if list[j] < current_min {
                current_min = list[j];
                smallest_item_idx = j;
            }
        }

        // swap item pointed at i, with smallest item in other elements
        if i != smallest_item_idx {
            let temp = list[i];
            list[i] = list[smallest_item_idx];
            list[smallest_item_idx] = temp;
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
