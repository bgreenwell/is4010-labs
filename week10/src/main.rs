// Week 10: Mastering ownership and borrowing
//
// This lab has two parts:
//
// PART 1: borrow checker puzzles (7 problems)
//   Each problem is a function that has a compile error related to ownership or
//   borrowing. Read the comment above each one, fix the bug, then uncomment the
//   call in main() to verify it runs.
//
// PART 2: implementation exercises (5 functions)
//   Write functions that demonstrate correct ownership/borrowing patterns.
//   The test suite at the bottom verifies your implementations.
//
// Run: cargo test

fn main() {
    println!("Week 10: Mastering ownership and borrowing");
    println!("Uncomment one problem at a time and fix it!\n");

    // Uncomment problems one at a time after fixing them:
    // problem_1();
    // problem_2();
    // problem_3();
    // problem_4();
    // problem_5();
    // problem_6();
    // problem_7();
}

// ============================================================================
// PROBLEM 1: Value used after move
// ============================================================================
// Error: This function tries to use a value after ownership has moved.
// Fix: Change calculate_length to borrow instead of taking ownership.
//
// Learning goal: Understand move semantics and when to use references.
// ============================================================================
/*
fn problem_1() {
    println!("Problem 1: Value used after move");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("  The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
*/

// ============================================================================
// PROBLEM 2: Immutable and mutable borrow conflict
// ============================================================================
// Error: Tries to create a mutable borrow while an immutable borrow exists.
// Fix: Ensure immutable borrows are no longer used before creating a mutable borrow.
//
// Learning goal: Understand the "one mutable OR many immutable" rule.
// ============================================================================
/*
fn problem_2() {
    println!("Problem 2: Mutable and immutable borrow conflict");
    let mut s = String::from("hello");
    let r1 = &s;      // immutable borrow
    let r2 = &mut s;  // mutable borrow, ERROR!
    println!("  {}, {}", r1, r2);
}
*/

// ============================================================================
// PROBLEM 3: Mutating through an immutable reference
// ============================================================================
// Error: Tries to mutate a value through an immutable reference.
// Fix: Change both the variable declaration and function signature to use &mut.
//
// Learning goal: Know when to use &String vs &mut String.
// ============================================================================
/*
fn problem_3() {
    println!("Problem 3: Mutating through an immutable reference");
    let s = String::from("hello");
    add_to_string(&s);
    println!("  Result: {}", s);
}

fn add_to_string(s: &String) {
    s.push_str(", world");
}
*/

// ============================================================================
// PROBLEM 4: Multiple mutable borrows
// ============================================================================
// Error: Creates two mutable references to the same data simultaneously.
// Fix: Use scopes to limit the lifetime of the first mutable borrow.
//
// Learning goal: Control borrow lifetimes with scopes.
// ============================================================================
/*
fn problem_4() {
    println!("Problem 4: Multiple mutable borrows");
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // ERROR: can't have two mutable borrows at once!

    println!("  {}, {}", r1, r2);
}
*/

// ============================================================================
// PROBLEM 5: Dangling reference
// ============================================================================
// Error: Returns a reference to data that will be dropped at end of scope.
// Fix: Return the owned String instead of a reference.
//
// Learning goal: Prevent use-after-free bugs.
// ============================================================================
/*
fn problem_5() {
    println!("Problem 5: Dangling reference");
    let r = create_string();
    println!("  Got: {}", r);
}

fn create_string() -> &String {
    let s = String::from("hello");
    &s // ERROR: returning reference to local variable
}
*/

// ============================================================================
// PROBLEM 6: Ownership in loops
// ============================================================================
// Error: Tries to move a value multiple times in a loop.
// Fix: Pass a reference instead of transferring ownership.
//
// Learning goal: Understand ownership with iteration.
// ============================================================================
/*
fn problem_6() {
    println!("Problem 6: Ownership in loops");
    let data = String::from("Rust");

    for i in 0..3 {
        print_with_number(data, i); // ERROR: moves `data` on first iteration
    }
}

fn print_with_number(s: String, n: i32) {
    println!("  {}: {}", n, s);
}
*/

// ============================================================================
// PROBLEM 7: lifetime, reference doesn't live long enough
// ============================================================================
// Error: The reference outlives the data it points to.
// Fix: Move the String declaration outside the inner scope.
//
// Learning goal: Understand scope and lifetime relationships.
// ============================================================================
/*
fn problem_7() {
    println!("Problem 7: Lifetime extension");
    let result;
    {
        let s = String::from("inner scope");
        result = &s; // ERROR: `s` is dropped at end of this block
    }
    println!("  Result: {}", result);
}
*/

// ============================================================================
// PART 2: implementation exercises
//
// Write these functions from scratch using correct ownership/borrowing.
// The tests below verify your implementations.
// ============================================================================

/// Takes ownership of a String, converts it to uppercase, and returns it.
///
/// Demonstrates: move in, transform, move out ("consume and return" pattern).
pub fn to_uppercase_owned(_s: String) -> String {
    todo!("Implement to_uppercase_owned, hint: .to_uppercase()")
}

/// Borrows a String immutably and returns its length.
///
/// Demonstrates: read-only borrowing.
#[allow(clippy::ptr_arg)]
pub fn string_length(_s: &String) -> usize {
    todo!("Implement string_length, hint: .len()")
}

/// Borrows a String mutably and appends `suffix` to it in place.
///
/// Demonstrates: in-place mutation through a mutable borrow.
pub fn append_suffix(_s: &mut String, _suffix: &str) {
    todo!("Implement append_suffix, hint: .push_str()")
}

/// Creates a new owned String by concatenating two borrowed string slices.
///
/// Demonstrates: producing owned data from borrowed inputs.
pub fn concat_strings(_s1: &str, _s2: &str) -> String {
    todo!("Implement concat_strings, hint: format!() or String::from() + push_str()")
}

// ============================================================================
// TESTS: DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase_owned() {
        let s = String::from("hello");
        let result = to_uppercase_owned(s);
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_to_uppercase_owned_already_upper() {
        let s = String::from("RUST");
        assert_eq!(to_uppercase_owned(s), "RUST");
    }

    #[test]
    fn test_string_length() {
        let s = String::from("testing");
        let len = string_length(&s);
        assert_eq!(len, 7);
        // Original string must still be usable after the borrow.
        assert_eq!(s, "testing");
    }

    #[test]
    fn test_string_length_empty() {
        let s = String::from("");
        assert_eq!(string_length(&s), 0);
    }

    #[test]
    fn test_append_suffix() {
        let mut s = String::from("hello");
        append_suffix(&mut s, ", world");
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_append_suffix_empty() {
        let mut s = String::from("");
        append_suffix(&mut s, "hi");
        assert_eq!(s, "hi");
    }

    #[test]
    fn test_concat_strings() {
        let result = concat_strings("hello", " world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_concat_strings_empty() {
        assert_eq!(concat_strings("", "abc"), "abc");
        assert_eq!(concat_strings("abc", ""), "abc");
    }
}
