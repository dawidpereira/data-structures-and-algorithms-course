# Memory Management in Arrays

## Prerequisites

- [Array Theory](../../../theory.md) - Basic understanding of arrays
- Basic Rust knowledge (ownership, borrowing)
- Understanding of stack vs heap

## Table of Contents

- [Introduction](#introduction)
- [Stack vs Heap Memory](#stack-vs-heap-memory)
- [Memory Layout](#memory-layout)
- [Alignment](#alignment)
- [Uninitialized Memory](#uninitialized-memory)
- [Memory Safety Rules](#memory-safety-rules)

## Introduction

This guide explains how arrays work at the lowest level, focusing on manual memory
management without relying on built-in array types. Understanding these concepts
is crucial for implementing custom data structures.

## Stack vs Heap Memory

### Stack Memory

- **Speed**: Very fast access
- **Management**: Automatic (freed when function returns)
- **Size**: Limited (typically 1-8MB)
- **Use case**: Local variables, function parameters

### Heap Memory

- **Speed**: Slower than stack (requires allocation)
- **Management**: Manual (must explicitly free)
- **Size**: Much larger (limited by available RAM)
- **Use case**: Dynamic data structures, large allocations

### Why Arrays Use Heap Memory

Our array implementation allocates on the heap because:

1. **Runtime Size**: Array size can be determined at runtime
2. **Persistence**: Data survives beyond function scope
3. **Large Data**: Can store more data than stack allows

## Memory Layout

Think of computer memory as a massive apartment building. An array is like
renting an entire floor where all apartments are:

- The same size (each element has the same type)
- Numbered sequentially (indices 0, 1, 2...)
- Right next to each other (contiguous memory)

When you create an array of 10 integers, you're essentially saying "I need
10 consecutive apartments, each big enough for one integer."

### Direct Access Through Address Calculation

Just like finding apartment 305 (3rd floor, 5th apartment), accessing array
elements is simple math:

```
Memory address = Starting address + (Index × Element size)
```

If your array starts at memory address 1000 and each integer takes 4 bytes:

- Element 0: Address 1000
- Element 1: Address 1004
- Element 2: Address 1008
- And so on...

This direct calculation is why arrays have O(1) access time - no searching
needed!

### Memory Allocation Requirements

When allocating memory, we need to specify two critical properties:

### 1. Size

Total bytes needed = capacity × size_of::<T>()

### 2. Alignment

Memory address requirements for efficient CPU access

### Examples

**i32 array with capacity 5:**

```
Size: 5 × 4 bytes = 20 bytes
Alignment: 4 (address must be divisible by 4)
Memory layout: [i32][i32][i32][i32][i32]
```

**u8 array with capacity 10:**

```
Size: 10 × 1 byte = 10 bytes
Alignment: 1 (any address works)
Memory layout: [u8][u8][u8][u8][u8][u8][u8][u8][u8][u8]
```

## Alignment

Proper alignment is crucial for CPU performance:

### Good Alignment (i32 requires 4-byte alignment)

```
0x1000: [i32] ✓ (address divisible by 4)
0x1004: [i32] ✓ (address divisible by 4)
0x1008: [i32] ✓ (address divisible by 4)
```

### Bad Alignment

```
0x1001: [i32] ✗ (not divisible by 4)
```

Misaligned access can cause:

- Performance penalties (2-3x slower)
- Hardware exceptions on some architectures
- Undefined behavior

## Uninitialized Memory

When we allocate memory, it contains whatever data was previously there:

```
Just allocated array<i32, 4>:
Index:    [0]      [1]      [2]      [3]
Values: [842150] [-23847] [0]      [918273]
           ^        ^        ^        ^
        garbage  garbage  garbage  garbage
```

### Critical Rule

**Never read uninitialized memory!** This is undefined behavior and can:

- Return random values
- Crash your program
- Cause security vulnerabilities

This is why we track `len` separately from `capacity`:

- `capacity`: Total allocated space
- `len`: Number of initialized elements
- Safe to read: indices 0..len only

## Memory Safety Rules

### 1. Write Before Read

Always initialize memory before accessing it:

```rust
// ✗ WRONG: Reading uninitialized memory
let value = unsafe { *ptr };

// ✓ CORRECT: Write first, then read
unsafe { 
    ptr.write(42);
    let value = ptr.read();
}
```

### 2. Track Valid Data

Use `len` to track which elements are initialized:

```rust
// Only elements at indices 0..len are safe to read
if index < self.len {
    unsafe { Some(&*self.ptr.add(index)) }
} else {
    None  // Out of bounds or uninitialized
}
```

### 3. Proper Cleanup

Implement Drop to:

1. Drop all initialized elements (to free their resources)
2. Deallocate the memory block

### 4. Bounds Checking

Never access memory outside your allocation:

```rust
// Allocation: 10 elements
// Valid indices: 0..10
// Accessing index 10 or higher = undefined behavior
```

## Common Pitfalls

For a comprehensive list of pitfalls and their solutions, see
[Common Array Implementation Pitfalls](./common-pitfalls.md).

Key memory-related pitfalls:

1. Forgetting to drop old values when overwriting
2. Reading past `len` into uninitialized memory
3. Not checking allocation success
4. Incorrect layout calculations
5. Missing Drop implementation

## Best Practices

1. Always validate indices against `len`, not `capacity`
2. Use `ptr::write` for uninitialized memory, not assignment
3. Check for null after allocation
4. Implement Drop to prevent leaks
5. Use `PhantomData<T>` to indicate ownership
6. Document safety requirements clearly

## Next Steps

With memory management understanding:

1. **[Fixed Array Implementation](../../../exercises/rust/docs/fixed-array-guide.md)** - Build your first array
2. **[Dynamic Array Strategies](./dynamic-array-strategies.md)** - Learn about growable arrays
3. **[Dynamic Array Implementation](../../../exercises/rust/docs/dynamic-array-guide.md)** - Build a growable array
