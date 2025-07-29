# Dynamic Array Implementation Guide

## Prerequisites

- [Fixed Array Implementation](./fixed-array-guide.md) - Complete this first!
- [Dynamic Array Strategies](../../../implementations/rust/docs/dynamic-array-strategies.md) - Understand growth/shrink
- [Memory Management](../../../implementations/rust/docs/memory-management.md) - Know realloc

## Table of Contents

- [Overview](#overview)
- [Key Differences from Fixed Array](#key-differences-from-fixed-array)
- [Core Methods](#core-methods)
- [Growth and Shrink Logic](#growth-and-shrink-logic)
- [Iterator Implementation](#iterator-implementation)
- [Safety and Performance](#safety-and-performance)

## Overview

Dynamic arrays build upon fixed arrays by automatically resizing when needed.
This guide covers the implementation details specific to dynamic arrays.

## Key Differences from Fixed Array

| Feature | Fixed Array | Dynamic Array |
|---------|-------------|---------------|
| Capacity | Set once, immutable | Changes during lifetime |
| Push | Returns Result | Always succeeds (grows) |
| Memory | Single allocation | Multiple reallocations |
| Initial state | Must specify capacity | Can start with 0 |

## Core Methods

### `new() -> Self`

Creates empty array with no allocation.

```rust
Self {
    ptr: ptr::null_mut(),  // No allocation yet
    capacity: 0,
    len: 0,
    _marker: PhantomData,
}
```

**Key Point**: Delays allocation until first push.

### `push(&mut self, value: T)`

Always succeeds by growing if needed.

**Steps:**

1. Check if full: `self.len == self.capacity`
2. If full, call `grow()`
3. Write value at `self.ptr.add(self.len)`
4. Increment length

**No Result<> needed** - growth handles capacity.

### `grow(&mut self)`

Doubles capacity (or 0→1 for first allocation).

**Algorithm:**

```rust
let new_capacity = if self.capacity == 0 {
    1  // Bootstrap from empty
} else {
    self.capacity.checked_mul(2)
        .expect("Capacity overflow")
};
```

**Two paths:**

1. **First allocation** (capacity = 0): Use `alloc()`
2. **Reallocation**: Use `realloc()`

### `shrink_to_fit(&mut self)`

Implements smart shrinking to prevent thrashing.

**Decision logic:**

```rust
if self.len < self.capacity / 4 && self.capacity > 4 {
    // Shrink to half
    new_capacity = self.capacity / 2;
} else if self.len == 0 && self.capacity > 0 {
    // Deallocate completely
    self.ptr = ptr::null_mut();
    self.capacity = 0;
}
```

## Growth and Shrink Logic

### Memory Reallocation

Using `realloc` for efficiency:

```rust
let ptr = unsafe {
    realloc(
        self.ptr as *mut u8,  // Current pointer
        old_layout,           // Current layout
        new_layout.size()     // New size only
    )
} as *mut T;
```

**Benefits of realloc:**

- May extend in-place (no copy)
- Handles copy if needed
- Single system call

### Handling Edge Cases

1. **Null pointer on first grow**

   ```rust
   if self.capacity == 0 {
       // Can't realloc null, use alloc
       alloc(layout)
   }
   ```

2. **Integer overflow protection**

   ```rust
   self.capacity.checked_mul(2)
       .unwrap_or_else(|| panic!("Overflow"))
   ```

3. **Minimum capacity for shrink**

   ```rust
   if self.capacity > 4  // Don't shrink tiny arrays
   ```

## Iterator Implementation

### IntoIterator Design

Ownership transfer requires careful handling:

```rust
pub struct DynamicArrayIter<T> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
    index: usize,         // Current position
    _marker: PhantomData<T>,
}
```

### Key Implementation Points

1. **Ownership Transfer**

   ```rust
   impl<T> IntoIterator for DynamicArray<T> {
       fn into_iter(self) -> DynamicArrayIter<T> {
           let iter = DynamicArrayIter { /* fields */ };
           std::mem::forget(self);  // Prevent double-free
           iter
       }
   }
   ```

2. **Iterator Drop**

   ```rust
   impl<T> Drop for DynamicArrayIter<T> {
       fn drop(&mut self) {
           // Drop remaining elements
           while self.index < self.len {
               unsafe { self.ptr.add(self.index).read(); }
               self.index += 1;
           }
           // Deallocate array
           if self.capacity > 0 {
               unsafe { dealloc(self.ptr as *mut u8, layout); }
           }
       }
   }
   ```

### Extend Trait

Enables building from iterators:

```rust
impl<T> Extend<T> for DynamicArray<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);  // Grows as needed
        }
    }
}
```

## Safety and Performance

### Invariants

Same as fixed array, plus:

1. `ptr` is null iff `capacity == 0`
2. After grow: `capacity >= len + 1`
3. After shrink: `capacity >= len`

### Performance Tips

1. **Pre-allocate when possible**

   ```rust
   // Better
   let mut arr = DynamicArray::with_capacity(1000);
   
   // Worse (multiple grows)
   let mut arr = DynamicArray::new();
   ```

2. **Avoid shrink in loops**

   ```rust
   // Bad: Might thrash
   while processing {
       arr.pop();
       arr.shrink_to_fit();
   }
   
   // Good: Shrink once
   while processing {
       arr.pop();
   }
   arr.shrink_to_fit();
   ```

3. **Consider growth factor**
   - 2x: Simple, good performance
   - 1.5x: Less memory waste
   - Choose based on use case

### Common Pitfalls

1. **Forgetting null check in shrink**

   ```rust
   // Wrong
   if self.capacity > 0 {
       dealloc(self.ptr, ...);  // Might be null!
   }
   
   // Right
   if self.capacity > 0 && !self.ptr.is_null() {
       dealloc(self.ptr, ...);
   }
   ```

2. **Not updating ptr after realloc**

   ```rust
   // realloc might return different address
   self.ptr = new_ptr;  // Don't forget!
   ```

3. **Iterator memory management**
   - Must `mem::forget` original array
   - Iterator must handle cleanup

## Testing Strategies

Key test cases:

1. **Growth sequence**: 0 → 1 → 2 → 4 → 8 → 16 → 32 → 64 → ...
2. **Shrink behavior**: Verify 1/4 trigger
3. **Thrash prevention**: Push/pop cycles
4. **Iterator consumption**: Partial and full
5. **Edge cases**: Empty, single element, capacity limits

## Next Steps

Congratulations on implementing dynamic arrays!

1. **[Compare with Reference](../../../implementations/rust/src/dynamic_array.rs)** - See how your solution compares
2. **[Array Algorithms](../../../algorithms/)** - Use your arrays for algorithms
3. **[Next Data Structure: Linked Lists](../../../../02-linked-lists/)** - Learn a different approach to dynamic storage
