//! Run this file with `cargo test --test 04_merge_slices`.

//! TODO: Implement a function called `merge_slices`, which is useful for the merge sort algorithm.
//! It will take two sorted `u32` slices as inputs and merge them into a sorted vector (Vec).
//! The function will return the vector.
//!
//! The time complexity of this function has to be O(n).
//!
//! Bonus: Can you build a complete merge sort on top of this function? :)

fn merge_slices(left: &[u32], right: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut left_val = left_iter.next();
    let mut right_val = right_iter.next();
    while let (Some(l), Some(r)) = (left_val, right_val) {
        if l < r {
            result.push(*l);
            left_val = left_iter.next();
        } else {
            result.push(*r);
            right_val = right_iter.next();
        }
    }
    result.extend(left_val);
    result.extend(right_val);
    result.extend(left_iter);
    result.extend(right_iter);
    result
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::merge_slices;

    #[test]
    fn merge_slices_empty() {
        assert_eq!(merge_slices(&[], &[]), vec![]);
    }

    #[test]
    fn merge_slices_basic() {
        assert_eq!(merge_slices(&[1, 2, 3], &[4, 5, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_slices_interleaved() {
        assert_eq!(merge_slices(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_slices_duplicates() {
        assert_eq!(merge_slices(&[1, 1, 3], &[1, 3, 4]), vec![1, 1, 1, 3, 3, 4]);
    }

    #[test]
    fn merge_slices_uneven_size() {
        assert_eq!(
            merge_slices(&[1, 4, 6, 8], &[0, 1, 1, 3, 4, 5, 7, 8, 9]),
            vec![0, 1, 1, 1, 3, 4, 4, 5, 6, 7, 8, 8, 9]
        );
    }

    #[test]
    fn merge_slices_first_empty() {
        assert_eq!(merge_slices(&[], &[1, 4, 8]), vec![1, 4, 8]);
    }

    #[test]
    fn merge_slices_second_empty() {
        assert_eq!(merge_slices(&[1, 9, 11], &[]), vec![1, 9, 11]);
    }
}
