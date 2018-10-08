extern crate algorithms;


#[cfg(test)]
mod test {
    use algorithms::sort::bubble_sort::*;
    use algorithms::sort::bucket_sort::*;
    use algorithms::sort::insertion_sort::*;
    use algorithms::sort::merge_sort::*;
    use algorithms::sort::quick_sort::*;
    use algorithms::sort::selection_sort::*;

    #[test]
    fn test_bubble_sort_reverse() {
        assert_eq!(vec![1, 2, 3, 4], bubble_sort(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_bubble_sort_same() {
        assert_eq!(vec![1, 2, 3, 4], bubble_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_bubble_sort_random() {
        assert_eq!(vec![2, 4, 5, 6], bubble_sort(vec![6, 4, 5, 2]));
    }

    #[test]
    fn test_insertion_sort_reverse() {
        assert_eq!(vec![1, 2, 3, 4], insertion_sort(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_insertion_sort_same() {
        assert_eq!(vec![1, 2, 3, 4], insertion_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_insertion_sort_random() {
        assert_eq!(vec![2, 4, 5, 6], insertion_sort(vec![6, 4, 5, 2]));
    }

    #[test]
    fn test_bucket_sort_reverse() {
        assert_eq!(vec![1, 2, 3, 4], bucket_sort(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_bucket_sort_same() {
        assert_eq!(vec![1, 2, 3, 4], bucket_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_bucket_sort_random() {
        assert_eq!(vec![2, 4, 5, 6], bucket_sort(vec![6, 4, 5, 2]));
    }

    #[test]
    fn test_merge_sort_reverse() {
        assert_eq!(vec![1, 2, 3, 4], merge_sort(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_merge_sort_same() {
        assert_eq!(vec![1, 2, 3, 4], merge_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_merge_sort_random() {
        assert_eq!(vec![2, 4, 5, 6], merge_sort(vec![6, 4, 5, 2]));
    }

    #[test]
    fn test_quick_sort_reverse() {
        assert_eq!(vec![1, 2, 3, 4], quick_sort(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_quick_sort_same() {
        assert_eq!(vec![1, 2, 3, 4], quick_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_quick_sort_random() {
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], quick_sort(vec![6, 4, 5, 2, 3, 7, 1]));
    }

    #[test]
    fn test_selection_sort_reverse() {
        assert_eq!(vec![1, 2, 3, 4], selection_sort(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_selection_sort_same() {
        assert_eq!(vec![1, 2, 3, 4], selection_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_selection_sort_random() {
        assert_eq!(vec![2, 4, 5, 6], selection_sort(vec![6, 4, 5, 2]));
    }
}
