# What Are Data Structures?

## Table of Contents

- [Introduction](#introduction)
- [Real-World Analogies](#real-world-analogies)
  - [Array = Parking Lot](#array--parking-lot)
  - [Stack = Stack of Plates](#stack--stack-of-plates)
  - [Queue = Line at Coffee Shop](#queue--line-at-coffee-shop)
  - [HashMap = Phone Book](#hashmap--phone-book)
- [Why Different Structures?](#why-different-structures)
  - [Scenario 1: Frequent Access by Position](#scenario-1-frequent-access-by-position)
  - [Scenario 2: Frequent Insertions/Deletions at Front](#scenario-2-frequent-insertionsdeletions-at-front)
  - [Scenario 3: Checking Membership](#scenario-3-checking-membership)
- [The Big Picture: Time vs Space](#the-big-picture-time-vs-space)
- [Why This Matters](#why-this-matters)
- [Practice Exercise](#practice-exercise)
- [Key Takeaways](#key-takeaways)

## Introduction

Imagine you're organizing a library. You could throw all the books in a pile,
but finding a specific book would be a nightmare. Instead, you organize them
maybe alphabetically, by genre, or by publication date. Each organization
method has trade-offs: alphabetical makes finding by title easy but finding
all mysteries hard.

Data structures are the computer science equivalent, they're ways of organizing
data in memory so we can work with it efficiently. Just like organizing
a library, different structures excel at different tasks.

## Real-World Analogies

Let's connect data structures to things you already understand:

### Array = Parking Lot

```rust
// An array is like numbered parking spaces
let parking_lot: [&str; 5] = ["Car", "Truck", "Bike", "Empty", "Van"];

// Finding car in space 2 is instant - O(1)
println!("Space 2 has: {}", parking_lot[2]); // "Bike"

// But finding where the Van is requires checking each space - O(n)
for (space, vehicle) in parking_lot.iter().enumerate() {
    if vehicle == &"Van" {
        println!("Van found in space {}", space);
    }
}
```

### Stack = Stack of Plates

```rust
// A stack follows Last-In-First-Out (LIFO)
let mut plate_stack = Vec::new();

// Adding plates (push)
plate_stack.push("Blue plate");
plate_stack.push("Red plate");
plate_stack.push("Green plate");

// You can only take from the top (pop)
println!("Taking: {:?}", plate_stack.pop()); // Some("Green plate")
```

### Queue = Line at Coffee Shop

```rust
use std::collections::VecDeque;

// A queue follows First-In-First-Out (FIFO)
let mut coffee_line = VecDeque::new();

// People join the line (enqueue)
coffee_line.push_back("Alice");
coffee_line.push_back("Bob");
coffee_line.push_back("Charlie");

// First person gets served first (dequeue)
println!("Serving: {:?}", coffee_line.pop_front()); // Some("Alice")
```

### HashMap = Phone Book

```rust
use std::collections::HashMap;

// A HashMap provides key-value lookups
let mut phone_book = HashMap::new();

// Adding entries
phone_book.insert("Alice", "555-1234");
phone_book.insert("Bob", "555-5678");
phone_book.insert("Charlie", "555-9012");

// Looking up is fast - O(1) average
if let Some(number) = phone_book.get("Bob") {
    println!("Bob's number: {}", number);
}
```

Don't worry if you don't fully understand all these structures yet. In the
following sections, we'll dive deep into each one, exploring how they work
internally, when to use them, and how to implement them from scratch.

## Why Different Structures?

Each data structure makes different trade-offs. Consider these scenarios:

### Scenario 1: Frequent Access by Position

```rust
// Array wins! Direct indexing is O(1)
let scores = [95, 87, 92, 78, 88];
println!("Third score: {}", scores[2]); // Instant access
```

### Scenario 2: Frequent Insertions/Deletions at Front

```rust
use std::collections::VecDeque;

// Deque wins! Operations at both ends are O(1)
let mut tasks = VecDeque::new();
tasks.push_front("Urgent task");     // O(1)
tasks.push_back("Regular task");     // O(1)
let next = tasks.pop_front();        // O(1)
```

### Scenario 3: Checking Membership

```rust
use std::collections::HashSet;

// HashSet wins! Contains check is O(1) average
let mut visited_pages = HashSet::new();
visited_pages.insert("home.html");
visited_pages.insert("about.html");

// Checking if we've visited a page
if visited_pages.contains("home.html") {
    println!("Already visited!");
}
```

## The Big Picture: Time vs Space

Every data structure decision involves trade-offs:

```rust
// Option 1: Save space, sacrifice time
fn find_duplicate_naive(numbers: &[i32]) -> Option<i32> {
    // O(n²) time, O(1) space
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] == numbers[j] {
                return Some(numbers[i]);
            }
        }
    }
    None
}

// Option 2: Use more space for faster lookup
fn find_duplicate_fast(numbers: &[i32]) -> Option<i32> {
    use std::collections::HashSet;
    // O(n) time, O(n) space
    let mut seen = HashSet::new();
    for &num in numbers {
        if !seen.insert(num) {
            return Some(num);
        }
    }
    None
}
```

## Why This Matters

Choosing the right data structure can be the difference between:

- A search taking 1 millisecond vs 10 seconds
- Using 1MB vs 1GB of memory
- Code that scales to millions of users vs crashing at thousands

## Practice Exercise

Try implementing a simple task manager using different structures:

```rust
// Task: Store tasks and mark them complete
// Try implementing with:
// 1. Vec<String> - How do you mark complete?
// 2. HashMap<String, bool> - How do you maintain order?
// 3. Your own struct - What fields would you need?

// Starter code:
#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

fn main() {
    // Your implementation here
}
```

## Key Takeaways

1. **Data structures are tools** - Like a toolbox, each has its purpose
2. **No structure is universally best** - Context determines the right choice
3. **Trade-offs exist** - Usually between time, space, and code complexity
4. **Real-world parallels help** - If you understand a queue at a store, you
   understand a queue in code

Next, we'll explore how computers actually store this data in memory,
which will help you understand why these trade-offs exist.
