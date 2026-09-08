//! List and singly linked-list warm-ups.
//!
//! Run the exercises with:
//!
//! ```text
//! cargo test
//! ```
//!
//! The exercise tests are `#[ignore]` until you implement the corresponding
//! function. Run them explicitly with `cargo test -- --ignored` while working.

/// A node in a singly linked list.
///
/// `Option<Box<Node>>` means either there is no next node (`None`) or the next
/// node is owned on the heap (`Some(Box<Node>)`). `Box` gives each node a known
/// size while allowing the list to grow recursively.
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Self { value, next: None }
    }
}

/// Build a linked list from left to right.
///
/// This helper is complete so you can focus on the algorithms below. Notice
/// how `head` is moved into each new node: ownership makes the list's memory
/// management explicit and avoids a garbage collector.
pub fn from_slice(values: &[i32]) -> Option<Box<Node>> {
    values.iter().rev().fold(None, |next, &value| {
        Some(Box::new(Node { value, next }))
    })
}

/// Convert a list into a vector for easy assertions in tests.
pub fn to_vec(head: &Option<Box<Node>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head.as_deref();
    while let Some(node) = current {
        result.push(node.value);
        // `as_deref` borrows nodes without taking ownership of the list.
        current = node.next.as_deref();
    }
    result
}

/// Exercise 1: prepend a value to an existing list.
///
/// Implement this without cloning the existing list. The old `head` should be
/// moved into the new node's `next` field.
pub fn push_front(head: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
    todo!("move head into a new Node { value, next: head }")
}

/// Exercise 2: reverse a singly linked list in place.
///
/// The key ownership pattern is to repeatedly `take()` the current node's
/// `next`, redirect that pointer to `previous`, and advance both variables.
/// `take()` replaces an Option with `None`, allowing you to move the Box out
/// instead of trying to hold two mutable borrows at once.
pub fn reverse(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    todo!("use previous/current pointers and Option::take")
}

/// Exercise 3: merge two sorted lists into one sorted list.
///
/// Reuse the existing nodes rather than allocating new ones. Pattern matching
/// on `Option<Box<Node>>` moves ownership; use `as_ref()` or `as_mut()` when a
/// shared or mutable borrow is enough. Be careful not to keep a borrow alive
/// while moving the same Option.
pub fn merge_sorted(
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
) -> Option<Box<Node>> {
    todo!("compare the heads and recursively/iteratively relink nodes")
}

/// Exercise 4: remove the first node with `target`, if present.
///
/// This is a useful `Option`/borrowing exercise: walking with `&mut Option<Box<Node>>`
/// lets you splice a node out by replacing its slot with its successor.
pub fn remove_first(head: &mut Option<Box<Node>>, target: i32) -> bool {
    todo!("walk through mutable links and splice out the matching node")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers_build_and_read_lists() {
        assert_eq!(to_vec(&from_slice(&[1, 2, 3])), vec![1, 2, 3]);
        assert_eq!(to_vec(&from_slice(&[])), Vec::<i32>::new());
    }

    #[test]
    #[ignore = "exercise: implement push_front"]
    fn push_front_adds_without_changing_the_tail() {
        let list = push_front(from_slice(&[2, 3]), 1);
        assert_eq!(to_vec(&list), vec![1, 2, 3]);
    }

    #[test]
    #[ignore = "exercise: implement reverse"]
    fn reverse_handles_empty_single_and_long_lists() {
        assert_eq!(to_vec(&reverse(None)), vec![]);
        assert_eq!(to_vec(&reverse(from_slice(&[1]))), vec![1]);
        assert_eq!(to_vec(&reverse(from_slice(&[1, 2, 3, 4]))), vec![4, 3, 2, 1]);
    }

    #[test]
    #[ignore = "exercise: implement merge_sorted"]
    fn merge_sorted_handles_empty_and_duplicate_values() {
        assert_eq!(to_vec(&merge_sorted(None, from_slice(&[1, 3]))), vec![1, 3]);
        assert_eq!(
            to_vec(&merge_sorted(from_slice(&[1, 2, 4]), from_slice(&[1, 3, 4]))),
            vec![1, 1, 2, 3, 4, 4]
        );
    }

    #[test]
    #[ignore = "exercise: implement remove_first"]
    fn remove_first_splices_the_first_match_only() {
        let mut list = from_slice(&[1, 2, 2, 3]);
        assert!(remove_first(&mut list, 2));
        assert_eq!(to_vec(&list), vec![1, 2, 3]);
        assert!(!remove_first(&mut list, 9));
        assert_eq!(to_vec(&list), vec![1, 2, 3]);
    }
}
