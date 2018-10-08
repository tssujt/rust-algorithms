pub fn quick_sort(collection: Vec<usize>) -> Vec<usize> {
    let mut sorted = collection.clone();
    let len = sorted.len();
    _quick_sort(&mut sorted, 0, len - 1);

    return sorted;
}

fn partition(collection: &mut Vec<usize>, first: usize, last: usize) -> usize {
    let mut pivot = first;
    for i in first..last {
        if collection[i] < collection[last] {
            collection.swap(i, pivot);
            pivot += 1;
        }
    }
    collection.swap(last, pivot);

    return pivot;
}

fn _quick_sort(collection: &mut Vec<usize>, first: usize, last: usize) {
    if first < last {
        let pos = partition(collection, first, last);
        if pos == first {
            _quick_sort(collection, pos + 1, last);
        } else if pos == last {
            _quick_sort(collection, first, pos - 1);
        } else {
            _quick_sort(collection, first, pos - 1);
            _quick_sort(collection, pos + 1, last);
        }
    }
}
