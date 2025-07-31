# Jump Search Algorithm

## Table of Contents

- [Overview](#overview)
- [When to Use](#when-to-use)
- [Prerequisites](#prerequisites)
- [Learning Path](#learning-path)
- [Quick Reference](#quick-reference)

## Overview

Jump search is like checking every 10th page in a dictionary until you pass your
target word, then going back and checking each page in that section. It combines
the simplicity of linear search with the efficiency of skipping ahead, making it
faster than linear search but simpler than binary search.

## When to Use

Jump search is your best choice when:

- **Data is sorted** - Required for the algorithm to work
- **External memory access** - Minimizes disk/network seeks
- **Medium-sized datasets** - When O(√n) is acceptable
- **Sequential access preferred** - Better cache locality than binary search
- **Simplicity matters** - Easier to implement correctly than binary search

## Prerequisites

Before diving into jump search, ensure you understand:

- [Linear Search](../linear-search/) - Foundation for the linear phase
- [Big O Notation](../../../structures/00-fundamentals/big-o-notation.md) - For complexity analysis
- [Arrays](../../../structures/01-arrays/) - Primary data structure used
- Basic mathematics (square root concept)

## Learning Path

1. **[Read the Theory](./theory.md)** - Comprehensive explanation with examples
2. **[Study Visual Diagrams](./diagrams/)** - See the algorithm in action
3. **[Explore Implementations](../../../structures/01-arrays/implementations/rust/src/algorithms/jump_search.rs)** - See real code
4. **[Complete Exercises](../../../structures/01-arrays/exercises/rust/src/search_algorithms_exercise.rs)** - Practice yourself

## Quick Reference

| Aspect | Value |
|--------|-------|
| Time Complexity | O(√n) |
| Space Complexity | O(1) |
| Best Case | O(1) - element at first position |
| Worst Case | O(√n) - element at end or not present |
| Optimal Jump Size | √n |
| Requirements | Sorted data, random access |

### When NOT to Use Jump Search

- Unsorted data (requires O(n log n) sort first)
- Very large in-memory arrays (binary search is faster)
- Linked lists (no efficient random access)
- When O(log n) performance is required