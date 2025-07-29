//! Exercise: Implement a fixed-size array from scratch
//!
//! Your goal is to implement all methods marked with TODO.
//! All tests are provided and should pass when your implementation is correct.
//!
//! ## Learning Objectives
//! - Understand manual memory management
//! - Work with raw pointers safely
//! - Implement proper cleanup with Drop
//! - Handle bounds checking and error cases
//!
//! ## Resources
//! - Implementation Guide: See docs/fixed-array-guide.md in this directory
//! - Memory concepts: See implementations/rust/docs/memory-management.md

use std::alloc::{alloc, dealloc, Layout};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Index;

/// A fixed-size array implementation using raw memory allocation
pub struct Array<T> {
    // TODO: Add the necessary fields
    // HINT: You need to track:
    // - A pointer to your allocated memory
    // - The capacity (fixed size)
    // - The current number of elements
    // - PhantomData to indicate ownership of T
    _todo: PhantomData<T>, // Remove this when you add real fields
}

impl<T> Array<T> {
    /// Creates a new array with the specified capacity
    ///
    /// # Requirements
    /// - Allocate memory for `capacity` elements of type T
    /// - Panic if capacity is 0
    /// - Panic if allocation fails (null pointer)
    /// - Initialize len to 0
    ///
    /// # Hints
    /// - Use `Layout::array::<T>(capacity)` to calculate memory layout
    /// - Use `unsafe { alloc(layout) as *mut T }` to allocate
    /// - Check if the returned pointer is null
    /// - Remember to initialize all struct fields
    pub fn new(capacity: usize) -> Self {
        todo!("Implement array creation with fixed capacity")
    }

    /// Returns the number of elements currently in the array
    pub fn len(&self) -> usize {
        todo!("Return the current length")
    }

    /// Returns true if the array contains no elements
    pub fn is_empty(&self) -> bool {
        todo!("Check if array is empty")
    }

    /// Returns the capacity of the array
    pub fn capacity(&self) -> usize {
        todo!("Return the fixed capacity")
    }

    /// Adds an element to the end of the array
    ///
    /// # Requirements
    /// - Return `Err(value)` if array is full
    /// - Write the value to the correct position
    /// - Increment the length
    ///
    /// # Hints
    /// - Check if `self.len >= self.capacity`
    /// - Use `self.ptr.add(self.len)` to calculate position
    /// - Use `ptr.write(value)` to store the value
    /// - Don't forget the unsafe block
    pub fn push(&mut self, value: T) -> Result<(), T> {
        todo!("Implement push operation")
    }

    /// Removes and returns the last element
    ///
    /// # Requirements
    /// - Return None if array is empty
    /// - Decrement length before reading
    /// - Use `ptr.read()` to move the value out
    ///
    /// # Hints
    /// - Check if `self.len == 0`
    /// - Calculate position with `self.ptr.add(self.len)`
    /// - Remember: read() moves the value, preventing double-drop
    pub fn pop(&mut self) -> Option<T> {
        todo!("Implement pop operation")
    }

    /// Gets a reference to the element at the given index
    ///
    /// # Requirements
    /// - Return None if index >= len
    /// - Convert raw pointer to safe reference
    ///
    /// # Hints
    /// - Bounds check: `index >= self.len`
    /// - Use `&*self.ptr.add(index)` to get reference
    /// - Wrap in Some()
    pub fn get(&self, index: usize) -> Option<&T> {
        todo!("Implement get operation")
    }

    /// Gets a mutable reference to the element at the given index
    ///
    /// # Requirements
    /// - Same as get() but return &mut T
    ///
    /// # Hints
    /// - Use `&mut *self.ptr.add(index)`
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        todo!("Implement get_mut operation")
    }

    /// Sets the value at the given index
    ///
    /// # Requirements
    /// - Panic if index >= len (use assert!)
    /// - Drop the old value before writing new one
    ///
    /// # Hints
    /// - Use assert! for bounds check with custom message
    /// - Use `ptr.read()` to drop old value
    /// - Then `ptr.write(value)` for new value
    pub fn set(&mut self, index: usize, value: T) {
        todo!("Implement set operation")
    }

    /// Creates an array from a slice
    ///
    /// # Requirements
    /// - Return None if slice.len() > capacity
    /// - Clone each element from the slice
    /// - Set len correctly
    ///
    /// # Hints
    /// - Create array with Array::new(capacity)
    /// - Use a loop to push each cloned element
    /// - Return None if any push fails
    pub fn from_slice(slice: &[T], capacity: usize) -> Option<Self>
    where
        T: Clone,
    {
        todo!("Implement from_slice")
    }

    /// Clears the array, removing all elements
    ///
    /// # Requirements
    /// - Drop all elements but keep allocated memory
    /// - Set len to 0
    ///
    /// # Hints
    /// - Use pop() in a loop
    /// - Could also iterate and use ptr::drop_in_place
    pub fn clear(&mut self) {
        todo!("Implement clear operation")
    }
}

// TODO: Implement Drop trait
// CRITICAL: This prevents memory leaks!
//
// Requirements:
// 1. Call clear() to drop all elements
// 2. Deallocate the memory if capacity > 0
//
// Hints:
// - clear() handles dropping elements
// - Use Layout::array::<T>(self.capacity) for deallocation
// - Cast ptr to *mut u8 for dealloc
// - Only deallocate if capacity > 0

// impl<T> Drop for Array<T> {
//     fn drop(&mut self) {
//         todo!("Implement drop")
//     }
// }

// TODO: Implement Send and Sync traits
// These make the array thread-safe when T is thread-safe
//
// Hints:
// - Array<T> should be Send when T is Send
// - Array<T> should be Sync when T is Sync
// - Use `unsafe impl`

// TODO: Implement Debug trait
// This allows printing the array with {:?}
//
// Requirements:
// - Print in format: [elem1, elem2, elem3]
// - Only print initialized elements (0..len)
//
// Hints:
// - Use write!(f, "[")? for opening bracket
// - Loop through 0..self.len
// - Add ", " between elements (not after last)
// - Access elements with unsafe { &*self.ptr.add(i) }

// TODO: Implement Index trait
// This allows array[index] syntax
//
// Requirements:
// - Panic if index is out of bounds
// - Return reference to element
//
// Hints:
// - type Output = T;
// - Use self.get(index).expect("Index out of bounds")

// TODO: Implement Debug trait for Array<T>
// This is needed for the tests that use format!("{:?}", arr)
impl<T: fmt::Debug> fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // HINT: Format as [elem1, elem2, elem3]
        // HINT: Use write!(f, "[")? to start
        // HINT: Loop through elements and format them
        // HINT: Add commas between elements
        // HINT: End with write!(f, "]")
        todo!("Implement Debug formatting")
    }
}

// TODO: Implement Index trait for Array<T>
// This allows arr[index] syntax
impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        // HINT: Use the get() method and handle the Option
        // HINT: Panic with a helpful message if index is out of bounds
        todo!("Implement indexing")
    }
}

// ============================================================================
// TESTS - DO NOT MODIFY
// These tests verify your implementation is correct
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array() {
        let arr: Array<i32> = Array::new(10);
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());
        assert_eq!(arr.capacity(), 10);
    }

    #[test]
    fn test_push_and_pop() {
        let mut arr: Array<i32> = Array::new(3);

        // Test push
        assert!(arr.push(1).is_ok());
        assert!(arr.push(2).is_ok());
        assert!(arr.push(3).is_ok());
        assert_eq!(arr.len(), 3);

        // Array should be full
        assert!(arr.push(4).is_err());

        // Test pop
        assert_eq!(arr.pop(), Some(3));
        assert_eq!(arr.pop(), Some(2));
        assert_eq!(arr.pop(), Some(1));
        assert_eq!(arr.pop(), None);
    }

    #[test]
    fn test_get_and_set() {
        let mut arr = Array::from_slice(&[10, 20, 30], 5).unwrap();

        // Test get
        assert_eq!(arr.get(0), Some(&10));
        assert_eq!(arr.get(2), Some(&30));
        assert_eq!(arr.get(3), None);

        // Test set
        arr.set(1, 25);
        assert_eq!(arr.get(1), Some(&25));
    }

    #[test]
    #[should_panic(expected = "capacity must be greater than 0")]
    fn test_zero_capacity() {
        let _arr: Array<i32> = Array::new(0);
    }

    #[test]
    fn test_drop() {
        // This test ensures our Drop implementation works
        {
            let mut arr = Array::new(100);
            for i in 0..50 {
                arr.push(i).unwrap();
            }
        } // arr is dropped here, should not leak memory

        // With heap-allocated data
        {
            let mut arr: Array<String> = Array::new(10);
            arr.push(String::from("Hello")).unwrap();
            arr.push(String::from("World")).unwrap();
        } // Strings should be properly dropped
    }

    #[test]
    fn test_clear() {
        let mut arr = Array::<i32>::from_slice(&[1, 2, 3], 5).unwrap();
        assert_eq!(arr.len(), 3);

        arr.clear();
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());

        // Should be able to push again after clear
        assert!(arr.push(10).is_ok());
        assert_eq!(arr.get(0), Some(&10));
    }

    #[test]
    fn test_get_mut() {
        let mut arr = Array::<i32>::from_slice(&[10, 20, 30], 5).unwrap();

        // Modify through get_mut
        if let Some(elem) = arr.get_mut(1) {
            *elem = 25;
        }
        assert_eq!(arr.get(1), Some(&25));

        // Out of bounds
        assert!(arr.get_mut(5).is_none());
    }

    #[test]
    fn test_index_trait() {
        let arr = Array::<i32>::from_slice(&[10, 20, 30], 5).unwrap();

        // Test index access
        assert_eq!(arr[0], 10);
        assert_eq!(arr[1], 20);
        assert_eq!(arr[2], 30);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_index_panic() {
        let arr = Array::<i32>::from_slice(&[10, 20], 5).unwrap();
        let _ = arr[5]; // Should panic
    }

    #[test]
    fn test_debug_formatting() {
        let arr = Array::<i32>::from_slice(&[1, 2, 3], 5).unwrap();
        let debug_str = format!("{:?}", arr);
        assert_eq!(debug_str, "[1, 2, 3]");

        // Empty array
        let empty: Array<i32> = Array::new(5);
        assert_eq!(format!("{:?}", empty), "[]");
    }

    #[test]
    fn test_with_strings() {
        let mut arr: Array<String> = Array::new(3);

        // Push strings
        assert!(arr.push(String::from("Hello")).is_ok());
        assert!(arr.push(String::from("World")).is_ok());

        // Test access
        assert_eq!(arr.get(0), Some(&String::from("Hello")));
        assert_eq!(arr.get(1), Some(&String::from("World")));

        // Test pop (ensures proper drop)
        assert_eq!(arr.pop(), Some(String::from("World")));
        assert_eq!(arr.len(), 1);
    }

    #[test]
    fn test_from_slice_capacity_exact() {
        // Exact capacity match
        let mut arr = Array::<i32>::from_slice(&[1, 2, 3], 3).unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.capacity(), 3);

        // Should be full
        assert!(arr.push(4).is_err());
    }

    #[test]
    fn test_from_slice_too_large() {
        // Slice larger than capacity
        let result = Array::<i32>::from_slice(&[1, 2, 3, 4, 5], 3);
        assert!(result.is_none());
    }

    #[test]
    fn test_multiple_push_pop_cycles() {
        let mut arr: Array<i32> = Array::new(3);

        // First cycle
        arr.push(1).unwrap();
        arr.push(2).unwrap();
        assert_eq!(arr.pop(), Some(2));
        assert_eq!(arr.pop(), Some(1));

        // Second cycle - ensure array is reusable
        arr.push(10).unwrap();
        arr.push(20).unwrap();
        arr.push(30).unwrap();
        assert_eq!(arr.len(), 3);
        assert!(arr.push(40).is_err()); // Still respects capacity
    }

    #[test]
    fn test_edge_cases() {
        // Single element array
        let mut single: Array<i32> = Array::new(1);
        assert!(single.push(42).is_ok());
        assert!(single.push(43).is_err());
        assert_eq!(single[0], 42);

        // Large capacity
        let mut large: Array<u8> = Array::new(1000);
        for i in 0..1000 {
            assert!(large.push(i as u8).is_ok());
        }
        assert_eq!(large.len(), 1000);
        assert!(large.push(0).is_err());
    }

    #[test]
    fn test_set_with_drop_types() {
        let mut arr = Array::<String>::from_slice(
            &[String::from("A"), String::from("B"), String::from("C")],
            5,
        )
        .unwrap();

        // Set should drop old value
        arr.set(1, String::from("NEW"));
        assert_eq!(arr.get(1), Some(&String::from("NEW")));
    }

    #[test]
    #[should_panic(expected = "Index 5 out of bounds for length 3")]
    fn test_set_out_of_bounds() {
        let mut arr = Array::<i32>::from_slice(&[1, 2, 3], 5).unwrap();
        arr.set(5, 42); // Should panic
    }

    // Custom type for testing
    #[derive(Debug, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_custom_types() {
        let mut arr: Array<Point> = Array::new(3);

        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };

        assert!(arr.push(p1.clone()).is_ok());
        assert!(arr.push(p2.clone()).is_ok());

        assert_eq!(arr.get(0), Some(&p1));
        assert_eq!(arr.get(1), Some(&p2));

        // Test debug formatting with custom type
        let debug_str = format!("{:?}", arr);
        assert!(debug_str.contains("Point"));
    }
}
