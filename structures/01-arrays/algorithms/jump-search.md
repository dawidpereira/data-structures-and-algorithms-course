# Jump Search on Arrays

## Table of Contents

- [Why Arrays are Ideal for Jump Search](#why-arrays-are-ideal-for-jump-search)
- [Implementation Considerations](#implementation-considerations)
- [Performance Analysis](#performance-analysis)
- [Optimizations for Arrays](#optimizations-for-arrays)
- [Common Use Cases](#common-use-cases)
- [Comparison with Other Searches](#comparison-with-other-searches)
- [Links](#links)

## Why Arrays are Ideal for Jump Search

Jump search was practically designed for arrays due to their O(1) random
access capability.

### Perfect Random Access

Arrays allow instant jumping to any position:

```rust
// Jump directly to index - O(1) operation
let value = array[jump_position];
```

Compare to linked list where jumping requires O(jump_size) traversal!

### Cache-Friendly Jumping

When jumping by √n:

- Each jump loads a new cache line
- Linear search phase uses cached data
- Balances random access with sequential access

```
Array: [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29]
Jump size: 4

Jumps:     ↓           ↓           ↓
Access: [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29]
           └─ cached ─┘ └─ cached ─┘ └─ cached ─┘
```

## Implementation Considerations

### 1. Calculating Jump Size

```rust
fn optimal_jump_size(n: usize) -> usize {
    (n as f64).sqrt().ceil() as usize
}
```

### 2. Boundary Handling

Arrays require careful boundary checks:

```rust
// Prevent out-of-bounds access
let curr = min(prev + jump_size - 1, array.len() - 1);
```

### 3. Last Block Handling

The last block might be smaller than jump_size:

```rust
// Linear search in final block
for i in prev..array.len() {
    if array[i] == target {
        return Some(i);
    }
}
```

## Performance Analysis

### Memory Access Pattern

Jump search on arrays creates a unique access pattern:

1. **Jump Phase**: Random access every √n elements
2. **Linear Phase**: Sequential access within block

This hybrid approach works well with modern memory hierarchies.

### Real Performance Numbers

| Array Size | Linear Search | Jump Search | Binary Search |
|------------|--------------|-------------|---------------|
| 100 | 50 comparisons | 10 comparisons | 7 comparisons |
| 10,000 | 5,000 comparisons | 100 comparisons | 14 comparisons |
| 1,000,000 | 500,000 comparisons | 1,000 comparisons | 20 comparisons |

### Cache Miss Analysis

- **Jump Phase**: ~√n cache misses
- **Linear Phase**: Usually hits cache
- **Total**: O(√n) cache misses vs O(log n) for binary search

## Optimizations for Arrays

### 1. Adaptive Jump Size

Adjust jump size based on array characteristics:

```rust
// For arrays with clustered data
if data_is_clustered {
    jump_size = (n as f64).sqrt() * 1.5;
}
```

### 2. Prefetching

Manually prefetch next jump location:

```rust
// Hint to CPU about next access
prefetch(array.as_ptr().add(next_jump));
```

### 3. Parallel Jump Search

For very large arrays, search multiple blocks in parallel:

```rust
// Split array into regions
let regions = array.chunks(region_size);
// Search regions in parallel
regions.par_iter().find_map(|region| {
    region.jump_search(&target)
})
```

## Common Use Cases

### 1. External Memory Search

When array is on disk/SSD:

- Jumping minimizes disk seeks
- Better than binary search for external storage

### 2. Network-Based Arrays

When array elements are fetched over network:

- Fewer round trips than binary search
- Can prefetch entire blocks

### 3. Sorted Logs/Time Series

```rust
// Finding entries in a time range
let start_idx = log_entries.jump_search(&start_time);
let end_idx = log_entries.jump_search(&end_time);
```

### 4. Memory-Mapped Files

Jump search works well with memory-mapped arrays:

- OS handles paging
- Jump pattern is page-friendly

## Comparison with Other Searches

### When to Use Jump Search

| Scenario | Best Algorithm | Why |
|----------|---------------|-----|
| Small array (< 100) | Linear | Overhead not worth it |
| Medium array, memory | Binary | O(log n) beats O(√n) |
| Large array, external | Jump | Fewer seeks |
| Unknown distribution | Jump | More predictable |
| Need simplicity | Jump | Easier than binary |

### Performance Trade-offs

```
Linear Search:  ████████████████████ O(n)
Jump Search:    ████████             O(√n)  
Binary Search:  ████                 O(log n)
```

## Links

### Theory and Concepts

- [Jump Search Theory](../../../algorithms/searching/jump-search/theory.md)

### Implementation

- [Code Implementation](../implementations/rust/src/algorithms/jump_search.rs)
- [Tests and Examples](../implementations/rust/src/algorithms/jump_search.rs#L285)

### Related Algorithms

- [Linear Search](./linear-search.md) - Simpler, for small arrays
- [Binary Search](./binary-search.md) - Faster for in-memory arrays
- [Interpolation Search](./interpolation-search.md) - For numeric data
