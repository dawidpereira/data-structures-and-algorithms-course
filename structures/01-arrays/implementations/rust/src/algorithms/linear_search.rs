//! Linear search implementations for arrays.
//!
//! This module provides linear search functionality for both fixed-size
//! arrays (Array<T>) and dynamic arrays (DynamicArray<T>).
//!
//! For theory and visual explanations, see:
//! - Linear Search Theory: `algorithms/searching/linear-search/`
//! - Array-Specific Details: `structures/01-arrays/algorithms/linear-search.md`

use crate::core::Array;
use crate::dynamic_array::DynamicArray;

/// Trait for types that support linear search operations.
pub trait LinearSearchable<T> {
    /// Searches for `target` in the array from start to end.
    ///
    /// Returns the index of the first occurrence if found, or `None` if not found.
    /// Unlike binary search, this works on unsorted data.
    ///
    /// # Examples
    /// ```
    /// use arrays::core::Array;
    /// use arrays::algorithms::LinearSearchable;
    ///
    /// let arr = Array::from_slice(&[5, 2, 8, 1, 9], 10).unwrap();
    /// assert_eq!(arr.linear_search(&8), Some(2));
    /// assert_eq!(arr.linear_search(&3), None);
    /// ```
    fn linear_search(&self, target: &T) -> Option<usize>
    where
        T: PartialEq;

    /// Searches for an element that satisfies the given predicate.
    ///
    /// Returns the index of the first element for which the predicate returns true.
    ///
    /// # Examples
    /// ```
    /// use arrays::core::Array;
    /// use arrays::algorithms::LinearSearchable;
    ///
    /// let arr = Array::from_slice(&[1, 5, 3, 8, 2], 10).unwrap();
    /// // Find first element greater than 5
    /// assert_eq!(arr.linear_search_if(|&x| x > 5), Some(3));
    /// ```
    fn linear_search_if<F>(&self, predicate: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool;

    /// Finds all occurrences of the target in the array.
    ///
    /// Returns a vector of indices where the target appears.
    /// Empty vector if target not found.
    ///
    /// # Examples
    /// ```
    /// use arrays::core::Array;
    /// use arrays::algorithms::LinearSearchable;
    ///
    /// let arr = Array::from_slice(&[1, 2, 3, 2, 4, 2], 10).unwrap();
    /// assert_eq!(arr.linear_search_all(&2), vec![1, 3, 5]);
    /// ```
    fn linear_search_all(&self, target: &T) -> Vec<usize>
    where
        T: PartialEq;

    /// Searches for target starting from the end of the array.
    ///
    /// Returns the index of the last occurrence if found.
    ///
    /// # Examples
    /// ```
    /// use arrays::core::Array;
    /// use arrays::algorithms::LinearSearchable;
    ///
    /// let arr = Array::from_slice(&[1, 2, 3, 2, 4], 10).unwrap();
    /// assert_eq!(arr.reverse_linear_search(&2), Some(3));
    /// ```
    fn reverse_linear_search(&self, target: &T) -> Option<usize>
    where
        T: PartialEq;

    /// Checks if the array contains the target element.
    ///
    /// # Examples
    /// ```
    /// use arrays::core::Array;
    /// use arrays::algorithms::LinearSearchable;
    ///
    /// let arr = Array::from_slice(&[1, 2, 3, 4, 5], 10).unwrap();
    /// assert!(arr.contains(&3));
    /// assert!(!arr.contains(&6));
    /// ```
    fn contains(&self, target: &T) -> bool
    where
        T: PartialEq,
    {
        self.linear_search(target).is_some()
    }
}

// Helper macro to implement linear search for both array types
macro_rules! impl_linear_search {
    ($type:ty) => {
        impl<T> LinearSearchable<T> for $type {
            fn linear_search(&self, target: &T) -> Option<usize>
            where
                T: PartialEq,
            {
                for i in 0..self.len() {
                    if self.get(i)? == target {
                        return Some(i);
                    }
                }
                None
            }

            fn linear_search_if<F>(&self, mut predicate: F) -> Option<usize>
            where
                F: FnMut(&T) -> bool,
            {
                for i in 0..self.len() {
                    if predicate(self.get(i)?) {
                        return Some(i);
                    }
                }
                None
            }

            fn linear_search_all(&self, target: &T) -> Vec<usize>
            where
                T: PartialEq,
            {
                let mut indices = Vec::new();
                for i in 0..self.len() {
                    if let Some(elem) = self.get(i) {
                        if elem == target {
                            indices.push(i);
                        }
                    }
                }
                indices
            }

            fn reverse_linear_search(&self, target: &T) -> Option<usize>
            where
                T: PartialEq,
            {
                if self.is_empty() {
                    return None;
                }

                for i in (0..self.len()).rev() {
                    if self.get(i)? == target {
                        return Some(i);
                    }
                }
                None
            }
        }
    };
}

// Implement for both array types
impl_linear_search!(Array<T>);
impl_linear_search!(DynamicArray<T>);

/// Additional linear search utilities
pub mod utils {
    use super::*;

    /// Sentinel linear search - eliminates bounds checking
    /// For demonstration purposes only - requires unsafe manipulation
    pub fn sentinel_linear_search<T: PartialEq + Clone>(
        arr: &Array<T>,
        target: &T,
    ) -> Option<usize> {
        // Since we can't modify Array internals safely,
        // we'll demonstrate the concept with regular linear search
        // In a real implementation, you'd need access to raw array memory

        // Conceptual implementation:
        // 1. Place target at end of array (sentinel)
        // 2. Search without bounds checking
        // 3. Check if found before sentinel position

        // For now, use regular linear search
        arr.linear_search(target)
    }

    /// Count occurrences of target in array
    pub fn count_occurrences<T: PartialEq>(arr: &Array<T>, target: &T) -> usize {
        let mut count = 0;
        for i in 0..arr.len() {
            if let Some(elem) = arr.get(i) {
                if elem == target {
                    count += 1;
                }
            }
        }
        count
    }

    /// Find minimum element in array
    pub fn find_min<T: Ord>(arr: &Array<T>) -> Option<(usize, &T)> {
        if arr.is_empty() {
            return None;
        }

        let mut min_idx = 0;
        let mut min_val = arr.get(0)?;

        for i in 1..arr.len() {
            if let Some(elem) = arr.get(i) {
                if elem < min_val {
                    min_idx = i;
                    min_val = elem;
                }
            }
        }

        Some((min_idx, min_val))
    }

    /// Find maximum element in array
    pub fn find_max<T: Ord>(arr: &Array<T>) -> Option<(usize, &T)> {
        if arr.is_empty() {
            return None;
        }

        let mut max_idx = 0;
        let mut max_val = arr.get(0)?;

        for i in 1..arr.len() {
            if let Some(elem) = arr.get(i) {
                if elem > max_val {
                    max_idx = i;
                    max_val = elem;
                }
            }
        }

        Some((max_idx, max_val))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search_basic() {
        let arr = Array::from_slice(&[3, 1, 4, 1, 5, 9, 2, 6], 10).unwrap();

        // Find existing elements
        assert_eq!(arr.linear_search(&4), Some(2));
        assert_eq!(arr.linear_search(&9), Some(5));
        assert_eq!(arr.linear_search(&3), Some(0));
        assert_eq!(arr.linear_search(&6), Some(7));

        // Not found
        assert_eq!(arr.linear_search(&7), None);
        assert_eq!(arr.linear_search(&0), None);
    }

    #[test]
    fn test_linear_search_empty() {
        let arr: Array<i32> = Array::new(5);
        assert_eq!(arr.linear_search(&42), None);
    }

    #[test]
    fn test_linear_search_duplicates() {
        let arr = Array::from_slice(&[1, 2, 3, 2, 4, 2, 5], 10).unwrap();

        // Returns first occurrence
        assert_eq!(arr.linear_search(&2), Some(1));

        // Find all occurrences
        assert_eq!(arr.linear_search_all(&2), vec![1, 3, 5]);
        assert_eq!(arr.linear_search_all(&1), vec![0]);
        assert_eq!(arr.linear_search_all(&6), vec![]);
    }

    #[test]
    fn test_linear_search_if() {
        let arr = Array::from_slice(&[1, 3, 5, 7, 9, 11], 10).unwrap();

        // Find first even number (none exist)
        assert_eq!(arr.linear_search_if(|&x| x % 2 == 0), None);

        // Find first number greater than 5
        assert_eq!(arr.linear_search_if(|&x| x > 5), Some(3));

        // Find first number divisible by 3
        assert_eq!(arr.linear_search_if(|&x| x % 3 == 0), Some(1));
    }

    #[test]
    fn test_reverse_linear_search() {
        let arr = Array::from_slice(&[1, 2, 3, 2, 4, 2, 5], 10).unwrap();

        // Returns last occurrence
        assert_eq!(arr.reverse_linear_search(&2), Some(5));
        assert_eq!(arr.reverse_linear_search(&1), Some(0));
        assert_eq!(arr.reverse_linear_search(&5), Some(6));
        assert_eq!(arr.reverse_linear_search(&6), None);
    }

    #[test]
    fn test_contains() {
        let arr = Array::from_slice(&[10, 20, 30, 40, 50], 5).unwrap();

        assert!(arr.contains(&30));
        assert!(arr.contains(&10));
        assert!(arr.contains(&50));
        assert!(!arr.contains(&25));
        assert!(!arr.contains(&60));
    }

    #[test]
    fn test_dynamic_array_linear_search() {
        let mut arr = DynamicArray::new();
        for i in &[5, 2, 8, 1, 9, 3, 7] {
            arr.push(*i);
        }

        assert_eq!(arr.linear_search(&8), Some(2));
        assert_eq!(arr.linear_search(&1), Some(3));
        assert_eq!(arr.linear_search(&10), None);

        assert!(arr.contains(&7));
        assert!(!arr.contains(&0));
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

        assert_eq!(words.linear_search(&"cherry".to_string()), Some(2));
        assert_eq!(words.linear_search(&"grape".to_string()), None);

        // Search with predicate
        assert_eq!(words.linear_search_if(|s| s.starts_with('b')), Some(1));
    }

    #[test]
    fn test_find_min_max() {
        let arr = Array::from_slice(&[3, 1, 4, 1, 5, 9, 2, 6], 10).unwrap();

        let (min_idx, min_val) = utils::find_min(&arr).unwrap();
        assert_eq!(min_idx, 1);
        assert_eq!(*min_val, 1);

        let (max_idx, max_val) = utils::find_max(&arr).unwrap();
        assert_eq!(max_idx, 5);
        assert_eq!(*max_val, 9);
    }

    #[test]
    fn test_count_occurrences() {
        let arr = Array::from_slice(&[1, 2, 3, 2, 4, 2, 5, 2], 10).unwrap();

        assert_eq!(utils::count_occurrences(&arr, &2), 4);
        assert_eq!(utils::count_occurrences(&arr, &1), 1);
        assert_eq!(utils::count_occurrences(&arr, &6), 0);
    }

    #[test]
    fn test_sentinel_search() {
        let arr = Array::<i32>::from_slice(&[3, 1, 4, 1, 5], 10).unwrap();

        assert_eq!(utils::sentinel_linear_search(&arr, &4), Some(2));
        assert_eq!(utils::sentinel_linear_search(&arr, &5), Some(4));
        assert_eq!(utils::sentinel_linear_search(&arr, &6), None);
    }

    #[test]
    fn test_performance_comparison() {
        // Create a large array for performance testing
        let mut arr = DynamicArray::with_capacity(10000);
        for i in 0..10000 {
            arr.push(i);
        }

        // Linear search finds element at beginning quickly
        assert_eq!(arr.linear_search(&5), Some(5));

        // Linear search takes longer for elements at end
        assert_eq!(arr.linear_search(&9999), Some(9999));

        // Element not in array
        assert_eq!(arr.linear_search(&10000), None);
    }
}
