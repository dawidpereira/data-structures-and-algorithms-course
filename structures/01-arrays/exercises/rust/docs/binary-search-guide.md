# Binary Search Implementation Guide

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Exercise Structure](#exercise-structure)
- [Implementation Tips](#implementation-tips)
- [Common Mistakes](#common-mistakes)
- [Testing Your Solution](#testing-your-solution)
- [Solution Hints](#solution-hints)

## Overview

This guide will help you implement binary search from scratch. Binary search
is one of the most important algorithms to understand deeply, as it teaches
fundamental computer science concepts.

## Prerequisites

Before starting, make sure you understand:

1. **How binary search works conceptually**
   - Read the [theory documentation](/algorithms/searching/binary-search/theory.md)
   - Study the [visual diagrams](/algorithms/searching/binary-search/diagrams/)

2. **Rust syntax for:**
   - Slices and indexing
   - Option type
   - Ordering enum
   - Generic functions with trait bounds

3. **The requirement**: Array must be sorted!

## Exercise Structure

The exercises are designed to build on each other:

1. **Basic Binary Search** - The foundation
2. **Custom Comparison** - Flexibility in searching
3. **Find First/Last** - Handling duplicates
4. **Insertion Point** - Where to insert new elements
5. **Rotated Array** (Bonus) - Advanced application

## Implementation Tips

### Exercise 1: Basic Binary Search

```rust
pub fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    // Step 1: Handle empty slice
    if slice.is_empty() {
        return None;
    }
    
    // Step 2: Initialize boundaries
    let mut low = 0;
    let mut high = slice.len() - 1;
    
    // Step 3: Main loop
    while low <= high {
        // Calculate mid safely
        let mid = low + (high - low) / 2;
        
        // Compare and adjust
        match slice[mid].cmp(target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => {
                // Prevent underflow when mid is 0
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }
    }
    
    None
}
```

### Key Points

1. **Overflow Prevention**: Use `mid = low + (high - low) / 2` not `(low + high) / 2`
2. **Underflow Prevention**: Check `if mid == 0` before `high = mid - 1`
3. **Loop Condition**: Use `while low <= high` (with equals!)
4. **Match Statement**: Clean way to handle three-way comparison

### Exercise 2: Custom Comparison

This is similar to Exercise 1, but instead of using `.cmp()`, use the provided
comparison function:

```rust
match compare(&slice[mid]) {
    Ordering::Equal => return Some(mid),
    // ... rest is the same
}
```

### Exercise 3 & 4: Finding First/Last

The trick here is to not return immediately when you find a match:

```rust
// For finding first occurrence
Ordering::Equal => {
    result = Some(mid);  // Save the index
    // Continue searching in the left half
    if mid == 0 {
        break;
    }
    high = mid - 1;
}
```

### Exercise 5: Insertion Point

This is subtly different - you're looking for where to insert, not finding
an exact match:

```rust
while low < high {  // Note: < not <=
    let mid = low + (high - low) / 2;
    
    if slice[mid] < *target {
        low = mid + 1;
    } else {
        high = mid;  // Not mid - 1!
    }
}
```

## Common Mistakes

### 1. Integer Overflow

```rust
// ❌ Wrong - can overflow
let mid = (low + high) / 2;

// ✅ Correct
let mid = low + (high - low) / 2;
```

### 2. Infinite Loops

```rust
// ❌ Wrong - infinite loop when low == high
while low < high {
    let mid = low;  // If mid == low and we set low = mid...
}

// ✅ Correct - always make progress
while low <= high {
    // Ensure low = mid + 1 or high = mid - 1
}
```

### 3. Off-by-One Errors

```rust
// ❌ Wrong - misses last element
let mut high = slice.len() - 2;

// ✅ Correct
let mut high = slice.len() - 1;
```

### 4. Underflow with usize

```rust
// ❌ Wrong - underflow when mid is 0
high = mid - 1;  // If mid is 0, this underflows!

// ✅ Correct
if mid == 0 {
    break;
}
high = mid - 1;
```

## Testing Your Solution

Run the tests with:

```bash
cargo test binary_search_exercise
```

To run tests one at a time:

```bash
cargo test test_basic_binary_search
cargo test test_find_first_occurrence
# etc.
```

## Solution Hints

### Still Stuck?

1. **Draw it out**: Trace through a small example on paper
2. **Add debug prints**: Print low, mid, high, and the values at each step
3. **Test edge cases**: Empty array, single element, not found
4. **Check the reference**: Look at the implementation in `src/algorithms/binary_search.rs`

### Debugging Template

```rust
println!("Iteration: low={}, high={}, mid={}", low, high, mid);
println!("  slice[mid]={:?}, target={:?}", &slice[mid], target);
```

### Performance Check

Your solution should handle the large array test efficiently. If it's slow,
you might be doing linear search by mistake!

## Next Steps

After completing these exercises:

1. Try implementing binary search on our custom `Array<T>` type
2. Implement interpolation search for uniformly distributed data
3. Study how binary search is used in real applications (databases, etc.)
4. Practice binary search problems on coding platforms

Remember: Binary search is simple in concept but tricky in implementation.
Getting all the edge cases right is what makes you a better programmer!

