# Interpolation Search Algorithm

## Table of Contents

- [Overview](#overview)
- [When to Use](#when-to-use)
- [Prerequisites](#prerequisites)
- [Learning Path](#learning-path)
- [Quick Reference](#quick-reference)

## Overview

Interpolation search is like looking up a name in a phone book - if you're
searching for "Smith", you don't start in the middle, you start about 3/4 of
the way through because 'S' is near the end of the alphabet. It estimates the
position of the target based on its value, making it incredibly fast for
uniformly distributed data.

## When to Use

Interpolation search is your best choice when:

- **Data is uniformly distributed** - Essential for O(log log n) performance
- **Numeric data** - Need to calculate value ratios
- **Data is sorted** - Required like binary search
- **Very large datasets** - Where O(log log n) vs O(log n) matters
- **Sequential IDs or timestamps** - Common uniform distributions

## Prerequisites

Before diving into interpolation search, ensure you understand:

- [Binary Search](../binary-search/) - Foundation algorithm
- [Big O Notation](../../../structures/00-fundamentals/big-o-notation.md) - For complexity analysis
- [Arrays](../../../structures/01-arrays/) - Primary data structure used
- Basic algebra (linear interpolation)

## Learning Path

1. **[Read the Theory](./theory.md)** - Comprehensive explanation with examples
2. **[Study Visual Diagrams](./diagrams/)** - See the algorithm in action
3. **[Explore Implementations](../../../structures/01-arrays/algorithms/interpolation-search.md)** - Implementation notes
4. **[Complete Exercises](../../../structures/01-arrays/exercises/rust/src/search_algorithms_exercise.rs)** - Practice yourself

## Quick Reference

| Aspect | Value |
|--------|-------|
| Time Complexity (uniform) | O(log log n) |
| Time Complexity (worst) | O(n) |
| Space Complexity | O(1) |
| Best Case | O(1) - perfect guess |
| Worst Case | O(n) - poor distribution |
| Requirements | Sorted numeric data, uniform distribution |

### When NOT to Use Interpolation Search

- Non-numeric data (can't interpolate strings directly)
- Non-uniform distributions (exponential, clustered)
- Small datasets (overhead not worth it)
- When worst-case O(n) is unacceptable
- Unknown data distributions