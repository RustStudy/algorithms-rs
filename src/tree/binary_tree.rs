//! # Examples
//!
//! ```
//! use algorithms::tree::binary_tree::{BinTree};
//! let mut tree = BinTree::new();
//! tree.insert(2);
//! tree.insert(0);
//! tree.insert(3);
//! tree.insert(1);
//! tree.insert(4);
//! println!("{:#?}", tree);
//! println!("{}", tree);
//! ```

use std::fmt;

#[derive(Debug)]
pub struct BinTree<T>
where T: Ord {
    root: Option<Box<BinNode<T>>>,
}

#[derive(Debug)]
struct BinNode<T>
where T: Ord {
    value: T,
    left: Option<Box<BinNode<T>>>,
    right: Option<Box<BinNode<T>>>,
}

macro_rules! insert_at {
    ($node:expr, $value:expr) => {
        {
            (move || {
                if let Some(ref mut node) = $node.as_mut() {
                    node.insert($value);
                    return
                }

                $node = Some(Box::new(BinNode::new($value)));
            })()
        }
    };
}

macro_rules! fmt_node {
    ($fmt:expr, $node:expr) => {
        if let Some(ref node) = $node.as_ref() {
            try!(node.fmt($fmt));
        }
    };
}

impl<T> BinTree<T>
where T: Ord {
    pub fn new() -> Self {
        BinTree {
            root: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        insert_at!(self.root, value);
    }
}

impl<T> fmt::Display for BinTree<T>
where T: Ord + fmt::Display {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(fmt, "["));
        fmt_node!(fmt, self.root);
        try!(write!(fmt, "]"));
        Ok(())
    }
}

impl<T> BinNode<T>
where T: Ord {
    fn new(value: T) -> Self {
        BinNode {
            value: value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if value <= self.value {
            insert_at!(self.left, value);
        } else {
            insert_at!(self.right, value);
        }
    }
}

impl<T> fmt::Display for BinNode<T>
where T: Ord + fmt::Display {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt_node!(fmt, self.left);
        try!(write!(fmt, "{}, ", self.value));
        fmt_node!(fmt, self.right);
        Ok(())
    }
}
