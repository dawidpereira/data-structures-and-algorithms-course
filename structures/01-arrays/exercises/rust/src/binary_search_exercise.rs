//! Binary Search Exercise
//!
//! In this exercise, you'll implement binary search step by step.
//! Start with the basic version, then work on the variations.
//!
//! Binary search is a fundamental algorithm that every programmer should
//! understand deeply. It teaches important concepts like:
//! - Divide and conquer
//! - Loop invariants
//! - Edge case handling
//! - Integer overflow prevention

/// Exercise 1: Basic Binary Search
///
/// Implement binary search that finds an element in a sorted array.
/// Return the index if found, None if not found.
///
/// # Hints
/// - Use three pointers: low, mid, high
/// - Remember to handle the case when the array is empty
/// - Be careful with integer overflow when calculating mid
/// - Make sure your loop terminates (low <= high)
pub fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    // TODO: Handle empty slice case

    // TODO: Initialize low and high pointers

    // TODO: Main loop - while low <= high
    //   - Calculate mid (safely!)
    //   - Compare element at mid with target
    //   - Adjust low or high based on comparison
    //   - Return mid if found

    // TODO: Return None if not found
    todo!("Implement basic binary search")
}

/// Exercise 2: Binary Search with Custom Comparison
///
/// Sometimes you need to search using a different comparison than the
/// natural ordering. Implement binary search with a comparison function.
///
/// The comparator should return:
/// - Ordering::Less if the element is less than what you're looking for
/// - Ordering::Equal if it matches
/// - Ordering::Greater if the element is greater
///
/// # Example
/// ```ignore
/// // Search for a person by age in an array sorted by age
/// let people = vec![
///     Person { name: "Alice", age: 25 },
///     Person { name: "Bob", age: 30 },
/// ];
/// let index = binary_search_by(&people, |p| p.age.cmp(&30));
/// ```
pub fn binary_search_by<T, F>(slice: &[T], mut compare: F) -> Option<usize>
where
    F: FnMut(&T) -> std::cmp::Ordering,
{
    // TODO: Similar to basic binary search, but use the compare function
    // instead of direct comparison
    todo!("Implement binary search with custom comparison")
}

/// Exercise 3: Find First Occurrence
///
/// When the array contains duplicates, find the index of the FIRST
/// occurrence of the target element.
///
/// # Example
/// ```ignore
/// let arr = [1, 2, 2, 2, 3, 4];
/// assert_eq!(binary_search_first(&arr, &2), Some(1));
/// ```
///
/// # Hints
/// - When you find a match, don't return immediately
/// - Save the index and continue searching in the left half
/// - This ensures you find the leftmost occurrence
pub fn binary_search_first<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    // TODO: Initialize result variable to store found index

    // TODO: Implement binary search that continues searching left
    // after finding a match
    todo!("Implement binary search for first occurrence")
}

/// Exercise 4: Find Last Occurrence
///
/// Find the index of the LAST occurrence of the target element.
///
/// # Hints
/// - Similar to find_first, but search the right half after finding a match
pub fn binary_search_last<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    // TODO: Implement binary search for last occurrence
    todo!("Implement binary search for last occurrence")
}

/// Exercise 5: Find Insertion Point
///
/// Find where to insert an element to maintain sorted order.
/// If the element already exists, return the index of the first occurrence.
///
/// # Example
/// ```ignore
/// let arr = [1, 3, 5, 7];
/// assert_eq!(find_insertion_point(&arr, &4), 2); // Insert between 3 and 5
/// assert_eq!(find_insertion_point(&arr, &0), 0); // Insert at beginning
/// assert_eq!(find_insertion_point(&arr, &8), 4); // Insert at end
/// ```
///
/// # Hints
/// - This is slightly different from basic binary search
/// - You're looking for the first element that is >= target
/// - Be careful with the boundary conditions
pub fn find_insertion_point<T: Ord>(slice: &[T], target: &T) -> usize {
    // TODO: Implement insertion point search
    // Note: This should return usize, not Option<usize>
    todo!("Implement find insertion point")
}

/// Bonus Exercise: Search in Rotated Array
///
/// A sorted array has been rotated at some pivot unknown to you beforehand.
/// For example: [4, 5, 6, 7, 0, 1, 2] was originally [0, 1, 2, 4, 5, 6, 7]
///
/// Find the target element in O(log n) time.
///
/// # Hints
/// - First figure out which half is properly sorted
/// - Then determine which half to search based on the target value
/// - This is trickier than regular binary search!
pub fn search_rotated<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    // TODO: This is an advanced exercise!
    // Think about how to determine which half is sorted
    // and how to decide which half to search
    todo!("Implement search in rotated array (bonus)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_binary_search() {
        let arr = [1, 3, 5, 7, 9, 11, 13];

        // Test finding elements
        assert_eq!(binary_search(&arr, &7), Some(3));
        assert_eq!(binary_search(&arr, &1), Some(0));
        assert_eq!(binary_search(&arr, &13), Some(6));

        // Test not finding elements
        assert_eq!(binary_search(&arr, &0), None);
        assert_eq!(binary_search(&arr, &4), None);
        assert_eq!(binary_search(&arr, &14), None);

        // Test empty array
        let empty: [i32; 0] = [];
        assert_eq!(binary_search(&empty, &5), None);

        // Test single element
        let single = [42];
        assert_eq!(binary_search(&single, &42), Some(0));
        assert_eq!(binary_search(&single, &41), None);
    }

    #[test]
    fn test_binary_search_by() {
        #[derive(Debug)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let people = [
            Person {
                name: "Alice",
                age: 25,
            },
            Person {
                name: "Bob",
                age: 30,
            },
            Person {
                name: "Charlie",
                age: 35,
            },
            Person {
                name: "David",
                age: 40,
            },
        ];

        // Search by age
        assert_eq!(binary_search_by(&people, |p| p.age.cmp(&30)), Some(1));

        // Search for non-existent age
        assert_eq!(binary_search_by(&people, |p| p.age.cmp(&33)), None);
    }

    #[test]
    fn test_find_first_occurrence() {
        let arr = [1, 2, 2, 2, 3, 4, 5];

        assert_eq!(binary_search_first(&arr, &2), Some(1));
        assert_eq!(binary_search_first(&arr, &1), Some(0));
        assert_eq!(binary_search_first(&arr, &5), Some(6));
        assert_eq!(binary_search_first(&arr, &6), None);

        // Test with all same elements
        let same = [5, 5, 5, 5, 5];
        assert_eq!(binary_search_first(&same, &5), Some(0));
    }

    #[test]
    fn test_find_last_occurrence() {
        let arr = [1, 2, 2, 2, 3, 4, 5];

        assert_eq!(binary_search_last(&arr, &2), Some(3));
        assert_eq!(binary_search_last(&arr, &1), Some(0));
        assert_eq!(binary_search_last(&arr, &5), Some(6));
        assert_eq!(binary_search_last(&arr, &6), None);

        // Test with all same elements
        let same = [5, 5, 5, 5, 5];
        assert_eq!(binary_search_last(&same, &5), Some(4));
    }

    #[test]
    fn test_find_insertion_point() {
        let arr = [1, 3, 5, 7, 9];

        // Insert in middle
        assert_eq!(find_insertion_point(&arr, &4), 2);
        assert_eq!(find_insertion_point(&arr, &6), 3);

        // Insert at boundaries
        assert_eq!(find_insertion_point(&arr, &0), 0);
        assert_eq!(find_insertion_point(&arr, &10), 5);

        // Existing elements (should return first occurrence)
        assert_eq!(find_insertion_point(&arr, &5), 2);

        // With duplicates
        let dup = [1, 2, 2, 2, 3];
        assert_eq!(find_insertion_point(&dup, &2), 1);

        // Empty array
        let empty: [i32; 0] = [];
        assert_eq!(find_insertion_point(&empty, &5), 0);
    }

    #[test]
    #[ignore] // Remove this when you implement the bonus exercise
    fn test_search_rotated() {
        let arr = [4, 5, 6, 7, 0, 1, 2];

        // Test elements from both parts
        assert_eq!(search_rotated(&arr, &5), Some(1));
        assert_eq!(search_rotated(&arr, &1), Some(5));
        assert_eq!(search_rotated(&arr, &4), Some(0));
        assert_eq!(search_rotated(&arr, &2), Some(6));

        // Test not found
        assert_eq!(search_rotated(&arr, &3), None);
        assert_eq!(search_rotated(&arr, &8), None);

        // Test non-rotated array (rotation point at 0)
        let normal = [1, 2, 3, 4, 5];
        assert_eq!(search_rotated(&normal, &3), Some(2));
    }

    #[test]
    fn test_large_array_performance() {
        // This test verifies your solution works efficiently on large arrays
        let mut arr = Vec::new();
        for i in 0..10000 {
            arr.push(i * 2); // Even numbers 0, 2, 4, ..., 19998
        }

        // Should find these quickly (not iterate through thousands of elements)
        assert_eq!(binary_search(&arr, &10000), Some(5000));
        assert_eq!(binary_search(&arr, &19998), Some(9999));
        assert_eq!(binary_search(&arr, &0), Some(0));

        // Odd numbers shouldn't be found
        assert_eq!(binary_search(&arr, &10001), None);
    }
}

