use std::cell::UnsafeCell;
use std::mem;
use std::ops::{Deref, DerefMut};

struct Node<T> {
    parent: Option<*mut Node<T>>,
    first_child: Option<*mut Node<T>>,
    last_child: Option<*mut Node<T>>,
    prev_sibling: Option<*mut Node<T>>,
    next_sibling: Option<*mut Node<T>>,
    data: T,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            parent: None,
            first_child: None,
            last_child: None,
            prev_sibling: None,
            next_sibling: None,
            data,
        }
    }
}

struct Tree<T> {
    node: UnsafeCell<Node<T>>,
}

impl<T> Tree<T> {
    fn new(data: T) -> Tree<T> {
        Tree {
            node: UnsafeCell::new(Node::new(data)),
        }
    }
}
