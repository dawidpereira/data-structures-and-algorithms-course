# Binary Search on Arrays

## Table of Contents

- [Why Arrays are Perfect for Binary Search](#why-arrays-are-perfect-for-binary-search)
- [Implementation Considerations](#implementation-considerations)
- [Array-Specific Optimizations](#array-specific-optimizations)
- [Common Use Cases](#common-use-cases)
- [Performance Analysis](#performance-analysis)
- [Links](#links)

## Why Arrays are Perfect for Binary Search

Arrays and binary search are a match made in heaven. Here's why:

### 1. O(1) Random Access

The fundamental requirement for binary search is the ability to jump to any
element instantly. Arrays provide this through indexing:

```rust
// With arrays, we can access any element in constant time
let mid_value = array[mid_index];  // O(1) operation
```

Compare this to linked lists where reaching the middle requires O(n/2) traversal!

### 2. Contiguous Memory = Cache-Friendly

Arrays store elements in consecutive memory locations. When binary search
accesses an element, nearby elements are loaded into CPU cache, making
subsequent accesses faster.

```
Memory Layout:
[3][7][11][15][18][21][28][30][35]
 └─────────────┘
  Loaded into cache together
```

### 3. Simple Index Arithmetic

Calculating the middle index is straightforward with arrays:

```rust
let mid = low + (high - low) / 2;
```

No pointer manipulation or traversal needed!

## Implementation Considerations

### 1. Integer Overflow Prevention

```rust
// ❌ Bad - can overflow with large indices
let mid = (low + high) / 2;

// ✅ Good - prevents overflow
let mid = low + (high - low) / 2;
```

### 2. Bounds Checking

With our custom Array<T> and DynamicArray<T>, we need to ensure indices
are within bounds:

```rust
if index >= self.len() {
    return None;  // Not found
}
```

### 3. Generic Constraints

Elements must be comparable:

```rust
impl<T: Ord> Array<T> {
    pub fn binary_search(&self, target: &T) -> Option<usize> {
        // Implementation
    }
}
```

## Array-Specific Optimizations

### 1. Interpolation Search (for uniformly distributed data)

If your array contains uniformly distributed numbers (like 1, 2, 3, 4...),
you can guess better than the middle:

```rust
// Instead of always picking middle
let mid = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low]);
```

### 2. Exponential Search (for unbounded arrays)

When you don't know the array size or searching in a specific range:

```rust
// First find bounds by exponentially increasing index
let mut bound = 1;
while bound < arr.len() && arr[bound] < target {
    bound *= 2;
}
// Then binary search within [bound/2, min(bound, len-1)]
```

### 3. Block Search (for very large arrays)

Divide array into blocks and search block-wise first, then within the block.

## Common Use Cases

### 1. Finding Elements

```rust
let numbers = Array::from_slice(&[1, 3, 5, 7, 9], 10).unwrap();
if let Some(index) = numbers.binary_search(&5) {
    println!("Found at index {}", index);
}
```

### 2. Finding Insert Position

```rust
// Where should we insert 6 to maintain sorted order?
let pos = numbers.binary_search_insertion_point(&6);
// Returns index 3 (between 5 and 7)
```

### 3. Range Queries

```rust
// Find all elements between 10 and 50
let start = array.binary_search_first_gte(&10);
let end = array.binary_search_last_lte(&50);
```

## Performance Analysis

### Memory Access Patterns

Binary search on arrays has excellent cache locality in the final steps:

```
Iteration 1: Access index 500 (cache miss likely)
Iteration 2: Access index 250 or 750 (cache miss likely)
...
Final iterations: Accessing nearby elements (cache hits!)
```

### Comparison with Other Structures

| Structure | Binary Search Time | Random Access | Cache Performance |
|-----------|-------------------|---------------|------------------|
| Array | O(log n) | O(1) | Excellent |
| Linked List | Not practical | O(n) | Poor |
| B-Tree | O(log n) | O(log n) | Good |
| Skip List | O(log n) | O(log n) | Fair |

### When NOT to Use Binary Search on Arrays

1. **Small arrays** (< 100 elements) - Linear search might be faster due to
   better cache usage and no branching
2. **Frequently changing data** - Maintaining sorted order is expensive
3. **Non-uniform access patterns** - If you mostly access recent elements

## Links

### Theory and Concepts

- [Visual Diagrams](../../../algorithms/searching/binary-search/diagrams/)

### Implementation

- [Code Implementation](../implementations/rust/src/algorithms/binary_search.rs)
- [Tests and Examples](../implementations/rust/src/algorithms/binary_search.rs)

### Exercises

- [Binary Search Exercises](../exercises/rust/src/binary_search_exercise.rs)
- [Implementation Guide](../exercises/rust/docs/binary-search-guide.md)
