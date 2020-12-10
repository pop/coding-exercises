///
/// A simple binary tree.
///
/// This tree is guarenteed to be sorted, in that the left node is always less than the current
/// node, and right is always greater
///
/// The tree not sorted, meaning the optimal path from the root to any element in the tree is
/// determined by order of insertion, not optimal path.
///
/// For example, the following steps:
/// 
/// ```
/// # use exercises::btree::BTree;
/// let mut t = BTree::new(7);
/// t.insert(1);
/// t.insert(2);
/// t.insert(3);
/// t.insert(4);
/// t.insert(5);
/// t.insert(6);
/// ```
///
/// Should produce a tree like this:
///
/// ```text
/// 4 -> 6 -> 7
/// |     `-> 5
/// `-> 2 -> 3
///      `-> 1
/// ```
///
/// to minimize search time, but instead it produces a tree like this:
/// ```text
/// 7
/// `-> 1 -> 2 -> 3 -> 4 -> 5 -> 6
/// ```
///
/// Making the tree very imbalanced and sub-optimal to search.
///
/// The reason I haven't solved this is because the hoops one needs to jump through in Rust are
/// trickey to solve the ownership problem when sorting a tree in-place.
///
/// I could solve this by instead of sorting in-place, returning a sorted copy of the tree, but
/// that is a totally different (and easier) challenge.
///
/// Some day I will figure out the lifetimes, Rc, Arc, and Boxes required to sort a tree in-place.
/// For now I'm putting that on the back-burner.
///
#[derive(Debug, PartialEq, Clone)]
pub struct BTree<'a> {
    value: isize,
    left: Option<Box<BTree<'a>>>,
    right: Option<Box<BTree<'a>>>,
    _phantom: std::marker::PhantomData<&'a ()>
}

#[test]
fn test_btree() {
    let mut t = BTree::new(2);

    t.insert(1);
    t.insert(3);

    assert_eq!(t.balanced(), true);

    assert_eq!(
        t,
        BTree {
            value: 2,
            left: Some(Box::new(BTree::new(1))),
            right: Some(Box::new(BTree::new(3))),
            _phantom: std::marker::PhantomData
        }
    );

    t.insert(4);
    t.insert(5);

    assert_eq!(t.balanced(), false);

    // t.balance();

    // assert_eq!(t.balanced(), true);
}

impl<'a> BTree<'a> {
    pub fn new(val: isize) -> BTree<'a> {
        BTree {
            value: val,
            left: None,
            right: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Insert a value into the tree
    ///
    /// ```
    /// # use exercises::btree::BTree;
    /// let mut my_tree = BTree::new(5);
    /// my_tree.insert(3);
    /// my_tree.insert(7);
    /// ```
    pub fn insert(&mut self, val: isize) {
        // Duplicate values are not stored
        if val == self.value {
            return
        }

        // Credit to this article for helping me break through lifetimes confusion
        // https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370
        if val < self.value {
            // We match on the mutable value `self.left`
            match &mut self.left {
                // Each pattern must also include `&mut`, so we can modify the node.
                &mut None => *(&mut self.left) = Some(Box::new(BTree::new(val))),
                // We match on Some(ref ...) so we don't move into the node,
                // just use a refernece to the node.
                &mut Some(ref mut node) => node.insert(val),
            }
        } else {
            match &mut self.right {
                &mut None => *(&mut self.right) = Some(Box::new(BTree::new(val))),
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
    /// # use exercises::btree::BTree;
    /// let mut my_tree = BTree::new(10);
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
    /// If this were used in prod we would most certainly add a "depth" field to the "BTree"
    /// struct that cached the depth of each tree node.
    ///
    /// ```
    /// # use exercises::btree::BTree;
    /// let mut my_tree = BTree::new(5);
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
        1 + std::cmp::max(left, right)
    }
}
