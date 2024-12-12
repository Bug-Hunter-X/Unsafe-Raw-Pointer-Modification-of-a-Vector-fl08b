# Unsafe Raw Pointer Modification of a Vector in Rust

This repository demonstrates a potential issue with modifying a Rust vector using unsafe raw pointers.  Improper use can lead to memory unsafety and unexpected behavior.

The `bug.rs` file contains code that directly manipulates the vector's underlying memory using `as_mut_ptr()`.  This is generally discouraged unless absolutely necessary and with extreme caution.

The `bugSolution.rs` demonstrates a safer alternative, showcasing Rust's ownership and borrowing system to handle vector modifications.

## How to Reproduce

1. Clone this repository.
2. Navigate to the directory.
3. Compile and run `bug.rs` (expect a potential runtime error or unexpected output).
4. Compile and run `bugSolution.rs` (expect correct output).

## Lessons Learned

- Avoid using raw pointers unless absolutely necessary for performance-critical sections of your code.
- Utilize Rust's safe abstractions (e.g., borrowing, ownership) to avoid memory management issues.
- Always carefully consider the implications of unsafe code and ensure memory safety.