use alloc::vec::Vec;

mod map;
mod map_path;
pub use map::CollectionMap;
pub use map_path::CollectionMapPath;
mod fold;
mod fold_path;
pub use fold::{BinaryFold, Fold};
pub use fold_path::{BinaryFoldPath, FoldPath};
mod prune;
mod prune_depth;
mod prune_path;
mod trees;
pub use prune::{BinaryCollectionPrune, CollectionPrune};
pub use prune_depth::CollectionPruneDepth;
pub use prune_path::{BinaryCollectionPrunePath, CollectionPrunePath};
pub use trees::{BinaryTrees, Trees};

pub trait TreeCollectionIteratorBase<Value, Children>: Iterator<Item = Value>
where
    Self: Sized,
{
    #[doc = include_str!("../../doc_files/path.md")]
    fn current_path(&self) -> &[usize];

    /// This API may panic if called before this Iterator's first next() method
    /// call or after this Iterator's next() method yields None.
    ///
    /// Gets the depth of the current node in the trees. This is zero-based,
    /// so the root nodes are at depth zero.
    ///
    /// Ex. given a collection of trees like the following, the depths would be as labeled.
    /// ```text
    ///        0       11    <- depth: 0
    ///       / \     /  \
    ///      1   2   12  13  <- depth: 1
    ///     / \ / \   \   \
    ///    3  4 5  6  14  15 <- depth: 2
    ///           /   /
    ///          7   16      <- depth: 3
    ///           \
    ///            8         <- depth: 4
    ///           /
    ///          9           <- depth: 5
    ///           \
    ///           10         <- depth: 6
    /// ```
    fn current_depth(&self) -> usize {
        self.current_path().len() - 1
    }

    /// For use in tree_iterators_rs internals. Consumers should prefer using one of the following
    /// public APIs to accomplish their task:
    /// 1. [`TreeCollectionIterator::prune`] or [`BinaryTreeCollectionIterator::prune`]
    /// 2. [`prune_depth`](TreeCollectionIteratorBase::prune_depth)
    fn prune_current_subtree(&mut self);

    /// Takes a depth and prunes the trees such that all nodes at a higher depth
    /// are pruned from the trees.
    ///
    /// Depth is zero-based, so the root node is at depth zero.
    ///
    /// Ex. given trees like the following, the depths would be as labeled.
    ///
    /// ```text
    ///        0       11    <- depth: 0
    ///       / \     /  \
    ///      1   2   12  13  <- depth: 1
    ///     / \ / \   \   \
    ///    3  4 5  6  14  15 <- depth: 2
    ///           /   /
    ///          7   16      <- depth: 3
    ///           \
    ///            8         <- depth: 4
    ///           /
    ///          9           <- depth: 5
    ///           \
    ///           10         <- depth: 6
    /// ```
    ///
    /// Calling prune_depth with a depth of 2 would yield the following trees:
    ///
    /// ```text
    ///        0       11    <- depth: 0
    ///       / \     /  \
    ///      1   2   12  13  <- depth: 1
    ///     / \ / \   \   \
    ///    3  4 5  6  14  15 <- depth: 2
    /// ```
    ///
    /// ### Example Usage
    ///
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, OwnedIntoIteratorOfTrees, TreeCollectionIterator, TreeCollectionIteratorBase};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![Tree {
    ///         value: 1,
    ///         children: vec![Tree {
    ///             value: 2,
    ///             children: vec![],
    ///         }],
    ///     }],
    /// };
    ///
    /// let trees = vec![tree.clone(), tree.clone()];
    ///
    /// let result = trees.into_pipeline()
    ///     .prune_depth(1)
    ///     .trees()
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     vec![
    ///         Tree {
    ///             value: 0,
    ///             children: vec![Tree {
    ///                 value: 1,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 0,
    ///             children: vec![Tree {
    ///                 value: 1,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///     ],
    ///     result
    /// );
    /// ```
    #[must_use]
    fn prune_depth(self, depth_limit: usize) -> CollectionPruneDepth<Value, Children, Self> {
        CollectionPruneDepth {
            inner: self,
            depth: depth_limit,
            value: Default::default(),
            children: Default::default(),
        }
    }

    /// map_trees is the tree-collection-based analog of [`Iterator::map`]. It is named
    /// as such to avoid name conflicts with [`Iterator::map`]. Takes a collection of trees
    /// containing values and maps each value to a new value across all trees.
    ///
    /// Takes a closure and creates a TreeCollectionIterator or BinaryTreeCollectionIterator which
    /// calls that closure on each element.
    ///
    /// map_trees() transforms one TreeCollectionIterator into another, by means of its argument:
    /// something that implements FnMut. It produces a new iterator which calls
    /// this closure on each element across all trees in the collection.
    ///
    /// If you are good at thinking in types, you can think of map_trees() like this: If
    /// you have a TreeCollectionIterator that gives you elements of some type A, and you want a
    /// TreeCollectionIterator of some other type B, you can use map_trees(), passing a closure that
    /// takes an A and returns a B.
    ///
    /// ### Basic Usage
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, OwnedIntoIteratorOfTrees, TreeCollectionIterator, TreeCollectionIteratorBase};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![Tree {
    ///         value: 1,
    ///         children: vec![],
    ///     }],
    /// };
    ///
    /// let trees = vec![tree.clone(), tree.clone()];
    ///
    /// let result =
    ///     trees.into_pipeline()
    ///         .map_trees(|value| value + 1)
    ///         .trees()
    ///         .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 2,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 2,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///     ],
    ///     result
    /// );
    /// ```
    #[must_use]
    fn map_trees<F, Output>(self, f: F) -> CollectionMap<Value, Children, Self, F, Output>
    where
        F: FnMut(Value) -> Output,
    {
        CollectionMap::new(self, f)
    }

    /// Identical to [`map_trees`](TreeCollectionIteratorBase::map_trees) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](TreeCollectionIteratorBase::current_path) for more details).
    #[must_use]
    fn map_path<F, Output>(self, f: F) -> CollectionMapPath<Value, Children, Self, F, Output>
    where
        F: FnMut(&[usize], Value) -> Output,
    {
        CollectionMapPath::new(self, f)
    }
}

pub trait TreeCollectionIterator<Value, Children>:
    TreeCollectionIteratorBase<Value, Children>
where
    Self: Sized,
{
    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    /// Uses the given closure to determine if each subtree in these trees should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the trees for
    /// which this evaluates to true will be pruned out of the resulting trees.
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder_each`](crate::prelude::OwnedIntoIteratorOfTrees::dfs_preorder_each) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, TreeCollectionIterator, OwnedIntoIteratorOfTrees, TreeCollectionIteratorBase};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: vec![],
    ///         },
    ///     ],
    /// };
    ///
    /// let trees = vec![tree.clone(), tree];
    ///
    /// let result = trees.into_pipeline()
    ///     .prune(|value| {
    ///         println!("{value:?}");
    ///         *value == 1
    ///     })
    ///     .trees()
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     vec![
    ///         Tree {
    ///             value: 0,
    ///             children: vec![Tree {
    ///                 value: 2,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///         Tree {
    ///             value: 0,
    ///             children: vec![Tree {
    ///                 value: 2,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///     ],
    ///     result
    /// );
    /// ```
    /// The output for this code would be the following. A couple notes about this output:
    /// 1. the node with a value of '1' has been removed
    /// 2. the closure is never called on the nodes with values of '3' since it is already
    ///    determined to be pruned once '1' has been evaluated.
    ///
    /// ```text
    /// 0
    /// 1
    /// 2
    /// 0
    /// 1
    /// 2
    /// ```
    #[must_use]
    fn prune<F>(self, f: F) -> CollectionPrune<Value, Children, Self, F>
    where
        F: FnMut(&Value) -> bool,
    {
        CollectionPrune::new(self, f)
    }

    /// Identical to [`prune`](TreeCollectionIterator::prune) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](TreeCollectionIteratorBase::current_path) for more details).
    #[must_use]
    fn prune_path<F>(self, f: F) -> CollectionPrunePath<Value, Children, Self, F>
    where
        F: FnMut(&[usize], &Value) -> bool,
    {
        CollectionPrunePath::new(self, f)
    }

    /// A tree-based analog to [`fold_trees`](Iterator::fold).
    ///
    /// Folds every node in the trees into an accumulated value by applying an operation,
    /// returning the final result.
    ///
    /// fold_trees() takes one arguments: a closure with two arguments: an ‘accumulator’ (the
    /// result of accumulating all children of the current node), and the current node's
    /// value. The closure returns the value that the accumulator should have for the
    /// subtree's parent's iteration.
    ///
    /// After applying this closure to every element of the iterator, fold_trees() returns an
    /// iterator over each tree's accumulator.
    ///
    /// Folding is useful whenever you have trees of something, and want to produce a
    /// list of values from them.
    ///
    /// ### Basic Usage
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, OwnedIntoIteratorOfTrees, TreeCollectionIterator};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///     ],
    /// };
    ///
    /// let trees = vec![tree.clone(), tree];
    ///
    /// let num_nodes_in_tree =
    ///     trees.into_pipeline()
    ///         .fold_trees(|children, value| {
    ///             let num_nodes_in_subtrees = children
    ///                 .into_iter()
    ///                 .sum::<usize>();
    ///
    ///             num_nodes_in_subtrees + 1
    ///         })
    ///         .collect::<Vec<_>>();
    ///
    /// assert_eq!(num_nodes_in_tree, vec![4, 4]);
    /// ```
    #[must_use]
    fn fold_trees<Output, F>(self, f: F) -> Fold<Value, Children, Self, F, Output>
    where
        F: FnMut(Vec<Output>, Value) -> Output,
    {
        Fold::new(self, f)
    }

    /// Identical to [`fold_path`](TreeCollectionIterator::fold_path) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](TreeCollectionIteratorBase::current_path) for more details).
    #[must_use]
    fn fold_path<Output, F>(self, f: F) -> FoldPath<Value, Children, Self, F, Output>
    where
        F: FnMut(Vec<Output>, &[usize], Value) -> Output,
    {
        FoldPath::new(self, f)
    }

    /// trees() converts this TreeCollectionIterator into a standard
    /// Iterator which yields each Tree from the collection after applying all transformations.
    ///
    /// ### Basic Usage
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, OwnedIntoIteratorOfTrees, TreeCollectionIterator};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![
    ///         Tree {
    ///             value: 1,
    ///             children: vec![],
    ///         },
    ///         Tree {
    ///             value: 2,
    ///             children: vec![Tree {
    ///                 value: 3,
    ///                 children: vec![],
    ///             }],
    ///         },
    ///     ],
    /// };
    ///
    /// let trees = vec![tree.clone(), tree.clone()];
    ///
    /// for recollected_tree in trees.into_pipeline().trees() {
    ///     assert_eq!(tree, recollected_tree);
    /// }
    /// ```
    #[must_use]
    fn trees(self) -> Trees<Value, Children, Self> {
        Trees::new(self)
    }
}

pub trait BinaryTreeCollectionIterator<Value, Children>:
    TreeCollectionIteratorBase<Value, Children>
where
    Self: Sized,
{
    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    /// Uses the given closure to determine if each subtree in these binary trees should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the trees for
    /// which this evaluates to true will be pruned out of the resulting trees.
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder_each`](crate::prelude::OwnedIntoIteratorOfBinaryTrees::dfs_preorder_each) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, BinaryTreeCollectionIterator, OwnedIntoIteratorOfBinaryTrees, TreeCollectionIteratorBase};
    ///
    /// let tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: Some(Box::new(BinaryTree {
    ///             value: 3,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: None,
    ///     }))
    /// };
    ///
    /// let trees = vec![tree.clone(), tree];
    ///
    /// let result = trees.into_pipeline()
    ///     .prune(|value| {
    ///         println!("{value:?}");
    ///         *value == 1
    ///     })
    ///     .trees()
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     vec![
    ///         BinaryTree {
    ///             value: 0,
    ///             left: None,
    ///             right: Some(Box::new(BinaryTree {
    ///                 value: 2,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///         },
    ///         BinaryTree {
    ///             value: 0,
    ///             left: None,
    ///             right: Some(Box::new(BinaryTree {
    ///                 value: 2,
    ///                 left: None,
    ///                 right: None,
    ///             })),
    ///         },
    ///     ],
    ///     result
    /// );
    /// ```
    /// The output for this code would be the following. A couple notes about this output:
    /// 1. the node with a value of '1' has been removed
    /// 2. the closure is never called on the node with a value of '3' since it is already
    ///    determined to be pruned once '1' has been evaluated.
    ///
    /// ```text
    /// 0
    /// 1
    /// 2
    /// 0
    /// 1
    /// 2
    /// ```
    #[must_use]
    fn prune<F>(self, f: F) -> BinaryCollectionPrune<Value, Children, Self, F>
    where
        F: FnMut(&Value) -> bool,
    {
        BinaryCollectionPrune::new(self, f)
    }

    /// Identical to [`prune`](TreeCollectionIterator::prune) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](TreeCollectionIteratorBase::current_path) for more details).
    #[must_use]
    fn prune_path<F>(self, f: F) -> BinaryCollectionPrunePath<Value, Children, Self, F>
    where
        F: FnMut(&[usize], &Value) -> bool,
    {
        BinaryCollectionPrunePath::new(self, f)
    }

    /// A tree-based analog to [`fold`](Iterator::fold).
    ///
    /// Folds every node in the tree into an accumulated value by applying an operation,
    /// returning the final result.
    ///
    /// fold_trees() takes one arguments: a closure with two arguments: an ‘accumulator’ (the
    /// result of accumulating both children of the current node), and the current node's
    /// value. The closure returns the value that the accumulator should have for the
    /// subtree's parent's iteration.
    ///
    /// After applying this closure to every element of the iterator, fold_trees() returns an
    /// iterator over the accumulators.
    ///
    /// Folding is useful whenever you have a tree of something, and want to produce a
    /// single value from it.
    ///
    /// ### Basic Usage
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, OwnedIntoIteratorOfBinaryTrees, BinaryTreeCollectionIterator};
    ///
    /// let tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: None,
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 3,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///     })),
    /// };
    ///
    /// let trees = vec![tree.clone(), tree.clone()];
    ///
    /// let num_nodes_in_tree =
    ///     trees.into_pipeline()
    ///         .fold_trees(|children, value| {
    ///             let num_nodes_in_subtrees = children
    ///                 .into_iter()
    ///                 .flat_map(|opt| opt)
    ///                 .sum::<usize>();
    ///
    ///             num_nodes_in_subtrees + 1
    ///         })
    ///         .collect::<Vec<_>>();
    ///
    /// assert_eq!(num_nodes_in_tree, vec![4, 4]);
    /// ```
    #[must_use]
    fn fold_trees<Output, F>(self, f: F) -> BinaryFold<Value, Children, Self, F, Output>
    where
        F: FnMut([Option<Output>; 2], Value) -> Output,
    {
        BinaryFold::new(self, f)
    }

    /// Identical to [`fold_path`](BinaryTreeCollectionIterator::fold_path) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](TreeCollectionIteratorBase::current_path) for more details).
    #[must_use]
    fn fold_path<Output, F>(self, f: F) -> BinaryFoldPath<Value, Children, Self, F, Output>
    where
        F: FnMut([Option<Output>; 2], &[usize], Value) -> Output,
    {
        BinaryFoldPath::new(self, f)
    }

    /// trees() converts this BinaryTreeCollectionIterator into a standard
    /// Iterator which yields each BinaryTree from the collection after applying all transformations.
    ///
    /// ### Basic Usage
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, OwnedIntoIteratorOfBinaryTrees, BinaryTreeCollectionIterator};
    ///
    /// let tree = BinaryTree {
    ///     value: 0,
    ///     left: Some(Box::new(BinaryTree {
    ///         value: 1,
    ///         left: None,
    ///         right: None,
    ///     })),
    ///     right: Some(Box::new(BinaryTree {
    ///         value: 2,
    ///         left: None,
    ///         right: None,
    ///     })),
    /// };
    ///
    /// let trees = vec![tree.clone(), tree.clone()];
    ///
    /// for recollected_tree in trees.into_pipeline().trees() {
    ///     assert_eq!(tree, recollected_tree);
    /// }
    /// ```
    #[must_use]
    fn trees(self) -> BinaryTrees<Value, Children, Self> {
        BinaryTrees::new(self)
    }
}
