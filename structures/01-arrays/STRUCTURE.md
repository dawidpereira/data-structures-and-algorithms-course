# Array Module Structure

## Overview

This module is organized to separate reference implementations, exercises, and documentation.

## Directory Structure

```
01-arrays/
├── implementations/           # Reference implementations
│   └── rust/
│       ├── src/              # Clean implementation code
│       │   ├── core.rs       # Fixed-size array
│       │   └── dynamic_array.rs  # Dynamic array
│       └── docs/             # Conceptual documentation
│           ├── memory-management.md      # Memory concepts
│           └── dynamic-array-strategies.md  # Growth strategies
│
├── exercises/                # Hands-on exercises
│   └── rust/
│       ├── src/             # Exercise skeletons with tests
│       │   ├── fixed_array_exercise.rs
│       │   └── dynamic_array_exercise.rs
│       └── docs/            # Implementation guides
│           ├── fixed-array-guide.md     # Step-by-step guide
│           └── dynamic-array-guide.md   # Step-by-step guide
│
├── theory.md                # Array theory
├── use-cases.md            # Real-world applications
└── diagrams/               # Visual explanations
```

## Learning Path

1. **Start with Theory**: Read `theory.md` to understand array concepts
2. **Study Concepts**: Read docs in `implementations/rust/docs/` for memory and strategy understanding
3. **Follow Guides**: Use guides in `exercises/rust/docs/` while implementing
4. **Code Exercises**: Implement arrays in `exercises/rust/src/`
5. **Compare**: Check your solution against `implementations/rust/src/`

## Key Benefits

- **Clean Separation**: Implementation code is uncluttered
- **Progressive Learning**: Start with exercises, compare with reference
- **Comprehensive Docs**: All documentation is close to relevant code
- **Test-Driven**: Exercises include full test suites

