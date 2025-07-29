# Dynamic Array Growth and Shrink Strategies

## Prerequisites

- [Array Theory](../../../theory.md) - Understanding static vs dynamic arrays
- [Memory Management](./memory-management.md) - Memory allocation concepts
- [Fixed Array Implementation](../../../exercises/rust/docs/fixed-array-guide.md) - Basic array operations

## Table of Contents

- [Overview](#overview)
- [Growth Strategy](#growth-strategy)
- [The Thrashing Problem](#the-thrashing-problem)
- [Smart Shrinking Strategy](#smart-shrinking-strategy)
- [Amortized Analysis](#amortized-analysis)
- [Implementation Details](#implementation-details)

## Overview

Dynamic arrays automatically resize to accommodate more elements. The key challenge
is choosing growth and shrink strategies that balance memory usage with performance.

## Growth Strategy

### Common Approaches

1. **Fixed Increment** (capacity += 10)
   - ✗ Poor for large arrays (O(n²) total cost)
   - ✓ Predictable memory usage

2. **Multiplicative Growth** (capacity *= factor)
   - ✓ Amortized O(1) insertion
   - ✓ Good performance for any size
   - Common factors: 1.5x, 2x, golden ratio

### Our Choice: 2x Growth

We double capacity when full:

```
0 → 1 → 2 → 4 → 8 → 16 → 32 → 64 → ...
```

**Benefits:**

- Simple to implement
- Amortized O(1) push operations
- Good cache performance
- Power-of-2 sizes can be optimized by allocators

## The Thrashing Problem

### What is Thrashing?

Rapid alternation between growing and shrinking:

```
capacity=20, len=20 → push() → grow to 40
capacity=40, len=21 → pop() → shrink to 21?
capacity=21, len=20 → push() → grow to 42?
... endless resizing!
```

### Why It's Bad

1. **Performance**: Each resize is O(n)
2. **Memory fragmentation**: Constant allocation/deallocation
3. **Cache misses**: Data keeps moving

### Bad Shrinking Strategies

1. **Shrink to exact size**

   ```
   len=10, capacity=20 → shrink to 10
   Problem: Next push forces immediate grow
   ```

2. **Shrink when not full**

   ```
   if len < capacity { shrink(); }
   Problem: Thrashes at boundary
   ```

## Smart Shrinking Strategy

### The 1/4 Full, Shrink to 1/2 Rule

Only shrink when array is less than 25% full, and shrink to 50%:

```
if len < capacity / 4 && capacity > 4 {
    new_capacity = capacity / 2;
}
```

### Example Walkthrough

```
Initial: capacity=100, len=100

Remove 50 elements:
capacity=100, len=50 (50% full) → No shrink

Remove 25 more:
capacity=100, len=25 (25% full) → No shrink

Remove 2 more:
capacity=100, len=23 (23% full) → Shrink!
new_capacity = 50

After shrink:
capacity=50, len=23 (46% full)
```

### Why This Works

1. **Hysteresis Gap**
   - Grow at: 100% full
   - Shrink at: <25% full
   - Large gap prevents oscillation

2. **Room to Grow**
   - After shrinking to 50%, array is 46-50% full
   - Can double elements before needing to grow

3. **Batch Operations**
   - Must remove 75% of elements to trigger shrink
   - Must then add 100% to trigger grow
   - Natural buffer for add/remove patterns

## Amortized Analysis

### Growth Cost

When doubling capacity:

1. Allocate new array: O(n)
2. Copy elements: O(n)
3. Deallocate old: O(1)

Total: O(n) per growth

### Amortized O(1)

For n insertions starting from empty:

- Growths at: 1, 2, 4, 8, ..., n/2, n
- Total cost: 1 + 2 + 4 + ... + n = 2n - 1
- Amortized cost per operation: O(1)

### Shrinking Cost

Similar analysis:

- Must remove n/2 elements to trigger shrink
- Shrink cost: O(n)
- Amortized over n/2 removals: O(1) per operation

## Implementation Details

### Growth Implementation

```rust
fn grow(&mut self) {
    let new_capacity = if self.capacity == 0 {
        1  // Initial allocation
    } else {
        self.capacity.checked_mul(2)
            .expect("Array too large")
    };
    
    // Reallocate
    let new_layout = Layout::array::<T>(new_capacity).unwrap();
    let new_ptr = unsafe {
        if self.capacity == 0 {
            alloc(new_layout)
        } else {
            realloc(self.ptr as *mut u8, old_layout, new_layout.size())
        }
    } as *mut T;
    
    // Update state
    self.ptr = new_ptr;
    self.capacity = new_capacity;
}
```

### Shrink Implementation

```rust
fn shrink_to_fit(&mut self) {
    // Check if should shrink
    if self.len > 0 && self.len < self.capacity / 4 && self.capacity > 4 {
        let new_capacity = self.capacity / 2;
        // ... realloc to new_capacity
    }
    // Special case: empty array
    else if self.len == 0 && self.capacity > 0 {
        // Deallocate completely
    }
}
```

### Edge Cases

1. **Initial Growth**: 0 → 1 (special case)
2. **Integer Overflow**: Use `checked_mul`
3. **Minimum Size**: Don't shrink below 4
4. **Empty Array**: Deallocate completely

## Performance Characteristics

| Operation | Worst Case | Amortized |
|-----------|------------|-----------|
| Push      | O(n)       | O(1)      |
| Pop       | O(1)       | O(1)      |
| Shrink    | O(n)       | O(1)      |
| Access    | O(1)       | O(1)      |

## Best Practices

1. **Pre-allocate** when size is known: `with_capacity(n)`
2. **Avoid manual shrinking** in loops
3. **Consider memory pressure** vs performance
4. **Profile your specific use case**

## Alternative Strategies

### Growth Factors

- **1.5x**: Better memory usage, more frequent reallocations
- **Golden Ratio (1.618x)**: Theoretical benefits, complex
- **2x**: Simple, good performance, our choice

### Shrink Triggers

- **1/3 full**: More aggressive memory reclaim
- **1/4 full**: Balanced approach (our choice)
- **1/8 full**: Very conservative

### Capacity Policies

- **Never shrink**: Maximum performance
- **Shrink on request**: User control
- **Automatic shrink**: Convenience (our choice)

## Next Steps

Ready to implement dynamic arrays:

1. **[Dynamic Array Implementation Guide](../../../exercises/rust/docs/dynamic-array-guide.md)** - Build your own dynamic array
2. **[Reference Implementation](../src/dynamic_array.rs)** - See a complete implementation
3. **[Exercise Tests](../../../exercises/rust/src/dynamic_array_exercise.rs)** - Test your implementation
