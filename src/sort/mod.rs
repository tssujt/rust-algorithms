pub mod bubble_sort;
pub mod insertion_sort;

#[macro_export]
macro_rules! test_reverse {
    ($func_name:ident) => (
        fn test_reverse() {
            assert_eq!(vec![1, 2, 3, 4], $func_name(vec![4, 3, 2, 1]));
        }
    )
}

#[macro_export]
macro_rules! test_same {
    ($func_name:ident) => (
        fn test_reverse() {
            assert_eq!(vec![1, 2, 3, 4], $func_name(vec![1, 2, 3, 4]));
        }
    )
}

#[macro_export]
macro_rules! test_random {
    ($func_name:ident) => (
        fn test_reverse() {
            assert_eq!(vec![2, 4, 5, 6], $func_name(vec![6, 4, 5, 2]));
        }
    )
}
