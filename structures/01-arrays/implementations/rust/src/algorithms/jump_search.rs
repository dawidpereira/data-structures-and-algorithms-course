//! Jump search implementations for arrays.
//!
//! This module provides jump search functionality for sorted arrays.
//! Jump search is faster than linear search but simpler than binary search.
//!
//! For theory and visual explanations, see:
//! - Jump Search Theory: `algorithms/searching/jump-search/`
//! - Array-Specific Details: `structures/01-arrays/algorithms/jump-search.md`

use crate::core::Array;
use crate::dynamic_array::DynamicArray;
use std::cmp::{min, Ordering};

/// Trait for types that support jump search operations.
/// Requires sorted data for correct operation.
pub trait JumpSearchable<T> {
    /// Performs jump search on a sorted array.
    ///
    /// Returns the index of the target if found, or `None` if not found.
    /// Uses optimal jump size of √n.
    ///
    /// # Requirements
    /// - The array must be sorted in ascending order
    ///
    /// # Examples
    /// ```
    /// use arrays::core::Array;
    /// use arrays::algorithms::JumpSearchable;
    ///
    /// let arr = Array::from_slice(&[1, 3, 5, 7, 9, 11, 13], 10).unwrap();
    /// assert_eq!(arr.jump_search(&7), Some(3));
    /// assert_eq!(arr.jump_search(&8), None);
    /// ```
    fn jump_search(&self, target: &T) -> Option<usize>
    where
        T: Ord;

    /// Performs jump search with a custom jump size.
    ///
    /// Allows manual control over the jump size for specific use cases.
    ///
    /// # Examples
    /// ```
    /// use arrays::core::Array;
    /// use arrays::algorithms::JumpSearchable;
    ///
    /// let arr = Array::from_slice(&[1, 3, 5, 7, 9, 11], 10).unwrap();
    /// // Use jump size of 2
    /// assert_eq!(arr.jump_search_with_size(&9, 2), Some(4));
    /// ```
    fn jump_search_with_size(&self, target: &T, jump_size: usize) -> Option<usize>
    where
        T: Ord;

    /// Finds the leftmost (first) occurrence using jump search.
    ///
    /// Useful when the array contains duplicates.
    fn jump_search_first(&self, target: &T) -> Option<usize>
    where
        T: Ord;

    /// Finds the rightmost (last) occurrence using jump search.
    ///
    /// Useful when the array contains duplicates.
    fn jump_search_last(&self, target: &T) -> Option<usize>
    where
        T: Ord;
}

// Helper function to calculate optimal jump size
fn optimal_jump_size(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    // Calculate √n, rounding up for better coverage
    let sqrt = (n as f64).sqrt().ceil() as usize;
    sqrt.max(1) // Ensure at least 1
}

// Helper macro to implement jump search for both array types
macro_rules! impl_jump_search {
    ($type:ty) => {
        impl<T> JumpSearchable<T> for $type {
            fn jump_search(&self, target: &T) -> Option<usize>
            where
                T: Ord,
            {
                let n = self.len();
                if n == 0 {
                    return None;
                }

                let jump = optimal_jump_size(n);
                self.jump_search_with_size(target, jump)
            }

            fn jump_search_with_size(&self, target: &T, jump_size: usize) -> Option<usize>
            where
                T: Ord,
            {
                let n = self.len();
                if n == 0 || jump_size == 0 {
                    return None;
                }

                // Jump phase: Find the block where element may exist
                let mut prev = 0;
                let mut curr = min(jump_size, n) - 1;

                // Jump until we find a block where target might be
                while curr < n && self.get(curr)? < target {
                    prev = curr + 1;
                    curr = min(curr + jump_size, n - 1);

                    // If we've reached the end, break
                    if curr == n - 1 && prev > n - jump_size {
                        break;
                    }
                }

                // Linear search phase: Search within the identified block
                while prev <= curr && prev < n {
                    match self.get(prev)?.cmp(target) {
                        Ordering::Equal => return Some(prev),
                        Ordering::Greater => return None, // Sorted array, can't find it
                        Ordering::Less => prev += 1,
                    }
                }

                None
            }

            fn jump_search_first(&self, target: &T) -> Option<usize>
            where
                T: Ord,
            {
                let n = self.len();
                if n == 0 {
                    return None;
                }

                let jump = optimal_jump_size(n);
                let mut result = None;

                // Find any occurrence first
                if let Some(idx) = self.jump_search_with_size(target, jump) {
                    result = Some(idx);

                    // Now search backwards to find the first occurrence
                    let mut i = idx;
                    while i > 0 {
                        if let Some(elem) = self.get(i - 1) {
                            if elem == target {
                                result = Some(i - 1);
                                i -= 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }

                result
            }

            fn jump_search_last(&self, target: &T) -> Option<usize>
            where
                T: Ord,
            {
                let n = self.len();
                if n == 0 {
                    return None;
                }

                let jump = optimal_jump_size(n);
                let mut result = None;

                // Find any occurrence first
                if let Some(idx) = self.jump_search_with_size(target, jump) {
                    result = Some(idx);

                    // Now search forwards to find the last occurrence
                    let mut i = idx;
                    while i < n - 1 {
                        if let Some(elem) = self.get(i + 1) {
                            if elem == target {
                                result = Some(i + 1);
                                i += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }

                result
            }
        }
    };
}

// Implement for both array types
impl_jump_search!(Array<T>);
impl_jump_search!(DynamicArray<T>);

/// Additional jump search utilities
pub mod utils {
    use super::*;

    /// Performs adaptive jump search that adjusts jump size based on comparisons.
    pub fn adaptive_jump_search<T: Ord>(arr: &Array<T>, target: &T) -> Option<usize> {
        let n = arr.len();
        if n == 0 {
            return None;
        }

        let mut jump = optimal_jump_size(n);
        let mut prev = 0;

        loop {
            let curr = min(prev + jump - 1, n - 1);

            match arr.get(curr)?.cmp(target) {
                Ordering::Equal => return Some(curr),
                Ordering::Less => {
                    prev = curr + 1;
                    if prev >= n {
                        return None;
                    }
                }
                Ordering::Greater => {
                    // Reduce jump size and search in current block
                    if jump == 1 {
                        return None;
                    }
                    jump = jump / 2;
                    continue;
                }
            }

            if prev + jump >= n {
                // Last block, do linear search
                for i in prev..n {
                    match arr.get(i)?.cmp(target) {
                        Ordering::Equal => return Some(i),
                        Ordering::Greater => return None,
                        Ordering::Less => continue,
                    }
                }
                return None;
            }
        }
    }

    /// Checks if an array is sorted (precondition for jump search).
    pub fn is_sorted_for_jump<T: Ord>(arr: &Array<T>) -> bool {
        for i in 1..arr.len() {
            if arr.get(i - 1) > arr.get(i) {
                return false;
            }
        }
        true
    }

    /// Finds the optimal jump size experimentally for a given array.
    pub fn find_optimal_jump_size<T: Ord>(arr: &Array<T>, samples: &[T]) -> usize {
        let n = arr.len();
        if n == 0 || samples.is_empty() {
            return optimal_jump_size(n);
        }

        let mut best_size = 1;
        let mut min_comparisons = usize::MAX;

        // Try different jump sizes
        for size in 1..=(n.min(100)) {
            let mut total_comparisons = 0;

            for target in samples {
                let mut comparisons = 0;
                let mut prev = 0;

                // Simulate jump search
                while prev < n {
                    let curr = min(prev + size - 1, n - 1);
                    comparisons += 1;

                    if let Some(elem) = arr.get(curr) {
                        if elem >= target {
                            // Linear search phase
                            for i in prev..=curr {
                                comparisons += 1;
                                if arr.get(i) == Some(target) {
                                    break;
                                }
                            }
                            break;
                        }
                    }
                    prev = curr + 1;
                }

                total_comparisons += comparisons;
            }

            if total_comparisons < min_comparisons {
                min_comparisons = total_comparisons;
                best_size = size;
            }
        }

        best_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_search_basic() {
        let arr = Array::from_slice(&[1, 3, 5, 7, 9, 11, 13, 15, 17], 10).unwrap();

        // Test finding existing elements
        assert_eq!(arr.jump_search(&1), Some(0));
        assert_eq!(arr.jump_search(&9), Some(4));
        assert_eq!(arr.jump_search(&17), Some(8));

        // Test not finding elements
        assert_eq!(arr.jump_search(&0), None);
        assert_eq!(arr.jump_search(&4), None);
        assert_eq!(arr.jump_search(&18), None);
    }

    #[test]
    fn test_jump_search_empty() {
        let arr: Array<i32> = Array::new(10);
        assert_eq!(arr.jump_search(&5), None);
    }

    #[test]
    fn test_jump_search_single_element() {
        let arr = Array::from_slice(&[42], 5).unwrap();
        assert_eq!(arr.jump_search(&42), Some(0));
        assert_eq!(arr.jump_search(&41), None);
        assert_eq!(arr.jump_search(&43), None);
    }

    #[test]
    fn test_custom_jump_size() {
        let arr = Array::from_slice(&[1, 3, 5, 7, 9, 11, 13, 15], 10).unwrap();

        // Test with jump size 1 (degenerates to linear search)
        assert_eq!(arr.jump_search_with_size(&7, 1), Some(3));

        // Test with jump size 4
        assert_eq!(arr.jump_search_with_size(&13, 4), Some(6));

        // Test with jump size larger than array
        assert_eq!(arr.jump_search_with_size(&5, 20), Some(2));
    }

    #[test]
    fn test_jump_search_duplicates() {
        let arr = Array::from_slice(&[1, 2, 2, 2, 3, 4, 5], 10).unwrap();

        // Regular jump search (might return any of the 2s)
        let result = arr.jump_search(&2);
        assert!(result.is_some());
        let idx = result.unwrap();
        assert!(idx >= 1 && idx <= 3);

        // First occurrence
        assert_eq!(arr.jump_search_first(&2), Some(1));

        // Last occurrence
        assert_eq!(arr.jump_search_last(&2), Some(3));
    }

    #[test]
    fn test_optimal_jump_size() {
        assert_eq!(optimal_jump_size(0), 0);
        assert_eq!(optimal_jump_size(1), 1);
        assert_eq!(optimal_jump_size(4), 2);
        assert_eq!(optimal_jump_size(9), 3);
        assert_eq!(optimal_jump_size(16), 4);
        assert_eq!(optimal_jump_size(100), 10);
        assert_eq!(optimal_jump_size(1000), 32); // ceil(√1000) = 32
    }

    #[test]
    fn test_dynamic_array_jump_search() {
        let mut arr = DynamicArray::new();
        for i in &[1, 3, 5, 7, 9, 11, 13, 15, 17, 19] {
            arr.push(*i);
        }

        assert_eq!(arr.jump_search(&11), Some(5));
        assert_eq!(arr.jump_search(&12), None);
        assert_eq!(arr.jump_search_first(&5), Some(2));
    }

    #[test]
    fn test_adaptive_jump_search() {
        let arr = Array::from_slice(&[1, 3, 5, 7, 9, 11, 13, 15, 17], 10).unwrap();

        assert_eq!(utils::adaptive_jump_search(&arr, &7), Some(3));
        assert_eq!(utils::adaptive_jump_search(&arr, &8), None);
        assert_eq!(utils::adaptive_jump_search(&arr, &17), Some(8));
    }

    #[test]
    fn test_is_sorted_check() {
        let sorted = Array::from_slice(&[1, 2, 3, 4, 5], 5).unwrap();
        assert!(utils::is_sorted_for_jump(&sorted));

        let unsorted = Array::from_slice(&[1, 3, 2, 4, 5], 5).unwrap();
        assert!(!utils::is_sorted_for_jump(&unsorted));
    }

    #[test]
    fn test_edge_cases() {
        // Array with same elements
        let same = Array::from_slice(&[5, 5, 5, 5], 5).unwrap();
        assert!(same.jump_search(&5).is_some());
        assert_eq!(same.jump_search_first(&5), Some(0));
        assert_eq!(same.jump_search_last(&5), Some(3));

        // Large gaps between elements
        let gaps = Array::from_slice(&[1, 100, 200, 300, 400], 5).unwrap();
        assert_eq!(gaps.jump_search(&200), Some(2));
        assert_eq!(gaps.jump_search(&150), None);
    }

    #[test]
    fn test_performance_comparison() {
        // Create a large sorted array
        let mut arr = DynamicArray::with_capacity(10000);
        for i in 0..10000 {
            arr.push(i * 2); // Even numbers
        }

        // Jump search should find elements efficiently
        assert_eq!(arr.jump_search(&5000), Some(2500));
        assert_eq!(arr.jump_search(&9998), Some(4999));
        assert_eq!(arr.jump_search(&5001), None); // Odd number
    }
}
