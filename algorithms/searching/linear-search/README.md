# Linear Search Algorithm

## Table of Contents

- [Overview](#overview)
- [When to Use](#when-to-use)
- [Prerequisites](#prerequisites)
- [Learning Path](#learning-path)
- [Quick Reference](#quick-reference)

## Overview

Linear search is like looking for your keys by checking every possible location
one by one - the kitchen counter, coffee table, jacket pockets, and so on. It's
the simplest search algorithm that works by checking each element sequentially
until finding the target or reaching the end.

## When to Use

Linear search is your best choice when:

- **Data is unsorted** - No preprocessing required
- **Small datasets** - For < 100 elements, often faster than complex algorithms
- **One-time searches** - No need to sort first
- **Finding all occurrences** - Must check everything anyway
- **Linked lists** - Sequential access is the only option

## Prerequisites

Before diving into linear search, ensure you understand:

- Basic programming concepts (loops, conditions)
- [Arrays](../../../structures/01-arrays/) - Primary data structure used
- [Big O Notation](../../../structures/00-fundamentals/big-o-notation.md) - For complexity analysis

## Learning Path

1. **[Read the Theory](./theory.md)** - Comprehensive explanation with examples
2. **[Study Visual Diagrams](./diagrams/)** - See the algorithm in action
3. **[Explore Implementations](../../../structures/01-arrays/implementations/rust/src/algorithms/linear_search.rs)** - See real code
4. **[Complete Exercises](../../../structures/01-arrays/exercises/rust/src/search_algorithms_exercise.rs)** - Practice yourself

## Quick Reference

| Aspect | Value |
|--------|-------|
| Time Complexity | O(n) |
| Space Complexity | O(1) |
| Best Case | O(1) - element at first position |
| Worst Case | O(n) - element at end or not present |
| Requirements | None - works on any data |

### When NOT to Use Linear Search

- Large sorted datasets (use binary search)
- Frequent searches on same data (sort first, then binary search)
- When O(n) is too slow for requirements
- When data structure provides better search (hash tables, trees)