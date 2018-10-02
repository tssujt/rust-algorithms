const BUCKET_SIZE: usize = 2;

pub fn bucket_sort(collection: Vec<usize>) -> Vec<usize> {
    let min = collection.iter().min().unwrap().to_owned();
    let max = collection.iter().max().unwrap().to_owned();

    let count = (max - min) / BUCKET_SIZE + 1;
    let mut buckets: Vec<Vec<usize>> = Vec::new();

    for _count in 0..count {
        buckets.push(Vec::new());
    }

    for num in collection {
        let index = (num - min) / BUCKET_SIZE;
        buckets[index].push(num);
    }

    let mut sorted_array: Vec<usize> = Vec::new();
    for _bucket in buckets {
        let mut bucket = _bucket;
        bucket.sort();
        sorted_array.extend(bucket);
    }

    return sorted_array;
}
