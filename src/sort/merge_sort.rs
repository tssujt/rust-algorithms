pub fn merge_sort(mut collection: Vec<usize>) -> Vec<usize> {
    let l: usize = collection.len();

    let mid = l / 2;

    if l > 1 {
        let left = merge_sort(collection[0..mid].to_vec());
        let right = merge_sort(collection[mid..l].to_vec());

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;

        let left_len = left.len();
        let right_len = right.len();

        while i < left_len && j < right_len {
            if left[i] < right[j] {
                collection[k] = left[i];
                i += 1;
            } else {
                collection[k] = right[j];
                j += 1;
            }
            k += 1;
        }
        while i < left_len {
            collection[k] = left[i];
            i += 1;
            k += 1;
        }

        while j < right_len {
            collection[k] = right[j];
            j += 1;
            k += 1;
        }
    }
    return collection;
}
