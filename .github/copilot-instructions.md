# Integer Sequences - Copilot Instructions

This is a Rust library (`#![no_std]`) for implementing integer sequences from the Online Encyclopedia of Integer Sequences (OEIS). The library provides a trait-based framework for defining and computing integer sequences with both precomputed values and formula-based computation.

## Code Standards

### Required Before Each Commit
- Run `cargo fmt` before committing any changes to ensure proper code formatting
- Run `cargo clippy` to catch common mistakes and improve code quality
- Ensure all tests pass with `cargo test`

### Development Flow
- Build: `cargo build --verbose`
- Test: `cargo test --verbose`
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Repository Structure
- `src/lib.rs`: Main library entry point, exports public API
- `src/traits.rs`: Core `IntegerSequence` trait definition
- `src/macros.rs`: Macro definitions (currently empty)
- `src/oeis/`: OEIS sequence implementations
  - Each sequence is implemented in its own file (e.g., `a000001.rs`)
  - `mod.rs`: Module exports
- `src/tester.rs`: Test utilities for validating sequence implementations

## Key Guidelines

### 1. Implementing New Sequences
When adding a new OEIS sequence:
- Create a new file in `src/oeis/` named after the OEIS sequence number (e.g., `a000002.rs`)
- Implement the `IntegerSequence` trait with all required constants:
  - `NAME`: Descriptive name of the sequence
  - `HEAD`: Array of precomputed initial values
  - `OFFSET`: Starting index for the sequence
  - `SOURCE`: URL to the OEIS page (e.g., "https://oeis.org/A000002")
  - `AUTHOR`: Original author/contributor
- Implement `formula(n: Index) -> Value` for computing arbitrary terms
- Add a test using `test_sequance_formula_matchces_head` to verify HEAD values match formula
- Export the new sequence in `src/oeis/mod.rs`

### 2. Code Organization
- Use `const fn` wherever possible for compile-time computation
- Keep implementations `#![no_std]` compatible (no standard library dependencies)
- Use `isize` for both indices and values (defined as `Index` and `Value` types)
- Write detailed comments for complex mathematical algorithms

### 3. Testing
- All sequence implementations must include a test that verifies the formula matches HEAD values
- Use the provided `tester::test_sequance_formula_matchces_head` helper function
- Tests should be marked with `#[cfg(test)]` to avoid compilation in release builds

### 4. Mathematical Accuracy
- Ensure formulas are mathematically correct and match OEIS definitions
- For complex sequences, include references to mathematical papers or OEIS comments
- Handle edge cases (n <= 0, n = 1, etc.) explicitly
- Consider performance for large values of n

### 5. Documentation
- Add doc comments for public items using `///`
- Include mathematical background where helpful
- Reference OEIS pages in comments
- Document any mathematical assumptions or limitations

## Example Sequence Implementation

```rust
/// Description of the sequence
/// https://oeis.org/AXXXXXX

pub const fn compute_term(n: crate::Index) -> crate::Value {
    // Implementation logic
    n * n  // Example
}

pub struct AXXXXXX;

impl crate::traits::IntegerSequence for AXXXXXX {
    const NAME: &str = "Sequence description";
    
    const HEAD: &[crate::Value] = &[0, 1, 4, 9, 16, 25];
    
    const OFFSET: crate::Index = 0;
    
    const SOURCE: &str = "https://oeis.org/AXXXXXX";
    
    const AUTHOR: &str = "Contributor Name";
    
    fn formula(n: crate::Index) -> crate::Value {
        compute_term(n)
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<AXXXXXX>();
}
```

## Important Notes
- This is a `no_std` library - do not use features from the Rust standard library
- All code must be const-compatible where possible for compile-time evaluation
- Sequence indices and values are `isize` to handle negative numbers and offsets
- The library uses Rust edition 2024
