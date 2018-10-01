pub fn insertion_sort(mut collection: Vec<usize>) -> Vec<usize> {
    let l = collection.len();
    for i in 1..l {
        let num = collection[i];
        let mut j = i;
        while j > 0 && num < collection[j - 1] {
            collection[j] = collection[j - 1];
            j -= 1;
        }
        collection[j] = num;
    }
    return collection;
}
