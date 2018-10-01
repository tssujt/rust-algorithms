pub fn bubble_sort(mut collection: Vec<usize>) -> Vec<usize> {
    let l = collection.len();
    for i in 0..l - 1 {
        for j in 0..l - i - 1 {
            if collection[j] > collection[j + 1] {
                collection.swap(j, j + 1);
                println!("j: {}, {:?}", j, collection);
            }
        }
    }
    return collection;
}
