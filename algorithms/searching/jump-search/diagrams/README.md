# Jump Search Visual Diagrams

This directory contains visual representations of the jump search algorithm
to help understand how it works.

## Available Diagrams

### 1. Jump Search Process (jump-search-process.png)

Shows the two-phase process:

- Jump phase: Skipping ahead by √n steps
- Linear phase: Sequential search within the identified block

### 2. Optimal Jump Size (jump-size-calculation.png)

Illustrates why √n is the mathematically optimal jump size, showing the
trade-off between jump phase and linear phase comparisons.

### 3. Jump Search vs Others (jump-search-comparison.png)

Visual comparison showing how jump search performance (O(√n)) sits between
linear search (O(n)) and binary search (O(log n)).

## How to Use These Diagrams

1. **Study the process diagram** to understand the two-phase approach
2. **Review jump size calculation** to understand the optimization
3. **Compare with other algorithms** to know when to use jump search

## Creating New Diagrams

If you want to create additional diagrams:

1. Use Excalidraw for consistency
2. Follow the style guide in the project root
3. Export as PNG with transparent background
4. Name descriptively (e.g., `jump-search-external-memory.png`)

