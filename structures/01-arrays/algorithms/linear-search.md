# Linear Search on Arrays

## Table of Contents

- [Why Arrays and Linear Search](#why-arrays-and-linear-search)
- [Implementation Details](#implementation-details)
- [Performance Characteristics](#performance-characteristics)
- [When Linear Search Beats Binary Search](#when-linear-search-beats-binary-search)
- [Common Use Cases](#common-use-cases)
- [Optimizations](#optimizations)
- [Links](#links)

## Why Arrays and Linear Search

While linear search is O(n), arrays make it surprisingly efficient for many
real-world scenarios due to their memory layout.

### Memory Locality Advantages

Arrays store elements contiguously, which means:

1. **CPU Cache Efficiency**: When you access one element, nearby elements
   are loaded into cache
2. **Predictable Access Pattern**: CPUs can prefetch upcoming data
3. **No Pointer Chasing**: Unlike linked lists, no indirection

```
Array in Memory:
[1][2][3][4][5][6][7][8]
└─────────────┘
  Loaded into cache together
```

### Simple is Sometimes Better

For small arrays (< 100 elements), linear search often outperforms binary
search because:

- No branching logic (better CPU pipeline utilization)
- Sequential memory access (cache-friendly)
- Less code = faster execution

## Implementation Details

### Basic Linear Search

```rust
for i in 0..array.len() {
    if array[i] == target {
        return Some(i);
    }
}
```

### Why It's Fast on Arrays

1. **Vectorization**: Modern compilers can vectorize simple loops
2. **Branch Prediction**: CPU predicts the loop will continue
3. **Prefetching**: CPU loads next cache line while processing current

## Performance Characteristics

### Real-World Performance

| Array Size | Linear Search | Binary Search | Winner |
|------------|--------------|---------------|---------|
| 10 | ~5 ns | ~15 ns | Linear |
| 100 | ~50 ns | ~35 ns | Binary |
| 1,000 | ~500 ns | ~50 ns | Binary |
| 10,000 | ~5 μs | ~70 ns | Binary |

*Note: Times are approximate and system-dependent*

### Cache Effects

Linear search benefits from:

- **Spatial Locality**: Next element is right next to current
- **Temporal Locality**: Recently accessed data stays in cache
- **Hardware Prefetching**: CPU anticipates sequential access

## When Linear Search Beats Binary Search

### 1. Small Arrays

```rust
// For n < 64, linear often wins
let small_array = [1, 2, 3, 4, 5];
// Linear: 2-3 comparisons average
// Binary: 2-3 comparisons + overhead
```

### 2. Unsorted Data

```rust
// No sorting required!
let data = [5, 2, 8, 1, 9];
data.linear_search(&8); // Works immediately
```

### 3. Finding Multiple Elements

```rust
// Find all occurrences in one pass
let indices = array.linear_search_all(&target);
```

### 4. Early Elements

If target is likely in first 10-20% of array, linear search wins even
for larger arrays.

## Common Use Cases

### 1. Validation Checks

```rust
// Check if array contains invalid values
if array.linear_search_if(|&x| x < 0).is_some() {
    return Err("Negative values not allowed");
}
```

### 2. Finding First Match

```rust
// Find first element matching condition
let first_even = array.linear_search_if(|&x| x % 2 == 0);
```

### 3. Small Lookup Tables

```rust
const KEYWORDS: [&str; 10] = ["if", "else", "while", ...];
// Linear search is perfect for small constant arrays
```

## Optimizations

### 1. Sentinel Search

Eliminates bounds checking by placing target at end:

```rust
// Place sentinel at end
array[n] = target;
let mut i = 0;
while array[i] != target {
    i += 1;
}
// Check if found before sentinel
```

### 2. Unrolled Loops

Check multiple elements per iteration:

```rust
while i + 4 <= n {
    if array[i] == target { return Some(i); }
    if array[i+1] == target { return Some(i+1); }
    if array[i+2] == target { return Some(i+2); }
    if array[i+3] == target { return Some(i+3); }
    i += 4;
}
// Handle remaining elements
```

### 3. SIMD (Advanced)

Modern CPUs can compare multiple elements simultaneously:

- AVX2: Compare 8 integers at once
- AVX-512: Compare 16 integers at once

## Links

### Theory and Concepts

- [Linear Search Theory](../../../algorithms/searching/linear-search/theory.md)

### Implementation

- [Code Implementation](../implementations/rust/src/algorithms/linear_search.rs)
- [Tests and Examples](../implementations/rust/src/algorithms/linear_search.rs#L273)

### Related Algorithms

- [Jump Search](./jump-search.md) - Faster for sorted data
- [Binary Search](./binary-search.md) - O(log n) for sorted data
