// Week 12: Generics and traits
//
// Implement a generic Stack<T> data structure and make it work with Rust's
// standard Display trait.
//
// Run: cargo test

use std::fmt;

fn main() {
    println!("Week 12: Generics and traits");

    // Try it out once you have implementations working:
    // let mut s: Stack<i32> = Stack::new();
    // s.push(1);
    // s.push(2);
    // s.push(3);
    // println!("{}", s);           // requires Display impl
}

// ============================================================================
// STACK<T>: implement all methods and trait impls below.
// When you implement a method, remove the leading `_` from each parameter name.
// ============================================================================

/// A generic last-in, first-out (LIFO) stack.
///
/// The top of the stack is the last element pushed.
#[allow(dead_code)]
pub struct Stack<T> {
    data: Vec<T>,
}

#[allow(clippy::new_without_default)]
impl<T> Stack<T> {
    /// Creates a new, empty stack.
    pub fn new() -> Self {
        todo!("Implement Stack::new")
    }

    /// Pushes `item` onto the top of the stack.
    pub fn push(&mut self, _item: T) {
        todo!("Implement push")
    }

    /// Removes and returns the top item, or `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        todo!("Implement pop")
    }

    /// Returns a reference to the top item without removing it,
    /// or `None` if the stack is empty.
    pub fn peek(&self) -> Option<&T> {
        todo!("Implement peek")
    }

    /// Returns `true` if the stack contains no items.
    pub fn is_empty(&self) -> bool {
        todo!("Implement is_empty")
    }

    /// Returns the number of items in the stack.
    pub fn len(&self) -> usize {
        todo!("Implement len")
    }
}

// ============================================================================
// DISPLAY: format the stack as "[bottom, ..., top]"
//
// Example: a stack with 1 pushed first and 3 pushed last prints as "[1, 2, 3]".
// An empty stack prints as "[]".
// ============================================================================
impl<T: fmt::Debug> fmt::Display for Stack<T> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Implement Display for Stack<T>, hint: write!(f, \"[...]\") using self.data")
    }
}

// ============================================================================
// TESTS: DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // --- basic operations ---

    #[test]
    fn test_new_stack_is_empty() {
        let s: Stack<i32> = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_push_increases_len() {
        let mut s = Stack::new();
        s.push(1);
        assert_eq!(s.len(), 1);
        s.push(2);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_pop_returns_lifo_order() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut s: Stack<i32> = Stack::new();
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_peek_does_not_remove() {
        let mut s = Stack::new();
        s.push(42);
        assert_eq!(s.peek(), Some(&42));
        assert_eq!(s.len(), 1); // still there
    }

    #[test]
    fn test_peek_empty_stack() {
        let s: Stack<i32> = Stack::new();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn test_is_empty_after_pop() {
        let mut s = Stack::new();
        s.push(1);
        s.pop();
        assert!(s.is_empty());
    }

    // --- works with different types ---

    #[test]
    fn test_stack_of_strings() {
        let mut s = Stack::new();
        s.push(String::from("hello"));
        s.push(String::from("world"));
        assert_eq!(s.pop(), Some(String::from("world")));
    }

    #[test]
    fn test_stack_of_floats() {
        let mut s = Stack::new();
        s.push(1.1_f64);
        s.push(2.2_f64);
        assert_eq!(s.len(), 2);
    }

    // --- Display ---

    #[test]
    fn test_display_empty() {
        let s: Stack<i32> = Stack::new();
        assert_eq!(format!("{}", s), "[]");
    }

    #[test]
    fn test_display_with_items() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        // bottom → top, so display order is [1, 2, 3]
        assert_eq!(format!("{}", s), "[1, 2, 3]");
    }
}
