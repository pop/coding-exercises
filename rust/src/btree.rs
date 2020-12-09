use std::rc::Rc;
use std::cell::{RefCell, RefMut};

pub fn run() {
    println!("Implement a Binary Tree");
}

///
/// Implement a Binary Search Tree which is not balanced.
///
/// Add the following functions:
/// * `new()` new empty btree.
/// * `insert(usize)` insert an element to the btree.
/// * `balanced()` returns if the tree is currently balanced.
/// * `balance()` balances the tree.
///
struct BSTree<'a> {
    value: &'a usize,
    left: Option<Box<BSTree<'a>>>,
    right: Option<Box<BSTree<'a>>>,
}

#[test]
fn test_btree() {
    let mut t = BSTree::new(&2);

    t.insert(&1);
    t.insert(&3);

    assert_eq!(t.balanced(), true);

    assert_eq!(
        t,
        BSTree {
            value: &2,
            left: Some(&BSTree::new(&1)),
            right: Some(&BSTree::new(&3))
        }
    );

    t.insert(&4);
    t.insert(&5);

    assert_eq!(t.balanced(), false);

    t.balance();

    assert_eq!(t.balanced(), true);
}

impl<'a> BSTree<'a> {
    pub fn new(val: &'a usize) -> BSTree<'a> {
        BSTree {
            value: val,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, val: &'a usize) {
        if val < self.value {
            match self.left {
                None => self.left = Some(Box::new(BSTree::new(val))),
                Some(node) => self.left.unwrap().insert(val),
            }
        } else if val >= self.value {
            match self.right {
                None => self.right = Some(Box::new(BSTree::new(val))),
                Some(node) => self.right.unwrap().insert(val),
            }
        }
    }
}
