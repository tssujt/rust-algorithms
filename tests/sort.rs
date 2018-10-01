#[macro_use]
extern crate algorithms;


#[cfg(test)]
mod test {
    use algorithms::sort::bubble_sort::*;
    use algorithms::sort::insertion_sort::*;

    #[test]
    fn test_bubble_sort_reverse() {
        test_reverse!(bubble_sort);
    }

    #[test]
    fn test_bubble_sort_same() {
        test_same!(bubble_sort);
    }

    #[test]
    fn test_bubble_random_same() {
        test_random!(bubble_sort);
    }

    #[test]
    fn test_insertion_sort_reverse() {
        test_reverse!(insertion_sort);
    }

    #[test]
    fn test_insertion_sort_same() {
        test_same!(insertion_sort);
    }

    #[test]
    fn test_insertion_random_same() {
        test_random!(insertion_sort);
    }
}
