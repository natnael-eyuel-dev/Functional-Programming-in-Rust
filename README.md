# Functional Programming in Rust

This project demonstrates **Functional Programming (FP)** concepts using the **Rust programming language**.

Functional programming is a programming paradigm where computation is treated as the evaluation of **mathematical functions**, and **state and side effects are minimized**. Rust's type system, iterators, and closures make it well-suited for exploring these concepts.

---

## Key Principles Illustrated in the Class

1. **Pure Functions** – Functions always return the same output for the same input and have no side effects.
   *(Example: [`pure_function_topic`](./src/topics/pure_function_topic.rs))*

2. **Immutability** – Data is not changed; new data is returned instead.

3. **Higher-Order Functions** – Functions that can take other functions as arguments or return functions.
   *(Examples: [`closure_topic`](./src/topics/closure_topic.rs), [`lambda_topic`](./src/topics/lambda_topic.rs), [`function_composition_topic`](./src/topics/function_composition_topic.rs))*

4. **Function Composition** – Combining small functions to create new behavior.
   *(Examples: [`function_composition_topic`](./topics/function_composition_topic.rs), [`partial_composition_topic`](./src/topics/partial_composition_topic.rs))*

5. **Recursion** – Using function calls instead of loops to perform repetitive computations.
   *(Example: [`recursion_topic`](./src/topics/recursion_topic.rs))*

6. **Lazy Evaluation** – Computations are delayed until their results are needed.
   *(Example: [`lazy_evaluation_topic`](./src/topics/lazy_evaluation_topic.rs))*

7. **Mapping and Reducing** – Transforming and aggregating collections.
   *(Examples: [`map_topic`](./src/topics/map_topic.rs), [`sum_topic`](./src/topics/sum_topic.rs), [`list_comprehension_topic`](./src/topics/list_comprehension_topic.rs))*

8. **Pattern Matching & Enums** – Handling multiple possible states cleanly.
   *(Examples: [`enum_topic`](./src/topics/enum_topic.rs), [`pattern_matching_topic`](./src/topics/pattern_matching_topic.rs), [`switch_topic`](./src/topics/switch_topic.rs))*

9. **Currying** – Transforming multi-argument functions into a sequence of single-argument functions.
   *(Example: [`currying_topic`](./src/topics/currying_topic.rs))*

---

## Project Structure

```
- [main.rs](src/main.rs)
- topics
  - [builder_pattern_topic.rs](src/topics/builder_pattern_topic.rs)
  - [closure_topic.rs](src/topics/closure_topic.rs)
  - [currying_topic.rs](src/topics/currying_topic.rs)
  - [enum_topic.rs](src/topics/enum_topic.rs)
  - [function_composition_topic.rs](src/topics/function_composition_topic.rs)
  - [functional_programming.rs](src/topics/functional_programming.rs)
  - [lambda_topic.rs](src/topics/lambda_topic.rs)
  - [lazy_evaluation_topic.rs](src/topics/lazy_evaluation_topic.rs)
  - [list_comprehension_topic.rs](src/topics/list_comprehension_topic.rs)
  - [map_topic.rs](src/topics/map_topic.rs)
  - [partial_composition_topic.rs](src/topics/partial_composition_topic.rs)
  - [pattern_matching_topic.rs](src/topics/pattern_matching_topic.rs)
  - [pure_function_topic.rs](src/topics/pure_function_topic.rs)
  - [recursion_topic.rs](src/topics/recursion_topic.rs)
  - [sum_topic.rs](src/topics/sum_topic.rs)
  - [switch_topic.rs](src/topics/switch_topic.rs)

```

* Each file corresponds to a topic discussed in class.
* `main.rs` demonstrates all examples together.

---

## How to Run

1. Ensure you have Rust installed: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Clone the repository:

```bash
git clone <repository-link>
cd functional-programming-rust
```

3. Run the project:

```bash
cargo run
```

4. You will see outputs for all functional programming examples in sequence.

---

## Learning Outcomes

By exploring this project, you will understand:

* How to write **pure, immutable functions** in Rust
* Using **higher-order functions, closures, and lambdas**
* Combining functions using **composition and currying**
* Implementing **recursion, lazy evaluation, and collection transformations**
* Using **pattern matching, enums, and switch-like logic** effectively

This project serves as a practical guide for learning **functional programming concepts** in Rust in a structured and easy-to-understand way.

