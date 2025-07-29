//! Fixed-size array implementation using raw memory allocation.
//!
//! This module demonstrates building arrays from scratch without built-in types.
//! For detailed explanations, see the docs/ folder in this directory.

use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;

/// A fixed-size array with capacity set at creation time.
pub struct Array<T> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> Array<T> {
    /// Creates a new array with the specified capacity.
    ///
    /// # Panics
    /// - If capacity is 0
    /// - If memory allocation fails
    ///
    /// # Examples
    /// ```
    /// use arrays::core::Array;
    /// let arr: Array<i32> = Array::new(10);
    /// assert_eq!(arr.capacity(), 10);
    /// assert_eq!(arr.len(), 0);
    /// ```
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("Array capacity must be greater than 0");
        }

        let layout = Layout::array::<T>(capacity).unwrap();

        let ptr = unsafe { alloc(layout) as *mut T };

        if ptr.is_null() {
            panic!("Failed to allocate memory");
        }

        Self {
            ptr,
            capacity,
            len: 0,
            _marker: PhantomData,
        }
    }

    /// Returns the number of elements currently in the array
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the array contains no elements
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the maximum capacity of the array
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Adds an element to the end of the array.
    ///
    /// Returns `Err(value)` if the array is full.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= self.capacity {
            return Err(value);
        }

        unsafe {
            let ptr = self.ptr.add(self.len);

            ptr.write(value);
        }

        self.len += 1;
        Ok(())
    }

    /// Removes and returns the last element.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        unsafe {
            let ptr = self.ptr.add(self.len);
            Some(ptr.read())
        }
    }

    /// Returns a reference to the element at the given index.
    ///
    /// Returns `None` if index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(&*self.ptr.add(index)) }
    }

    /// Returns a mutable reference to the element at the given index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(&mut *self.ptr.add(index)) }
    }

    /// Sets the value at the given index.
    ///
    /// # Panics
    /// Panics if index is out of bounds.
    pub fn set(&mut self, index: usize, value: T) {
        assert!(
            index < self.len,
            "Index {} out of bounds for length {}",
            index,
            self.len
        );

        unsafe {
            let ptr = self.ptr.add(index);
            ptr.read(); // This drops the old value
            ptr.write(value);
        }
    }

    /// Creates an array from a slice.
    ///
    /// Returns `None` if the slice is larger than the specified capacity.
    pub fn from_slice(slice: &[T], capacity: usize) -> Option<Self>
    where
        T: Clone,
    {
        if slice.len() > capacity {
            return None;
        }

        let mut array = Self::new(capacity);

        for item in slice {
            array.push(item.clone()).ok()?;
        }

        Some(array)
    }

    /// Clears the array, removing all elements.
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        self.clear();

        if self.capacity > 0 {
            unsafe {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout)
            }
        }
    }
}

unsafe impl<T: Send> Send for Array<T> {}
unsafe impl<T: Sync> Sync for Array<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for i in 0..self.len {
            if i > 0 {
                write!(f, ", ")?;
            }
            unsafe {
                let element = &*self.ptr.add(i);
                write!(f, "{element:?}")?;
            }
        }

        write!(f, "]")
    }
}

use std::ops::Index;
impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Index out of bounds")
    }
}

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

        assert!(arr.push(1).is_ok());
        assert!(arr.push(2).is_ok());
        assert!(arr.push(3).is_ok());
        assert_eq!(arr.len(), 3);

        assert!(arr.push(4).is_err());

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
        // We create arrays in a scope and let them drop
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

        // Original "B" should have been dropped (no memory leak)
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
