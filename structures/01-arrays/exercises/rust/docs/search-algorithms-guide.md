# Search Algorithms Implementation Guide

## Table of Contents

- [Overview](#overview)
- [Exercise Structure](#exercise-structure)
- [Exercise 1: Linear Search Variants](#exercise-1-linear-search-variants)
- [Exercise 2: Jump Search](#exercise-2-jump-search)
- [Exercise 3: Interpolation Search](#exercise-3-interpolation-search)
- [Exercise 4: Performance Comparison](#exercise-4-performance-comparison)
- [Exercise 5: Practical Applications](#exercise-5-practical-applications)
- [Testing Your Solutions](#testing-your-solutions)
- [Common Pitfalls](#common-pitfalls)

## Overview

This guide will help you implement various search algorithms from scratch.
You'll start with simple linear search, progress to jump search, and optionally
tackle interpolation search.

## Exercise Structure

The exercises build on each other:

1. **Linear Search** - Foundation for all searching
2. **Jump Search** - Optimization for sorted data
3. **Interpolation Search** - Advanced technique for numeric data
4. **Performance Analysis** - Understanding trade-offs
5. **Real Applications** - Using the right algorithm

## Exercise 1: Linear Search Variants

### Basic Linear Search

The simplest search - check each element:

```rust
pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for i in 0..arr.len() {
        if &arr[i] == target {
            return Some(i);
        }
    }
    None
}
```

**Key Points:**

- Works on unsorted data
- Returns first occurrence
- O(n) time complexity

### Find All Occurrences

Instead of returning on first match, collect all matches:

```rust
pub fn linear_search_all<T: PartialEq>(arr: &[T], target: &T) -> Vec<usize> {
    let mut indices = Vec::new();
    for i in 0..arr.len() {
        if &arr[i] == target {
            indices.push(i);
        }
    }
    indices
}
```

### Search with Predicate

More flexible - search based on a condition:

```rust
pub fn linear_search_if<T, F>(arr: &[T], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    for i in 0..arr.len() {
        if predicate(&arr[i]) {
            return Some(i);
        }
    }
    None
}
```

### Reverse Linear Search

Start from the end:

```rust
pub fn reverse_linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for i in (0..arr.len()).rev() {
        if &arr[i] == target {
            return Some(i);
        }
    }
    None
}
```

## Exercise 2: Jump Search

### Understanding Jump Search

Jump search combines jumping and linear search:

1. Jump forward by √n steps
2. When you overshoot, go back
3. Linear search in that block

### Calculate Jump Size

```rust
pub fn calculate_jump_size(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    (n as f64).sqrt().ceil() as usize
}
```

### Implementation Strategy

```rust
pub fn jump_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let n = arr.len();
    if n == 0 {
        return None;
    }
    
    let jump = calculate_jump_size(n);
    let mut prev = 0;
    
    // Jump forward until we find a block that might contain target
    while prev < n && &arr[min(prev + jump - 1, n - 1)] < target {
        prev += jump;
    }
    
    // Linear search in the identified block
    for i in prev..min(prev + jump, n) {
        if &arr[i] == target {
            return Some(i);
        }
        if &arr[i] > target {
            break; // Can't find it (sorted array)
        }
    }
    
    None
}
```

**Important:** Use `min()` to avoid out-of-bounds access!

## Exercise 3: Interpolation Search

### The Interpolation Formula

Instead of always checking the middle, estimate position:

```rust
let pos = low + ((target - arr[low]) as f64 / 
                 (arr[high] - arr[low]) as f64 * 
                 (high - low) as f64) as usize;
```

### Implementation Tips

1. **Check bounds**: Ensure target is within current range
2. **Handle edge cases**: Same values, single element
3. **Prevent overflow**: Be careful with arithmetic

```rust
pub fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    
    while low <= high && target >= arr[low] && target <= arr[high] {
        if low == high {
            return if arr[low] == target { Some(low) } else { None };
        }
        
        // Calculate position
        let pos = low + ((target - arr[low]) as f64 / 
                        (arr[high] - arr[low]) as f64 * 
                        (high - low) as f64) as usize;
        
        match arr[pos].cmp(&target) {
            Ordering::Equal => return Some(pos),
            Ordering::Less => low = pos + 1,
            Ordering::Greater => {
                if pos == 0 { return None; }
                high = pos - 1;
            }
        }
    }
    
    None
}
```

## Exercise 4: Performance Comparison

### Adding Counters

Modify algorithms to count comparisons:

```rust
pub struct SearchResult {
    pub index: Option<usize>,
    pub comparisons: usize,
}

pub fn linear_search_counted<T: PartialEq>(arr: &[T], target: &T) -> SearchResult {
    let mut comparisons = 0;
    
    for i in 0..arr.len() {
        comparisons += 1;
        if &arr[i] == target {
            return SearchResult { index: Some(i), comparisons };
        }
    }
    
    SearchResult { index: None, comparisons }
}
```

### Expected Results

For array of size n:

- Linear search: Average n/2 comparisons
- Jump search: Average √n comparisons
- Binary search: Average log₂(n) comparisons

## Exercise 5: Practical Applications

### Choosing the Right Algorithm

| Scenario | Best Algorithm | Why |
|----------|---------------|-----|
| Find by sorted ID | Jump/Binary | O(√n) or O(log n) |
| Find all with grade | Linear | Must check all anyway |
| Find first above threshold | Linear with predicate | Not sorted by grade |

### Implementation Example

```rust
pub fn find_student_by_id(students: &[Student], id: u32) -> Option<usize> {
    // Students sorted by ID - use jump search
    let n = students.len();
    let jump = (n as f64).sqrt().ceil() as usize;
    
    let mut prev = 0;
    while prev < n && students[min(prev + jump - 1, n - 1)].id < id {
        prev += jump;
    }
    
    for i in prev..min(prev + jump, n) {
        if students[i].id == id {
            return Some(i);
        }
        if students[i].id > id {
            break;
        }
    }
    
    None
}
```

## Testing Your Solutions

Run tests incrementally:

```bash
# Test one exercise at a time
cargo test exercise1
cargo test exercise2
# etc.

# Run all tests
cargo test
```

## Common Pitfalls

### 1. Off-by-One Errors

```rust
// Wrong - misses last element
for i in 0..arr.len() - 1

// Correct
for i in 0..arr.len()
```

### 2. Integer Overflow

```rust
// Wrong - can overflow
let pos = (low + high) / 2;

// Correct
let pos = low + (high - low) / 2;
```

### 3. Forgetting Bounds Checks

```rust
// Wrong - can panic
let value = arr[prev + jump];

// Correct
let value = arr[min(prev + jump, n - 1)];
```

### 4. Not Handling Empty Arrays

Always check:

```rust
if arr.is_empty() {
    return None;
}
```

### Performance Tips

1. **Minimize comparisons**: Check algorithm efficiency
2. **Consider cache**: Linear search can be faster for small arrays
3. **Know your data**: Distribution matters for interpolation search
4. **Test edge cases**: Empty, single element, duplicates

## Next Steps

After completing these exercises:

1. Implement binary search variations
2. Try exponential search for unbounded arrays
3. Experiment with parallel search algorithms
4. Profile performance on real datasets

Remember: Understanding when to use each algorithm is as important as
knowing how to implement them!
