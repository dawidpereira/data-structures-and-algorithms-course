//! Dynamic array that automatically grows when full.
//!
//! For detailed explanations, see the docs/ folder in this directory.

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::marker::PhantomData;
use std::ptr;

/// A growable array that resizes automatically.
pub struct DynamicArray<T> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> Default for DynamicArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DynamicArray<T> {
    /// Creates a new empty dynamic array.
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            capacity: 0,
            len: 0,
            _marker: PhantomData,
        }
    }

    /// Creates a new dynamic array with initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
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

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the current capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Adds an element to the end, growing if needed.
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            let ptr = self.ptr.add(self.len);
            ptr.write(value);
        }
        self.len += 1;
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity.checked_mul(2).unwrap_or_else(|| {
                panic!("Cannot grow array beyond maximum capacity");
            })
        };

        if self.capacity == 0 {
            let layout = Layout::array::<T>(new_capacity).unwrap();
            let ptr = unsafe { alloc(layout) as *mut T };

            if ptr.is_null() {
                panic!("Failed to allocate memory");
            }

            self.ptr = ptr;
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            let new_layout = Layout::array::<T>(new_capacity).unwrap();
            let ptr =
                unsafe { realloc(self.ptr as *mut u8, old_layout, new_layout.size()) as *mut T };

            if ptr.is_null() {
                panic!("Failed to allocate memory");
            }

            self.ptr = ptr;
        }

        self.capacity = new_capacity;
    }

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

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(&*self.ptr.add(index)) }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(&mut *self.ptr.add(index)) }
    }

    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    /// Shrinks the capacity to reduce memory usage.
    ///
    /// Uses smart shrinking to prevent thrashing. Only shrinks when
    /// array is less than 25% full, and shrinks to 50% capacity.
    pub fn shrink_to_fit(&mut self) {
        if self.len > 0 && self.len < self.capacity / 4 && self.capacity > 4 {
            let new_capacity = self.capacity / 2;

            let new_capacity = new_capacity.max(self.len);

            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            let new_layout = Layout::array::<T>(new_capacity).unwrap();

            let ptr =
                unsafe { realloc(self.ptr as *mut u8, old_layout, new_layout.size()) as *mut T };

            if ptr.is_null() {
                panic!("Failed to shrink memory");
            }

            self.ptr = ptr;
            self.capacity = new_capacity;
        } else if self.len == 0 && self.capacity > 0 {
            unsafe {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
            self.ptr = ptr::null_mut();
            self.capacity = 0;
        }
    }
}

impl<T> Drop for DynamicArray<T> {
    fn drop(&mut self) {
        self.clear();

        if self.capacity > 0 {
            unsafe {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

unsafe impl<T: Send> Send for DynamicArray<T> {}
unsafe impl<T: Sync> Sync for DynamicArray<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for DynamicArray<T> {
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
impl<T> Index<usize> for DynamicArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Index out of bounds")
    }
}

pub struct DynamicArrayIter<T> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
    index: usize,
    _marker: PhantomData<T>,
}

impl<T> Iterator for DynamicArrayIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let value = unsafe {
                let ptr = self.ptr.add(self.index);
                ptr.read()
            };
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<T> IntoIterator for DynamicArray<T> {
    type Item = T;
    type IntoIter = DynamicArrayIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = DynamicArrayIter {
            ptr: self.ptr,
            capacity: self.capacity,
            len: self.len,
            index: 0,
            _marker: PhantomData,
        };

        std::mem::forget(self);

        iter
    }
}

impl<T> Extend<T> for DynamicArray<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T> Drop for DynamicArrayIter<T> {
    fn drop(&mut self) {
        while self.index < self.len {
            unsafe {
                let ptr = self.ptr.add(self.index);
                ptr.read();
            }
            self.index += 1;
        }

        if self.capacity > 0 && !self.ptr.is_null() {
            unsafe {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

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

        arr.push(1);
        assert_eq!(arr.len(), 1);
        assert!(arr.capacity() >= 1);

        let mut prev_cap = arr.capacity();

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

        for i in 0..100 {
            arr.push(i);
        }
        assert_eq!(arr.capacity(), 100);

        for _ in 0..76 {
            arr.pop();
        }
        assert_eq!(arr.len(), 24);

        arr.shrink_to_fit();
        assert_eq!(arr.capacity(), 50);
        assert_eq!(arr.len(), 24);

        for i in 0..24 {
            assert_eq!(arr.get(i), Some(&i));
        }
    }

    #[test]
    fn test_shrink_no_trigger() {
        let mut arr = DynamicArray::with_capacity(100);

        for i in 0..30 {
            arr.push(i);
        }

        arr.shrink_to_fit();
        assert_eq!(arr.capacity(), 100);
    }

    #[test]
    fn test_empty_shrink() {
        let mut arr: DynamicArray<i32> = DynamicArray::with_capacity(10);
        assert_eq!(arr.capacity(), 10);

        arr.shrink_to_fit();
        assert_eq!(arr.capacity(), 0);
        assert!(arr.ptr.is_null());

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

        assert_eq!(arr.capacity(), 4);
    }

    #[test]
    fn test_grow_shrink_cycle() {
        let mut arr = DynamicArray::new();

        for i in 0..50 {
            arr.push(i);
        }
        let grown_capacity = arr.capacity();
        assert!(grown_capacity >= 50);

        while arr.len() > grown_capacity / 5 {
            arr.pop();
        }

        arr.shrink_to_fit();
        assert!(arr.capacity() < grown_capacity);

        let shrunk_capacity = arr.capacity();
        while arr.len() < shrunk_capacity {
            arr.push(999);
        }
        assert_eq!(arr.capacity(), shrunk_capacity);
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

        arr.extend(vec![3, 4, 5]);
        assert_eq!(arr.len(), 5);

        arr.extend(6..=8);
        assert_eq!(arr.len(), 8);

        for i in 0..8 {
            assert_eq!(arr.get(i), Some(&(i as i32 + 1)));
        }
    }
}
