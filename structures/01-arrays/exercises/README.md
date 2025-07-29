# Array Implementation Exercises

## Overview

This directory contains hands-on exercises for implementing array data structures
from scratch. Each exercise includes a skeleton implementation with comprehensive
tests to verify correctness.

## Structure

```
exercises/
└── rust/
    ├── src/
    │   ├── fixed_array_exercise.rs    # Fixed-size array exercise
    │   └── dynamic_array_exercise.rs  # Dynamic array exercise
    └── Cargo.toml
```

## Getting Started

1. **Navigate to the Rust exercises:**

   ```bash
   cd rust
   ```

2. **Run tests to see what needs implementing:**

   ```bash
   cargo test
   ```

   All tests will fail initially - your goal is to make them pass!

3. **Start with the fixed array exercise:**
   - Open `src/fixed_array_exercise.rs`
   - Look for `TODO` comments
   - Implement each method following the hints
   - Run tests frequently: `cargo test fixed_array`

4. **Move on to dynamic arrays:**
   - Complete fixed arrays first (builds foundation)
   - Open `src/dynamic_array_exercise.rs`
   - Focus on growth/shrink logic
   - Test with: `cargo test dynamic_array`

## Exercise Progression

### 1. Fixed Array (Recommended First)

**Key concepts:**

- Manual memory allocation
- Pointer arithmetic
- Bounds checking
- Drop trait for cleanup

**Methods to implement:**

- `new()` - Allocate memory
- `push()` / `pop()` - Add/remove elements
- `get()` / `get_mut()` - Access elements
- `set()` - Replace values
- `Drop` - Prevent memory leaks

### 2. Dynamic Array (After Fixed Array)

**Additional concepts:**

- Growth strategies (doubling)
- Memory reallocation
- Smart shrinking
- Iterator pattern

**New methods:**

- `grow()` - Double capacity when full
- `shrink_to_fit()` - Reclaim unused memory
- `IntoIterator` - Consume array as iterator
- `Extend` - Build from iterators

## Tips for Success

1. **Read the hints carefully** - Each TODO has specific guidance

2. **Use the documentation:**
   - Implementation guides are in the `rust/docs/` folder
   - Concept explanations are in `implementations/rust/docs/`

3. **Test incrementally:**

   ```bash
   # Test specific methods
   cargo test test_new
   cargo test test_push_and_pop
   ```

4. **Check compiler errors:**

   ```bash
   cargo check
   ```

5. **Compare with reference:**
   - After completing, compare with `implementations/rust/src/`
   - Look for optimization opportunities

## Common Pitfalls

1. **Forgetting to check allocation success** - Always check for null pointers
2. **Not dropping old values in set()** - Causes memory leaks for String, Vec, etc.
3. **Reading uninitialized memory** - Only access indices < len
4. **Wrong deallocation layout** - Must match allocation exactly
5. **Iterator double-free** - Use `mem::forget()` correctly

## Verification

Your implementation is correct when:

- All tests pass: `cargo test`
- No memory leaks (tests include Drop verification)
- No undefined behavior (bounds checks work)
- Performance is reasonable (growth/shrink strategies work)

## Next Steps

After completing these exercises:

1. Implement additional methods (e.g., `insert`, `remove`)
2. Add your own test cases
3. Benchmark against `std::vec::Vec`
4. Try implementing other data structures

