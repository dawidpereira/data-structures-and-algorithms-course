# Interpolation Search on Arrays

## Table of Contents

- [Why Arrays Enable Interpolation Search](#why-arrays-enable-interpolation-search)
- [Implementation Requirements](#implementation-requirements)
- [Performance Characteristics](#performance-characteristics)
- [When Interpolation Search Excels](#when-interpolation-search-excels)
- [Real-World Applications](#real-world-applications)
- [Distribution Analysis](#distribution-analysis)
- [Links](#links)

## Why Arrays Enable Interpolation Search

Interpolation search requires two key features that arrays provide perfectly:

### 1. O(1) Random Access

The algorithm needs to jump to calculated positions instantly:

```rust
// Calculate position based on value
let pos = low + ((target - arr[low]) / (arr[high] - arr[low])) * (high - low);
// Jump directly there - only possible with arrays!
let value = arr[pos];
```

### 2. Numeric Indexing

Arrays have numeric indices that map naturally to the interpolation formula:

- Position 0 → smallest value
- Position n-1 → largest value
- Linear interpolation between them

## Implementation Requirements

### Data Must Be

1. **Sorted**: Like binary search
2. **Numeric**: Need to calculate ratios
3. **Uniformly Distributed**: For O(log log n) performance

### Type Constraints

```rust
// Elements must be convertible to/from numeric values
pub trait Interpolatable: Ord + Copy {
    fn to_f64(&self) -> f64;
}
```

## Performance Characteristics

### Best Case: Uniform Distribution

For perfectly uniform data:

```
Array: [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
Target: 70

Calculation: pos = 0 + ((70-10)/(100-10)) * 9 = 6
First probe: arr[6] = 70 ✓ Found in one step!
```

### Average Case Analysis

| Distribution | Time Complexity | Example |
|--------------|----------------|---------|
| Uniform | O(log log n) | Sequential IDs |
| Normal | O(log n) | Test scores |
| Exponential | O(n) | Network delays |
| Unknown | O(√n)* | Use adaptive version |

*With fallback to binary search

### Memory Access Pattern

Interpolation search has excellent cache behavior:

- Fewer total accesses than binary search
- Final probes are close together
- Works well with prefetching

## When Interpolation Search Excels

### 1. Database Indexes

```rust
// Searching for customer ID in range 1000000-9999999
let customer = customers.interpolation_search(&5234567);
// Likely finds in 2-3 probes vs 20 for binary search
```

### 2. Time Series Data

```rust
// Timestamps are usually uniformly distributed
let event = events.interpolation_search(&timestamp);
```

### 3. Sensor Readings

```rust
// Temperature readings over time
let reading = temperatures.interpolation_search(&target_time);
```

### 4. Financial Data

```rust
// Stock prices sampled at regular intervals
let price = prices.interpolation_search(&specific_time);
```

## Real-World Applications

### Log Analysis

When searching server logs with timestamps:

```rust
struct LogEntry {
    timestamp: u64,
    message: String,
}

// Logs are naturally sorted by time
logs.interpolation_search(&target_timestamp)
```

### Scientific Data

For uniformly sampled measurements:

```rust
// Astronomical observations at regular intervals
observations.interpolation_search(&julian_date)
```

### Geographic Data

When data is organized by coordinates:

```rust
// Weather stations sorted by latitude
stations.interpolation_search(&target_latitude)
```

## Distribution Analysis

### Checking If Data Is Suitable

```rust
fn is_uniform_distribution<T: Interpolatable>(arr: &Array<T>) -> bool {
    // Calculate expected step size
    let expected_step = (last - first) / (n - 1);
    
    // Check deviation from expected
    for i in 0..n {
        let expected = first + i * expected_step;
        let actual = arr[i];
        if deviation > threshold {
            return false;
        }
    }
    true
}
```

### Adaptive Strategy

```rust
// Automatically choose best algorithm
match analyze_distribution(&array) {
    Distribution::Uniform => array.interpolation_search(&target),
    Distribution::Normal => array.binary_search(&target),
    Distribution::Unknown => array.adaptive_search(&target),
}
```

### Performance Comparison

For array size 1,000,000:

| Algorithm | Uniform Data | Random Data | Clustered Data |
|-----------|-------------|-------------|----------------|
| Linear | 500,000 | 500,000 | 500,000 |
| Binary | 20 | 20 | 20 |
| Interpolation | 5 | 20 | 1,000+ |

## Links

### Theory and Concepts

- [Interpolation Search Theory](../../../algorithms/searching/interpolation-search/theory.md)

### Implementation

- [Code Implementation](../implementations/rust/src/algorithms/interpolation_search.rs)
- [Tests and Examples](../implementations/rust/src/algorithms/interpolation_search.rs#L385)

### Related Algorithms

- [Binary Search](./binary-search.md) - More reliable for unknown distributions
- [Jump Search](./jump-search.md) - Simpler alternative for sorted data
- [Linear Search](./linear-search.md) - When data isn't sorted
