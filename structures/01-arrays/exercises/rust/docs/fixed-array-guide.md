# Fixed Array Implementation Guide

## Prerequisites

- [Array Theory](../../../theory.md) - Understand array concepts
- [Memory Management](../../../implementations/rust/docs/memory-management.md) - Know heap allocation
- Basic Rust knowledge (ownership, unsafe code)

## Table of Contents

- [Overview](#overview)
- [Struct Design](#struct-design)
- [Core Methods](#core-methods)
- [Memory Management](#memory-management)
- [Safety Considerations](#safety-considerations)

## Overview

This guide walks through implementing a fixed-size array from scratch using raw
memory allocation. The array has a fixed capacity set at creation time.

## Struct Design

```rust
pub struct Array<T> {
    ptr: *mut T,             // Raw pointer to allocated memory
    capacity: usize,         // Fixed size (never changes)
    len: usize,              // Current number of elements
    _marker: PhantomData<T>, // Indicates we own values of type T
}
```

### Field Explanations

- **ptr**: Points to the start of our allocated memory block
- **capacity**: Total slots allocated (set once, never changes)
- **len**: How many slots actually contain valid data
- **_marker**: Tells Rust we conceptually own `T` values (for drop check)

## Core Methods

### `new(capacity: usize) -> Self`

Creates a new array with fixed capacity.

**Steps:**

1. Validate capacity > 0 (panic if not)
2. Calculate memory layout: `Layout::array::<T>(capacity)`
3. Allocate memory: `unsafe { alloc(layout) as *mut T }`
4. Check allocation succeeded (panic if null)
5. Initialize struct with ptr, capacity, len=0

**Key Concepts:**

- Heap allocation
- Layout calculation
- Null pointer checking

### `push(&mut self, value: T) -> Result<(), T>`

Adds element to end if space available.

**Steps:**

1. Check if full: `self.len >= self.capacity`
2. If full, return `Err(value)`
3. Calculate position: `self.ptr.add(self.len)`
4. Write value: `ptr.write(value)`
5. Increment length

**Key Concepts:**

- Bounds checking
- Writing to uninitialized memory
- Error handling via Result

### `pop(&mut self) -> Option<T>`

Removes and returns last element.

**Steps:**

1. Check if empty: `self.len == 0`
2. If empty, return None
3. Decrement length
4. Read value: `self.ptr.add(self.len).read()`
5. Return Some(value)

**Key Concepts:**

- Reading moves value out
- Length tracks valid elements

### `get(&self, index: usize) -> Option<&T>`

Returns reference to element at index.

**Steps:**

1. Bounds check: `index >= self.len`
2. If out of bounds, return None
3. Calculate pointer: `self.ptr.add(index)`
4. Convert to reference: `&*ptr`
5. Wrap in Some

**Why `&*ptr`?**

- `ptr` is type `*mut T` (raw pointer)
- `*ptr` dereferences to `T`
- `&*ptr` takes reference to get `&T`
- This converts unsafe pointer to safe reference

### `set(&mut self, index: usize, value: T)`

Replaces value at index.

**Steps:**

1. Bounds check (panic if out of bounds)
2. Get pointer: `self.ptr.add(index)`
3. Read old value: `ptr.read()` (this drops it)
4. Write new value: `ptr.write(value)`

**Why read before write?**
For types that own heap memory (String, Vec, etc.):

- Just overwriting would leak the old value's heap memory
- Reading moves it out, triggering its destructor

## Memory Management

### Drop Implementation

```rust
impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        // 1. Drop all elements
        self.clear();  // Calls pop() in loop
        
        // 2. Deallocate memory
        if self.capacity > 0 {
            unsafe {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}
```

**Why two steps?**

1. Elements might own resources that need cleanup
2. Then we free the memory block itself

### Memory Leak Prevention

Common leak scenarios and solutions:

1. **Overwriting without drop**

   ```rust
   // ✗ LEAKS: Old value never dropped
   unsafe { self.ptr.add(index).write(new_value); }
   
   // ✓ CORRECT: Read drops old value
   unsafe {
       self.ptr.add(index).read();
       self.ptr.add(index).write(new_value);
   }
   ```

2. **Not implementing Drop**
   - Memory block never freed
   - Elements never dropped

3. **Panic during operation**
   - Use drop guards or careful ordering
   - Ensure consistent state

## Safety Considerations

### Unsafe Operations

Our implementation uses unsafe for:

1. Memory allocation/deallocation
2. Pointer arithmetic
3. Reading/writing raw memory

### Safety Invariants

We maintain these invariants:

1. `ptr` is valid for `capacity` elements (or null if capacity=0)
2. Elements at indices `0..len` are initialized
3. Elements at indices `len..capacity` are uninitialized
4. No aliasing of mutable references

### Index Safety

Using `usize` for indices prevents negative indices:

```rust
// This won't compile - usize can't be negative
arr.get(-1);  // Compile error!
```

### Send and Sync

```rust
unsafe impl<T: Send> Send for Array<T> {}
unsafe impl<T: Sync> Sync for Array<T> {}
```

Array is Send/Sync if T is Send/Sync because:

- We own the data
- No internal mutability
- Proper synchronization is caller's responsibility

## Common Implementation Mistakes

For a detailed guide on avoiding pitfalls, see
[Common Array Implementation Pitfalls](../../../implementations/rust/docs/common-pitfalls.md).

Quick reminders:
- Check allocation success (null pointer)
- Match dealloc layout to alloc layout
- Only read indices < len
- Don't forget PhantomData
- Use `&*ptr` instead of `as_ref()`

## Next Steps

After implementing fixed arrays:

1. **[Dynamic Array Strategies](../../../implementations/rust/docs/dynamic-array-strategies.md)** - Learn about growth strategies
2. **[Dynamic Array Guide](./dynamic-array-guide.md)** - Build growable arrays
3. **[Reference Implementation](../../../implementations/rust/src/core.rs)** - Compare your solution

