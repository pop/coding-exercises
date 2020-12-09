pub fn run() {
    println!("Implement a Binary Tree");
}

///
/// Implement a Binary Search Tree which is not balanced.
///
/// Add the following functions:
/// * `new()` new empty btree.
/// * `insert(isize)` insert an element to the btree.
/// * `balanced()` returns if the tree is currently balanced.
/// * `balance()` balances the tree.
///
#[derive(Debug, PartialEq)]
pub struct BSTree<'a> {
    value: isize,
    left: Option<Box<BSTree<'a>>>,
    right: Option<Box<BSTree<'a>>>,
    _phantom: std::marker::PhantomData<&'a ()>
}

#[test]
fn test_btree() {
    let mut t = BSTree::new(2);

    t.insert(1);
    t.insert(3);

    assert_eq!(t.balanced(), true);

    assert_eq!(
        t,
        BSTree {
            value: 2,
            left: Some(Box::new(BSTree::new(1))),
            right: Some(Box::new(BSTree::new(3))),
            _phantom: std::marker::PhantomData
        }
    );

    t.insert(4);
    t.insert(5);

    assert_eq!(t.balanced(), false);

    // t.balance();

    // assert_eq!(t.balanced(), true);
}

impl<'a> BSTree<'a> {
    pub fn new(val: isize) -> BSTree<'a> {
        BSTree {
            value: val,
            left: None,
            right: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Insert a value into the tree
    ///
    /// ```
    /// # use bstree::BSTree;
    /// let mut my_tree = BSTree::new(5);
    /// my_tree.insert(3);
    /// my_tree.insert(7);
    /// ```
    pub fn insert(&mut self, val: isize) {
        // Credit to this article for helping me break through lifetimes confusion
        // https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370
        if val < self.value {
            // We match on the mutable value `self.left`
            match &mut self.left {
                // Each pattern must also include `&mut`, so we can modify the node.
                &mut None => *(&mut self.left) = Some(Box::new(BSTree::new(val))),
                // We match on Some(ref ...) so we don't move into the node,
                // just use a refernece to the node.
                &mut Some(ref mut node) => node.insert(val),
            }
        } else {
            match &mut self.right {
                &mut None => *(&mut self.right) = Some(Box::new(BSTree::new(val))),
                &mut Some(ref mut node) => node.insert(val),
            }
        }
    }

    /// Determines if a tree is balanced using the depth method.
    ///
    /// The tree is unbalanced if both branches differ in depth by more than 1
    ///
    /// Note that we could implement this without the use of `depth` by making recursive calls to
    /// `balanced` and have some logic that checks not only the immediate `left` and `right` nodes,
    /// but the child nodes as well.
    ///
    /// This solution is more concise and reuses logic we would have anyway in a tree type.
    ///
    /// ```
    /// # use bstree::BSTree;
    /// let mut my_tree = BSTree::new(10);
    ///
    /// assert_eq!(my_tree.balanced(), true);
    ///
    /// my_tree.insert(11);
    /// my_tree.insert(12);
    ///
    /// assert_eq!(my_tree.balanced(), false);
    ///
    /// my_tree.insert(1);
    /// my_tree.insert(2);
    ///
    /// assert_eq!(my_tree.balanced(), true);
    /// ```
    pub fn balanced(&self) -> bool {
        let left = match &self.left {
            Some(node) => node.depth(),
            None => 0,
        };
        let right = match &self.right {
            Some(node) => node.depth(),
            None => 0,
        };
        [left-1, left, left+1].contains(&right) && [right-1, right, right+1].contains(&left)
    }

    /// Determines the depth of a tree.
    ///
    /// Returns max depth of all on this tree.
    ///
    /// Does this dynamically, making it an expensive operation.
    /// The tradeoff is we do less book-keeping when inserting and balancing the tree keeping track
    /// of depths at that point.
    ///
    /// If this were used in prod we would most certainly add a "depth" field to the "BSTree"
    /// struct that cached the depth of each tree node.
    ///
    /// ```
    /// # use bstree::BSTree;
    /// let mut my_tree = BSTree::new(5);
    ///
    /// assert_eq!(my_tree.depth(), 1);
    ///
    /// my_tree.insert(6);
    /// my_tree.insert(7);
    /// my_tree.insert(8);
    ///
    /// assert_eq!(my_tree.depth(), 4);
    ///
    /// my_tree.insert(4);
    /// my_tree.insert(3);
    /// my_tree.insert(2);
    ///
    /// assert_eq!(my_tree.depth(), 4);
    /// ```
    pub fn depth(&self) -> isize {
        let right = match &self.right {
            Some(node) => node.depth(),
            None => 0,
        };
        let left = match &self.left {
            Some(node) => node.depth(),
            None => 0,
        };
        println!("tree: {:?} | right: {} | left: {}", self, right, left);
        1 + std::cmp::max(left, right)
    }
}
