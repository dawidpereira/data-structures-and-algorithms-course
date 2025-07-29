# Common Array Implementation Pitfalls

## Prerequisites

- Understanding of arrays and memory management
- Basic Rust knowledge
- Completed at least one array implementation

## Table of Contents

- [Memory Management Pitfalls](#memory-management-pitfalls)
- [Array Access Pitfalls](#array-access-pitfalls)
- [Dynamic Array Pitfalls](#dynamic-array-pitfalls)
- [Implementation Pitfalls](#implementation-pitfalls)
- [Safety Pitfalls](#safety-pitfalls)
- [Performance Pitfalls](#performance-pitfalls)
- [Best Practices Summary](#best-practices-summary)

## Memory Management Pitfalls

### 1. Not Checking Allocation Success

**Problem:**

```rust
let ptr = alloc(layout) as *mut T;
// Using ptr without checking if it's null
```

**Solution:**

```rust
let ptr = alloc(layout) as *mut T;
if ptr.is_null() {
    panic!("Failed to allocate memory");
}
```

**Better Solution (using Result):**

```rust
fn new(capacity: usize) -> Result<Self, &'static str> {
    let layout = Layout::array::<T>(capacity)
        .map_err(|_| "Invalid capacity")?;
    
    let ptr = unsafe { alloc(layout) as *mut T };
    if ptr.is_null() {
        return Err("Failed to allocate memory");
    }
    
    Ok(Self { ptr, capacity, len: 0, _marker: PhantomData })
}
```

### 2. Forgetting to Drop Old Values

**Problem:**

```rust
// In set() method
unsafe {
    *self.ptr.add(index) = value;  // Old value not dropped!
}
```

**Solution:**

```rust
unsafe {
    ptr::drop_in_place(self.ptr.add(index));
    ptr::write(self.ptr.add(index), value);
}
```

### 3. Wrong Deallocation Layout

**Problem:**

```rust
// Allocation
let layout = Layout::array::<T>(capacity)?;
let ptr = alloc(layout);

// Deallocation with wrong capacity
let wrong_layout = Layout::array::<T>(len)?;  // Should be capacity!
dealloc(ptr, wrong_layout);
```

**Solution:** Always use the same layout for allocation and deallocation.

### 4. Missing Drop Implementation

**Problem:** Not implementing Drop trait leads to memory leaks.

**Solution:**

```rust
impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // Drop all valid elements
            for i in 0..self.len {
                unsafe { ptr::drop_in_place(self.ptr.add(i)); }
            }
            // Deallocate memory
            unsafe {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}
```

## Array Access Pitfalls

### 1. Out-of-Bounds Access

**Problem:** Accessing elements beyond array bounds.

**Common mistakes:**

- Using capacity instead of len for bounds checking
- Off-by-one errors (forgetting arrays are 0-indexed)
- Not checking bounds before access

**Solution:**

```rust
pub fn get(&self, index: usize) -> Option<&T> {
    if index >= self.len {  // Not self.capacity!
        return None;
    }
    unsafe { Some(&*self.ptr.add(index)) }
}
```

### 2. Reading Uninitialized Memory

**Problem:** Accessing memory between len and capacity.

**Solution:** Only access indices < len, never read uninitialized slots.

### 3. Zero-Indexing Confusion

**Problem:** Treating arrays as 1-indexed.

**Remember:** First element is at index 0, last element is at index len-1.

## Dynamic Array Pitfalls

### 1. Integer Overflow in Growth

**Problem:**

```rust
let new_capacity = self.capacity * 2;  // Can overflow!
```

**Solution:**

```rust
let new_capacity = self.capacity.checked_mul(2)
    .expect("Capacity overflow");
```

**Better Solution (using Result):**

```rust
fn grow(&mut self) -> Result<(), &'static str> {
    let new_capacity = self.capacity.checked_mul(2)
        .ok_or("Capacity would overflow")?;
    
    // ... rest of growth logic
    Ok(())
}
```

### 2. Thrashing (Grow/Shrink Loop)

**Problem:** Growing at capacity, shrinking at capacity/2.

**Solution:** Use hysteresis - shrink at 1/4 full to 1/2 size.

### 3. Forgetting Null Check in Shrink

**Problem:**

```rust
if self.capacity > 0 {
    dealloc(self.ptr, ...);  // ptr might be null!
}
```

**Solution:**

```rust
if self.capacity > 0 && !self.ptr.is_null() {
    dealloc(self.ptr, ...);
}
```

### 4. Wrong Initial Growth

**Problem:** Not handling 0 → 1 growth specially.

**Solution:**

```rust
let new_capacity = if self.capacity == 0 {
    1  // Special case
} else {
    self.capacity * 2
};
```

## Implementation Pitfalls

### 1. Using `as_ref()` on Raw Pointers

**Problem:** Creates unbounded lifetime.

**Solution:** Use `&*ptr` for proper lifetime inference.

### 2. Forgetting PhantomData

**Problem:** Missing ownership indication.

**Solution:**

```rust
pub struct Array<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
    _marker: PhantomData<T>,  // Don't forget!
}
```

### 3. Iterator Double-Free

**Problem:** Both iterator and array try to drop elements.

**Solution:**

```rust
impl<T> IntoIterator for Array<T> {
    fn into_iter(self) -> ArrayIter<T> {
        let iter = ArrayIter { ... };
        std::mem::forget(self);  // Prevent double-free
        iter
    }
}
```

### 4. Incorrect Type Cast for Realloc

**Problem:**

```rust
let new_ptr = realloc(self.ptr, old_layout, new_layout);  // Type error!
```

**Solution:**

```rust
let new_ptr = realloc(self.ptr as *mut u8, old_layout, new_layout) as *mut T;
```

## Error Handling Pitfalls

### 1. Using panic! Instead of Result

**Problem:** Using panic! or expect() makes the application crash on errors.

**Better Approach:** Return Result<T, E> to let callers handle errors:

```rust
// Instead of:
pub fn new(capacity: usize) -> Self {
    if capacity == 0 {
        panic!("Capacity cannot be zero");
    }
    // ...
}

// Use:
pub fn new(capacity: usize) -> Result<Self, ArrayError> {
    if capacity == 0 {
        return Err(ArrayError::InvalidCapacity);
    }
    // ...
}
```

### 2. Custom Error Types

**Better Practice:** Define custom error types for clarity:

```rust
#[derive(Debug)]
pub enum ArrayError {
    AllocationFailed,
    InvalidCapacity,
    CapacityOverflow,
    IndexOutOfBounds,
}

impl std::fmt::Display for ArrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AllocationFailed => write!(f, "Failed to allocate memory"),
            Self::InvalidCapacity => write!(f, "Invalid capacity specified"),
            Self::CapacityOverflow => write!(f, "Capacity would overflow"),
            Self::IndexOutOfBounds => write!(f, "Index out of bounds"),
        }
    }
}

impl std::error::Error for ArrayError {}
```

### 3. Propagating Errors

**Use the ? operator for clean error propagation:**

```rust
fn push(&mut self, value: T) -> Result<(), ArrayError> {
    if self.len == self.capacity {
        self.grow()?;  // Propagate growth errors
    }
    // ... rest of push logic
    Ok(())
}
```

## Safety Pitfalls

### 1. Not Validating Indices

Always check bounds before any array access.

### 2. Using Wrong Pointer Arithmetic

**Problem:** Manual offset calculation.

**Solution:** Use `ptr.add(index)` for proper arithmetic.

### 3. Mixing Initialized and Uninitialized Memory

Keep clear separation between len (initialized) and capacity (total).

## Performance Pitfalls

### 1. Frequent Insertions at Beginning

Arrays require O(n) time to insert at beginning - consider different structure.

### 2. Not Pre-allocating

**Problem:** Many small growths.

**Solution:** Use `with_capacity()` when size is known.

### 3. Over-eager Shrinking

Shrinking too frequently can hurt performance.

## Best Practices Summary

1. **Always validate indices** against len, not capacity
2. **Check allocation success** - handle null pointers
3. **Use checked arithmetic** to prevent overflows
4. **Implement Drop properly** to prevent memory leaks
5. **Document unsafe code** with safety requirements
6. **Use ptr::write** for uninitialized memory
7. **Keep len ≤ capacity** invariant
8. **Test edge cases** - empty arrays, single elements, capacity limits
9. **Profile before optimizing** - measure actual performance
10. **Prefer safe abstractions** when possible
11. **Use Result<T, E> instead of panic!** - let callers handle errors gracefully
12. **Define custom error types** - provide clear error messages
13. **Propagate errors with ?** - keep error handling clean

## Common Error Messages and Solutions

| Error | Likely Cause | Solution |
|-------|--------------|----------|
| SIGSEGV | Null pointer access | Check allocation success, return Result |
| SIGABRT | Double-free | Use mem::forget in iterators |
| "index out of bounds" | Bad bounds check | Check against len, not capacity |
| Memory leak | Missing Drop | Implement Drop trait |
| Type mismatch in realloc | Wrong pointer type | Cast to *mut u8 |
| panic! in library | Using expect/unwrap | Return Result<T, E> instead |
| "capacity overflow" | Unchecked multiplication | Use checked_mul, return Result |

## Next Steps

- Review your implementations for these pitfalls
- Add tests for edge cases
- Consider using tools like Miri for undefined behavior detection
- Study the reference implementations for best practices
