pub fn selection_sort(mut collection: Vec<usize>) -> Vec<usize> {
    let l = collection.len();

    for i in 0..l {
        let mut min_i = i;
        for j in min_i + 1..l {
            if collection[j] < collection[min_i] {
                min_i = j;
            }
        }
        collection.swap(min_i, i);
    }

    return collection;
}
