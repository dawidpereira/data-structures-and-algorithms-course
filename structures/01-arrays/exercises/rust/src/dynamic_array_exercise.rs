//! Exercise: Implement a dynamic array that grows automatically
//!
//! Your goal is to implement a growable array that automatically resizes.
//! This builds upon the fixed array concepts with dynamic memory management.
//!
//! ## Learning Objectives
//! - Implement growth strategies
//! - Use realloc for efficient resizing
//! - Implement smart shrinking to avoid thrashing
//! - Add iterator support
//!
//! ## Resources
//! - Implementation Guide: See docs/dynamic-array-guide.md in this directory
//! - Growth strategies: See implementations/rust/docs/dynamic-array-strategies.md

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Index;
use std::ptr;

/// A growable array implementation using dynamic memory allocation
pub struct DynamicArray<T> {
    // TODO: Add fields (similar to fixed array but capacity changes)
    // HINT: Same fields as fixed array:
    // - ptr: *mut T
    // - capacity: usize (will change!)
    // - len: usize
    // - _marker: PhantomData<T>
    _todo: PhantomData<T>, // Remove this
}

// TODO: Implement Default trait
// This allows DynamicArray::default() to work
// HINT: Just call Self::new()

impl<T> DynamicArray<T> {
    /// Creates a new empty dynamic array with no initial allocation
    ///
    /// # Requirements
    /// - Start with null pointer
    /// - Zero capacity and length
    /// - Delay allocation until first push
    pub fn new() -> Self {
        todo!("Create empty array with no allocation")
    }

    /// Creates a new dynamic array with the specified initial capacity
    ///
    /// # Requirements
    /// - If capacity is 0, return Self::new()
    /// - Otherwise allocate memory like fixed array
    ///
    /// # Hints
    /// - Reuse logic from fixed array new()
    pub fn with_capacity(capacity: usize) -> Self {
        todo!("Create array with initial capacity")
    }

    /// Returns the number of elements currently in the array
    pub fn len(&self) -> usize {
        todo!("Return current length")
    }

    /// Returns true if the array contains no elements
    pub fn is_empty(&self) -> bool {
        todo!("Check if empty")
    }

    /// Returns the current capacity of the array
    pub fn capacity(&self) -> usize {
        todo!("Return current capacity")
    }

    /// Adds an element to the end (always succeeds by growing if needed)
    ///
    /// # Requirements
    /// - Check if array is full (len == capacity)
    /// - If full, call grow()
    /// - Write value and increment length
    ///
    /// # Key Difference
    /// Unlike fixed array, this NEVER returns an error!
    pub fn push(&mut self, value: T) {
        todo!("Implement push with automatic growth")
    }

    /// Grows the array to accommodate more elements
    ///
    /// # Requirements
    /// - If capacity is 0, grow to 1
    /// - Otherwise, double the capacity
    /// - Use checked_mul to prevent overflow
    /// - Handle two cases: first allocation vs reallocation
    ///
    /// # Hints
    /// - For first allocation (capacity == 0): use alloc()
    /// - For reallocation: use realloc()
    /// - Update self.ptr after reallocation!
    /// - Cast pointers to *mut u8 for realloc
    fn grow(&mut self) {
        todo!("Implement growth strategy")
    }

    /// Removes and returns the last element
    ///
    /// # Requirements
    /// - Same as fixed array
    pub fn pop(&mut self) -> Option<T> {
        todo!("Implement pop")
    }

    /// Gets a reference to the element at index
    pub fn get(&self, index: usize) -> Option<&T> {
        todo!("Implement get")
    }

    /// Gets a mutable reference to the element at index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        todo!("Implement get_mut")
    }

    /// Clears the array, removing all elements
    pub fn clear(&mut self) {
        todo!("Clear all elements")
    }

    /// Shrinks the capacity to reduce memory usage
    ///
    /// # Requirements
    /// - Only shrink if len < capacity/4 AND capacity > 4
    /// - Shrink to capacity/2 (not to len!)
    /// - Special case: if empty, deallocate completely
    ///
    /// # Hints
    /// - This prevents thrashing (rapid grow/shrink cycles)
    /// - Use realloc for shrinking
    /// - Set ptr to null_mut() when deallocating completely
    pub fn shrink_to_fit(&mut self) {
        todo!("Implement smart shrinking")
    }
}

// TODO: Implement Drop trait
// Similar to fixed array but remember capacity can be 0 with null ptr

// TODO: Implement Send and Sync traits

// TODO: Implement Debug trait
impl<T: fmt::Debug> fmt::Debug for DynamicArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // HINT: Same as fixed array - format as [elem1, elem2, elem3]
        todo!("Implement Debug formatting")
    }
}

// TODO: Implement Index trait
impl<T> Index<usize> for DynamicArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        // HINT: Same as fixed array - use get() and panic on None
        todo!("Implement indexing")
    }
}

// TODO: Iterator support - this is more complex!

/// Iterator for consuming a DynamicArray
pub struct DynamicArrayIter<T> {
    // TODO: Add fields to track iteration state
    // HINT: You need:
    // - ptr, capacity, len (from the array)
    // - index (current position)
    // - _marker
    _todo: PhantomData<T>, // Remove this
}

// TODO: Implement Iterator trait for DynamicArrayIter
impl<T> Iterator for DynamicArrayIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Return next element")
    }
}

// TODO: Implement IntoIterator for DynamicArray
// This is tricky! You need to:
// 1. Move array fields to iterator
// 2. Use std::mem::forget(self) to prevent double-free
impl<T> IntoIterator for DynamicArray<T> {
    type Item = T;
    type IntoIter = DynamicArrayIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!("Convert array to iterator")
    }
}

// TODO: Implement Drop for DynamicArrayIter
// Must drop remaining elements and free memory

// TODO: Implement Extend trait
// This allows: arr.extend(vec![1, 2, 3])
impl<T> Extend<T> for DynamicArray<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        todo!("Extend from iterator")
    }
}

// ============================================================================
// TESTS - DO NOT MODIFY
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dynamic_array() {
        let arr: DynamicArray<i32> = DynamicArray::new();
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.capacity(), 0);
    }

    #[test]
    fn test_push_triggers_growth() {
        let mut arr = DynamicArray::new();

        // First push should allocate
        arr.push(1);
        assert_eq!(arr.len(), 1);
        assert!(arr.capacity() >= 1);

        // Keep track of capacity changes
        let mut prev_cap = arr.capacity();

        // Push until we see growth
        for i in 2..20 {
            arr.push(i);
            if arr.capacity() > prev_cap {
                println!("Grew from {} to {}", prev_cap, arr.capacity());
                prev_cap = arr.capacity();
            }
        }
    }

    #[test]
    fn test_with_capacity() {
        let arr: DynamicArray<i32> = DynamicArray::with_capacity(10);
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.capacity(), 10);
    }

    #[test]
    fn test_shrink_behavior() {
        let mut arr = DynamicArray::with_capacity(100);

        // Fill to capacity
        for i in 0..100 {
            arr.push(i);
        }
        assert_eq!(arr.capacity(), 100);

        // Remove elements to get to 24% capacity (should trigger shrink)
        for _ in 0..76 {
            arr.pop();
        }
        assert_eq!(arr.len(), 24);

        // This should trigger shrinking
        arr.shrink_to_fit();
        assert_eq!(arr.capacity(), 50); // Shrunk to half
        assert_eq!(arr.len(), 24); // Length unchanged

        // Verify data integrity after shrink
        for i in 0..24 {
            assert_eq!(arr.get(i), Some(&i));
        }
    }

    #[test]
    fn test_shrink_no_trigger() {
        let mut arr = DynamicArray::with_capacity(100);

        // Fill to 30% capacity (should NOT trigger shrink)
        for i in 0..30 {
            arr.push(i);
        }

        arr.shrink_to_fit();
        assert_eq!(arr.capacity(), 100); // No shrink occurred
    }

    #[test]
    fn test_empty_shrink() {
        let mut arr: DynamicArray<i32> = DynamicArray::with_capacity(10);
        assert_eq!(arr.capacity(), 10);

        // Shrinking empty array should deallocate
        arr.shrink_to_fit();
        assert_eq!(arr.capacity(), 0);
        // Note: Can't check ptr.is_null() without exposing it

        // Should be able to push after deallocation
        arr.push(42);
        assert_eq!(arr.len(), 1);
        assert!(arr.capacity() >= 1);
        assert_eq!(arr.get(0), Some(&42));
    }

    #[test]
    fn test_small_capacity_no_shrink() {
        let mut arr = DynamicArray::with_capacity(4);

        arr.push(1);
        arr.shrink_to_fit();

        // Shouldn't shrink small arrays
        assert_eq!(arr.capacity(), 4);
    }

    #[test]
    fn test_grow_shrink_cycle() {
        let mut arr = DynamicArray::new();

        // Grow phase
        for i in 0..50 {
            arr.push(i);
        }
        let grown_capacity = arr.capacity();
        assert!(grown_capacity >= 50);

        // Shrink phase - remove enough to trigger shrink
        while arr.len() > grown_capacity / 5 {
            arr.pop();
        }

        arr.shrink_to_fit();
        assert!(arr.capacity() < grown_capacity);

        // Verify we can still use the array
        let shrunk_capacity = arr.capacity();
        while arr.len() < shrunk_capacity {
            arr.push(999);
        }
        assert_eq!(arr.capacity(), shrunk_capacity); // No new allocation needed
    }

    #[test]
    fn test_iterator() {
        let mut arr = DynamicArray::new();
        for i in 0..5 {
            arr.push(i);
        }

        let collected: Vec<i32> = arr.into_iter().collect();
        assert_eq!(collected, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_iterator_partial_consumption() {
        let mut arr = DynamicArray::new();
        for i in 0..10 {
            arr.push(i);
        }

        let mut iter = arr.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));

        // Drop the iterator early - remaining elements should be cleaned up
        drop(iter);
    }

    #[test]
    fn test_iterator_empty_array() {
        let arr: DynamicArray<i32> = DynamicArray::new();
        let collected: Vec<i32> = arr.into_iter().collect();
        assert!(collected.is_empty());
    }

    #[test]
    fn test_for_loop() {
        let mut arr = DynamicArray::new();
        arr.push(10);
        arr.push(20);
        arr.push(30);

        let mut sum = 0;
        for value in arr {
            sum += value;
        }
        assert_eq!(sum, 60);
    }

    #[test]
    fn test_extend() {
        let mut arr = DynamicArray::new();
        arr.push(1);
        arr.push(2);

        // Extend from a vector
        arr.extend(vec![3, 4, 5]);
        assert_eq!(arr.len(), 5);

        // Extend from an iterator
        arr.extend(6..=8);
        assert_eq!(arr.len(), 8);

        // Verify all values
        for i in 0..8 {
            assert_eq!(arr.get(i), Some(&(i as i32 + 1)));
        }
    }
}
