//! Binary search implementations for arrays.
//!
//! This module provides binary search functionality for both fixed-size
//! arrays (Array<T>) and dynamic arrays (DynamicArray<T>).
//!
//! For theory and visual explanations, see:
//! - Binary Search Theory: `algorithms/searching/binary-search/`
//! - Array-Specific Details: `structures/01-arrays/algorithms/binary-search.md`

use crate::core::Array;
use crate::dynamic_array::DynamicArray;
use std::cmp::Ordering;

/// Trait for types that support binary search operations.
pub trait BinarySearchable<T> {
    /// Searches for `target` in the sorted array.
    ///
    /// Returns the index of the element if found, or `None` if not found.
    ///
    /// # Requirements
    /// - The array must be sorted in ascending order
    /// - Elements must implement `Ord` for comparison
    ///
    /// # Examples
    /// ```
    /// use arrays::core::Array;
    /// use arrays::algorithms::BinarySearchable;
    ///
    /// let arr = Array::from_slice(&[1, 3, 5, 7, 9], 10).unwrap();
    /// assert_eq!(arr.binary_search(&5), Some(2));
    /// assert_eq!(arr.binary_search(&6), None);
    /// ```
    fn binary_search(&self, target: &T) -> Option<usize>
    where
        T: Ord;

    /// Searches using a custom comparison function.
    ///
    /// This is useful when you need custom ordering or when searching
    /// by a key that's different from the element type.
    ///
    /// The comparator function should return:
    /// - `Ordering::Less` if the element is less than the target
    /// - `Ordering::Equal` if the element matches the target
    /// - `Ordering::Greater` if the element is greater than the target
    fn binary_search_by<F>(&self, f: F) -> Option<usize>
    where
        F: FnMut(&T) -> Ordering;

    /// Finds the leftmost (first) occurrence of the target.
    ///
    /// Useful when the array contains duplicates.
    fn binary_search_first(&self, target: &T) -> Option<usize>
    where
        T: Ord;

    /// Finds the rightmost (last) occurrence of the target.
    ///
    /// Useful when the array contains duplicates.
    fn binary_search_last(&self, target: &T) -> Option<usize>
    where
        T: Ord;

    /// Finds the insertion point for the target to maintain sorted order.
    ///
    /// Returns the index where the target should be inserted.
    /// If the element already exists, returns the index of the first occurrence.
    fn binary_search_insertion_point(&self, target: &T) -> usize
    where
        T: Ord;
}

// Helper macro to implement binary search for both array types
macro_rules! impl_binary_search {
    ($type:ty) => {
        impl<T> BinarySearchable<T> for $type {
            fn binary_search(&self, target: &T) -> Option<usize>
            where
                T: Ord,
            {
                if self.is_empty() {
                    return None;
                }

                let mut low = 0;
                let mut high = self.len() - 1;

                while low <= high {
                    // Prevent overflow with large indices
                    let mid = low + (high - low) / 2;

                    match self.get(mid)?.cmp(target) {
                        Ordering::Equal => return Some(mid),
                        Ordering::Less => low = mid + 1,
                        Ordering::Greater => {
                            // Prevent underflow when mid is 0
                            if mid == 0 {
                                break;
                            }
                            high = mid - 1;
                        }
                    }
                }

                None
            }

            fn binary_search_by<F>(&self, mut f: F) -> Option<usize>
            where
                F: FnMut(&T) -> Ordering,
            {
                if self.is_empty() {
                    return None;
                }

                let mut low = 0;
                let mut high = self.len() - 1;

                while low <= high {
                    let mid = low + (high - low) / 2;

                    match f(self.get(mid)?) {
                        Ordering::Equal => return Some(mid),
                        Ordering::Less => low = mid + 1,
                        Ordering::Greater => {
                            if mid == 0 {
                                break;
                            }
                            high = mid - 1;
                        }
                    }
                }

                None
            }

            fn binary_search_first(&self, target: &T) -> Option<usize>
            where
                T: Ord,
            {
                if self.is_empty() {
                    return None;
                }

                let mut low = 0;
                let mut high = self.len() - 1;
                let mut result = None;

                while low <= high {
                    let mid = low + (high - low) / 2;

                    match self.get(mid)?.cmp(target) {
                        Ordering::Equal => {
                            result = Some(mid);
                            // Continue searching in the left half
                            if mid == 0 {
                                break;
                            }
                            high = mid - 1;
                        }
                        Ordering::Less => low = mid + 1,
                        Ordering::Greater => {
                            if mid == 0 {
                                break;
                            }
                            high = mid - 1;
                        }
                    }
                }

                result
            }

            fn binary_search_last(&self, target: &T) -> Option<usize>
            where
                T: Ord,
            {
                if self.is_empty() {
                    return None;
                }

                let mut low = 0;
                let mut high = self.len() - 1;
                let mut result = None;

                while low <= high {
                    let mid = low + (high - low) / 2;

                    match self.get(mid)?.cmp(target) {
                        Ordering::Equal => {
                            result = Some(mid);
                            // Continue searching in the right half
                            low = mid + 1;
                        }
                        Ordering::Less => low = mid + 1,
                        Ordering::Greater => {
                            if mid == 0 {
                                break;
                            }
                            high = mid - 1;
                        }
                    }
                }

                result
            }

            fn binary_search_insertion_point(&self, target: &T) -> usize
            where
                T: Ord,
            {
                if self.is_empty() {
                    return 0;
                }

                let mut low = 0;
                let mut high = self.len();

                while low < high {
                    let mid = low + (high - low) / 2;

                    match self.get(mid) {
                        Some(elem) if elem < target => low = mid + 1,
                        _ => high = mid,
                    }
                }

                low
            }
        }
    };
}

// Implement for both array types
impl_binary_search!(Array<T>);
impl_binary_search!(DynamicArray<T>);

/// Additional binary search utilities
pub mod utils {
    use super::*;

    /// Performs binary search on a slice (for comparison with our implementations).
    pub fn binary_search_slice<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
        slice.binary_search(target).ok()
    }

    /// Checks if an array is sorted (precondition for binary search).
    pub fn is_sorted<T: Ord>(arr: &Array<T>) -> bool {
        for i in 1..arr.len() {
            if arr.get(i - 1) > arr.get(i) {
                return false;
            }
        }
        true
    }

    /// Checks if a dynamic array is sorted.
    pub fn is_sorted_dynamic<T: Ord>(arr: &DynamicArray<T>) -> bool {
        for i in 1..arr.len() {
            if arr.get(i - 1) > arr.get(i) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_basic() {
        let arr = Array::from_slice(&[1, 3, 5, 7, 9, 11, 13], 10).unwrap();

        // Test finding existing elements
        assert_eq!(arr.binary_search(&1), Some(0));
        assert_eq!(arr.binary_search(&7), Some(3));
        assert_eq!(arr.binary_search(&13), Some(6));

        // Test not finding elements
        assert_eq!(arr.binary_search(&0), None);
        assert_eq!(arr.binary_search(&4), None);
        assert_eq!(arr.binary_search(&14), None);
    }

    #[test]
    fn test_binary_search_empty() {
        let arr: Array<i32> = Array::new(10);
        assert_eq!(arr.binary_search(&5), None);
    }

    #[test]
    fn test_binary_search_single_element() {
        let arr = Array::from_slice(&[42], 5).unwrap();
        assert_eq!(arr.binary_search(&42), Some(0));
        assert_eq!(arr.binary_search(&41), None);
        assert_eq!(arr.binary_search(&43), None);
    }

    #[test]
    fn test_binary_search_duplicates() {
        let arr = Array::from_slice(&[1, 2, 2, 2, 3, 4, 5], 10).unwrap();

        // Regular binary search (might return any of the 2s)
        let result = arr.binary_search(&2);
        assert!(result.is_some());
        let idx = result.unwrap();
        assert!(idx >= 1 && idx <= 3);

        // First occurrence
        assert_eq!(arr.binary_search_first(&2), Some(1));

        // Last occurrence
        assert_eq!(arr.binary_search_last(&2), Some(3));
    }

    #[test]
    fn test_binary_search_by() {
        #[derive(Debug, PartialEq, Clone)]
        struct Person {
            name: String,
            age: u32,
        }

        let people = Array::from_slice(
            &[
                Person {
                    name: "Alice".to_string(),
                    age: 25,
                },
                Person {
                    name: "Bob".to_string(),
                    age: 30,
                },
                Person {
                    name: "Charlie".to_string(),
                    age: 35,
                },
            ],
            5,
        )
        .unwrap();

        // Search by age
        let result = people.binary_search_by(|p| p.age.cmp(&30));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_insertion_point() {
        let arr = Array::from_slice(&[1, 3, 5, 7, 9], 10).unwrap();

        // Element that would go in the middle
        assert_eq!(arr.binary_search_insertion_point(&4), 2);

        // Element that would go at the beginning
        assert_eq!(arr.binary_search_insertion_point(&0), 0);

        // Element that would go at the end
        assert_eq!(arr.binary_search_insertion_point(&10), 5);

        // Existing element (returns first occurrence)
        assert_eq!(arr.binary_search_insertion_point(&5), 2);
    }

    #[test]
    fn test_dynamic_array_binary_search() {
        let mut arr = DynamicArray::new();
        for i in &[1, 3, 5, 7, 9, 11, 13] {
            arr.push(*i);
        }

        // Test same operations on dynamic array
        assert_eq!(arr.binary_search(&7), Some(3));
        assert_eq!(arr.binary_search(&4), None);
        assert_eq!(arr.binary_search_first(&5), Some(2));
        assert_eq!(arr.binary_search_insertion_point(&8), 4);
    }

    #[test]
    fn test_large_array() {
        let mut arr = DynamicArray::with_capacity(1000);
        for i in 0..1000 {
            arr.push(i * 2); // Even numbers 0, 2, 4, ..., 1998
        }

        // Test finding elements
        assert_eq!(arr.binary_search(&500), Some(250));
        assert_eq!(arr.binary_search(&1000), Some(500));
        assert_eq!(arr.binary_search(&1998), Some(999));

        // Test not finding odd numbers
        assert_eq!(arr.binary_search(&501), None);
        assert_eq!(arr.binary_search(&999), None);
    }

    #[test]
    fn test_string_search() {
        let words = Array::from_slice(
            &[
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string(),
                "date".to_string(),
                "elderberry".to_string(),
            ],
            10,
        )
        .unwrap();

        assert_eq!(words.binary_search(&"cherry".to_string()), Some(2));
        assert_eq!(words.binary_search(&"grape".to_string()), None);
    }

    #[test]
    fn test_edge_cases() {
        let arr = Array::from_slice(&[1, 2, 3], 5).unwrap();

        // Test boundary values
        assert_eq!(arr.binary_search(&1), Some(0));
        assert_eq!(arr.binary_search(&3), Some(2));

        // Test with same elements
        let same = Array::from_slice(&[5, 5, 5, 5], 5).unwrap();
        assert!(same.binary_search(&5).is_some());
        assert_eq!(same.binary_search_first(&5), Some(0));
        assert_eq!(same.binary_search_last(&5), Some(3));
    }

    #[test]
    fn test_is_sorted_utility() {
        let sorted = Array::from_slice(&[1, 2, 3, 4, 5], 5).unwrap();
        assert!(utils::is_sorted(&sorted));

        let unsorted = Array::from_slice(&[1, 3, 2, 4, 5], 5).unwrap();
        assert!(!utils::is_sorted(&unsorted));
    }
}
