# Interpolation Search Visual Diagrams

This directory contains visual representations of the interpolation search
algorithm to help understand how it works.

## Available Diagrams

### 1. Interpolation Formula (interpolation-formula.png)

Shows how the position is calculated based on the target value, using linear
interpolation between the low and high bounds.

### 2. Uniform vs Non-Uniform Distribution (distribution-effects.png)

Illustrates how data distribution affects interpolation search performance:

- Uniform distribution: O(log log n)
- Non-uniform distribution: Can degrade to O(n)

### 3. Interpolation Search Process (interpolation-search-process.png)

Step-by-step visualization showing how the algorithm estimates positions and
narrows the search space more efficiently than binary search for uniform data.

## How to Use These Diagrams

1. **Understand the formula** - Key to how interpolation search works
2. **Study distribution effects** - Critical for knowing when to use it
3. **Follow the process** - See how it differs from binary search

## Creating New Diagrams

If you want to create additional diagrams:

1. Use Excalidraw for consistency
2. Follow the style guide in the project root
3. Export as PNG with transparent background
4. Name descriptively (e.g., `interpolation-search-phone-book.png`)

