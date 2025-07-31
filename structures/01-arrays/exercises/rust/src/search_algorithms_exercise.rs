//! Search Algorithm Exercises
//!
//! Practice implementing various search algorithms from scratch.
//! Complete each exercise by filling in the TODO sections.

use std::cmp::Ordering;

/// Exercise 1: Linear Search Variants
///
/// Implement different variations of linear search.
pub mod exercise1 {
    /// Basic linear search - find first occurrence
    ///
    /// TODO: Implement linear search that returns the index of the first
    /// occurrence of the target, or None if not found.
    pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
        // TODO: Implement basic linear search
        // Hint: Iterate through array, compare each element with target
        unimplemented!("Implement linear search")
    }

    /// Find all occurrences of target
    ///
    /// TODO: Return a vector of all indices where target appears
    pub fn linear_search_all<T: PartialEq>(arr: &[T], target: &T) -> Vec<usize> {
        // TODO: Collect all indices where arr[i] == target
        unimplemented!("Implement linear search for all occurrences")
    }

    /// Search with a predicate function
    ///
    /// TODO: Find first element that satisfies the predicate
    pub fn linear_search_if<T, F>(arr: &[T], predicate: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        // TODO: Return index of first element where predicate returns true
        unimplemented!("Implement predicate-based linear search")
    }

    /// Reverse linear search - search from end
    ///
    /// TODO: Search from the end of the array backwards
    pub fn reverse_linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
        // TODO: Start from arr.len() - 1 and work backwards
        unimplemented!("Implement reverse linear search")
    }
}

/// Exercise 2: Jump Search Implementation
///
/// Implement jump search for sorted arrays.
pub mod exercise2 {
    use std::cmp::min;

    /// Calculate optimal jump size for array of length n
    ///
    /// TODO: Return the optimal jump size (hint: it's related to square root)
    pub fn calculate_jump_size(n: usize) -> usize {
        // TODO: Calculate and return optimal jump size
        unimplemented!("Calculate optimal jump size")
    }

    /// Basic jump search implementation
    ///
    /// TODO: Implement jump search algorithm
    /// Remember: Array must be sorted!
    pub fn jump_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
        let n = arr.len();
        if n == 0 {
            return None;
        }

        let jump = calculate_jump_size(n);

        // TODO: Implement jump search algorithm
        // 1. Jump forward by 'jump' steps until you find a value >= target
        // 2. Go back one jump
        // 3. Linear search in that block

        unimplemented!("Implement jump search")
    }

    /// Jump search with custom jump size
    ///
    /// TODO: Implement jump search with user-provided jump size
    pub fn jump_search_custom<T: Ord>(arr: &[T], target: &T, jump_size: usize) -> Option<usize> {
        // TODO: Similar to above but use provided jump_size
        unimplemented!("Implement jump search with custom size")
    }
}

/// Exercise 3: Interpolation Search (Bonus/Advanced)
///
/// Implement interpolation search for numeric data.
pub mod exercise3 {
    /// Interpolation search for i32 arrays
    ///
    /// TODO: Implement interpolation search
    /// This is more complex - uses value to estimate position
    pub fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
        let n = arr.len();
        if n == 0 {
            return None;
        }

        let mut low = 0;
        let mut high = n - 1;

        while low <= high && target >= arr[low] && target <= arr[high] {
            // TODO: Implement interpolation search
            // 1. Calculate interpolated position using formula:
            //    pos = low + [(target - arr[low]) / (arr[high] - arr[low])] * (high - low)
            // 2. Check if arr[pos] == target
            // 3. Adjust low or high based on comparison

            unimplemented!("Implement interpolation search")
        }

        None
    }
}

/// Exercise 4: Performance Comparison
///
/// Compare the performance of different search algorithms.
pub mod exercise4 {
    use super::exercise1::linear_search;
    use super::exercise2::jump_search;

    /// Count comparisons made by each algorithm
    ///
    /// TODO: Modify the search algorithms to count comparisons
    /// and return both the result and comparison count
    pub struct SearchResult {
        pub index: Option<usize>,
        pub comparisons: usize,
    }

    /// Linear search with comparison counting
    pub fn linear_search_counted<T: PartialEq>(arr: &[T], target: &T) -> SearchResult {
        // TODO: Implement linear search that counts comparisons
        unimplemented!("Implement linear search with counting")
    }

    /// Jump search with comparison counting
    pub fn jump_search_counted<T: Ord>(arr: &[T], target: &T) -> SearchResult {
        // TODO: Implement jump search that counts comparisons
        unimplemented!("Implement jump search with counting")
    }

    /// Compare algorithms on the same data
    ///
    /// TODO: Run both algorithms and return comparison counts
    pub fn compare_algorithms(arr: &[i32], target: i32) -> (usize, usize) {
        // TODO: Run both algorithms and return (linear_comparisons, jump_comparisons)
        unimplemented!("Compare algorithm performance")
    }
}

/// Exercise 5: Practical Applications
///
/// Apply search algorithms to real-world scenarios.
pub mod exercise5 {
    #[derive(Debug, PartialEq, Clone)]
    pub struct Student {
        pub id: u32,
        pub name: String,
        pub grade: f32,
    }

    /// Find student by ID using appropriate search
    ///
    /// TODO: Students are sorted by ID - choose best algorithm
    pub fn find_student_by_id(students: &[Student], id: u32) -> Option<usize> {
        // TODO: Implement search for student by ID
        // Hint: What search works best for sorted numeric IDs?
        unimplemented!("Find student by ID")
    }

    /// Find all students with specific grade
    ///
    /// TODO: Find all students with the given grade
    pub fn find_students_by_grade(students: &[Student], grade: f32) -> Vec<usize> {
        // TODO: What search algorithm is appropriate here?
        unimplemented!("Find students by grade")
    }

    /// Find first student with grade above threshold
    ///
    /// TODO: Find first student with grade > threshold
    pub fn find_first_above_grade(students: &[Student], threshold: f32) -> Option<usize> {
        // TODO: Use appropriate search variant
        unimplemented!("Find first student above grade threshold")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search_basic() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];

        assert_eq!(exercise1::linear_search(&arr, &4), Some(2));
        assert_eq!(exercise1::linear_search(&arr, &7), None);
        assert_eq!(exercise1::linear_search(&arr, &1), Some(1)); // First occurrence
    }

    #[test]
    fn test_linear_search_all() {
        let arr = vec![1, 2, 3, 2, 4, 2, 5];

        assert_eq!(exercise1::linear_search_all(&arr, &2), vec![1, 3, 5]);
        assert_eq!(exercise1::linear_search_all(&arr, &6), vec![]);
        assert_eq!(exercise1::linear_search_all(&arr, &1), vec![0]);
    }

    #[test]
    fn test_linear_search_if() {
        let arr = vec![1, 3, 5, 7, 9];

        // Find first even number (none exist)
        assert_eq!(exercise1::linear_search_if(&arr, |&x| x % 2 == 0), None);

        // Find first number greater than 5
        assert_eq!(exercise1::linear_search_if(&arr, |&x| x > 5), Some(3));
    }

    #[test]
    fn test_reverse_linear_search() {
        let arr = vec![1, 2, 3, 2, 4];

        assert_eq!(exercise1::reverse_linear_search(&arr, &2), Some(3)); // Last occurrence
        assert_eq!(exercise1::reverse_linear_search(&arr, &5), None);
    }

    #[test]
    fn test_jump_size_calculation() {
        assert_eq!(exercise2::calculate_jump_size(1), 1);
        assert_eq!(exercise2::calculate_jump_size(4), 2);
        assert_eq!(exercise2::calculate_jump_size(9), 3);
        assert_eq!(exercise2::calculate_jump_size(16), 4);
        assert_eq!(exercise2::calculate_jump_size(100), 10);
    }

    #[test]
    fn test_jump_search() {
        let arr = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];

        assert_eq!(exercise2::jump_search(&arr, &7), Some(3));
        assert_eq!(exercise2::jump_search(&arr, &1), Some(0));
        assert_eq!(exercise2::jump_search(&arr, &19), Some(9));
        assert_eq!(exercise2::jump_search(&arr, &8), None);
    }

    #[test]
    fn test_jump_search_custom_size() {
        let arr = vec![1, 3, 5, 7, 9, 11, 13];

        // With jump size 2
        assert_eq!(exercise2::jump_search_custom(&arr, &5, 2), Some(2));

        // With jump size 1 (degenerates to linear search)
        assert_eq!(exercise2::jump_search_custom(&arr, &11, 1), Some(5));
    }

    #[test]
    fn test_interpolation_search() {
        let arr = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

        assert_eq!(exercise3::interpolation_search(&arr, 70), Some(6));
        assert_eq!(exercise3::interpolation_search(&arr, 10), Some(0));
        assert_eq!(exercise3::interpolation_search(&arr, 100), Some(9));
        assert_eq!(exercise3::interpolation_search(&arr, 55), None);
    }

    #[test]
    fn test_performance_comparison() {
        let arr: Vec<i32> = (0..100).collect();

        let linear_result = exercise4::linear_search_counted(&arr, &75);
        let jump_result = exercise4::jump_search_counted(&arr, &75);

        assert_eq!(linear_result.index, Some(75));
        assert_eq!(jump_result.index, Some(75));

        // Jump search should use fewer comparisons
        assert!(jump_result.comparisons < linear_result.comparisons);

        let (linear_comps, jump_comps) = exercise4::compare_algorithms(&arr, 75);
        assert!(jump_comps < linear_comps);
    }

    #[test]
    fn test_student_search() {
        let students = vec![
            exercise5::Student {
                id: 1001,
                name: "Alice".to_string(),
                grade: 85.5,
            },
            exercise5::Student {
                id: 1002,
                name: "Bob".to_string(),
                grade: 92.0,
            },
            exercise5::Student {
                id: 1003,
                name: "Charlie".to_string(),
                grade: 78.5,
            },
            exercise5::Student {
                id: 1004,
                name: "Diana".to_string(),
                grade: 92.0,
            },
            exercise5::Student {
                id: 1005,
                name: "Eve".to_string(),
                grade: 88.0,
            },
        ];

        // Find by ID (sorted)
        assert_eq!(exercise5::find_student_by_id(&students, 1003), Some(2));
        assert_eq!(exercise5::find_student_by_id(&students, 1006), None);

        // Find all with specific grade
        assert_eq!(
            exercise5::find_students_by_grade(&students, 92.0),
            vec![1, 3]
        );

        // Find first above threshold
        assert_eq!(exercise5::find_first_above_grade(&students, 90.0), Some(1));
        assert_eq!(exercise5::find_first_above_grade(&students, 95.0), None);
    }

    #[test]
    fn test_edge_cases() {
        let empty: Vec<i32> = vec![];
        assert_eq!(exercise1::linear_search(&empty, &5), None);
        assert_eq!(exercise2::jump_search(&empty, &5), None);

        let single = vec![42];
        assert_eq!(exercise1::linear_search(&single, &42), Some(0));
        assert_eq!(exercise2::jump_search(&single, &42), Some(0));
    }

    #[test]
    fn test_large_array_performance() {
        let large_arr: Vec<i32> = (0..10000).collect();

        // Test that jump search is more efficient
        let (linear, jump) = exercise4::compare_algorithms(&large_arr, 7500);

        // Linear should take ~7500 comparisons
        assert!(linear > 7000);

        // Jump should take ~100 comparisons (√10000)
        assert!(jump < 200);
    }
}
