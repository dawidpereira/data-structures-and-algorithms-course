# Binary Search Algorithm

## Table of Contents

- [Overview](#overview)
- [When to Use](#when-to-use)
- [Prerequisites](#prerequisites)
- [Learning Path](#learning-path)
- [Quick Reference](#quick-reference)

## Overview

Binary search is like finding a specific page in a phone book - instead of
flipping through every page, you open to the middle and decide which half
contains your target. By repeatedly dividing the search space in half, you
can find any item in a sorted collection remarkably quickly.

## When to Use

Binary search is your best choice when:

- **Data is sorted** - This is mandatory
- **Need fast lookups** - O(log n) vs O(n) for linear search
- **Have random access** - Can jump to any position (arrays, not linked lists)
- **Searching frequently** - The sorting cost is amortized over many searches

## Prerequisites

Before diving into binary search, ensure you understand:

- [Big O Notation](../../../structures/00-fundamentals/big-o-notation.md) - For complexity analysis
- [Arrays](../../../structures/01-arrays/) - Primary data structure used
- Basic recursion concepts (optional - can be implemented iteratively)

## Learning Path

1. **[Read the Theory](./theory.md)** - Comprehensive explanation with examples
2. **[Study Visual Diagrams](./diagrams/)** - See the algorithm in action
3. **[Explore Implementations](../../../structures/01-arrays/implementations/rust/src/algorithms/binary_search.rs)** - See real code
4. **[Complete Exercises](../../../structures/01-arrays/exercises/rust/src/binary_search_exercise.rs)** - Practice yourself

## Quick Reference

| Aspect | Value |
|--------|-------|
| Time Complexity | O(log n) |
| Space Complexity | O(1) iterative, O(log n) recursive |
| Best Case | O(1) - element at middle |
| Worst Case | O(log n) - element at end or not present |
| Requirements | Sorted data, random access |

### When NOT to Use Binary Search

- Unsorted data (would need O(n log n) sort first)
- Linked lists (no efficient random access)
- Small datasets (overhead not worth it)
- Data that changes frequently (maintaining sort is expensive)
