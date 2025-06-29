use alloc::vec::Vec;

mod map;
pub use map::CollectionMap;
mod fold;
pub use fold::{FallibleFold, FallibleBinaryFold};
mod prune;
mod prune_depth;
mod prune_path;
mod trees;
pub use prune::{FallibleBinaryCollectionPrune, FallibleCollectionPrune};
pub use prune_depth::FallibleCollectionPruneDepth;
pub use prune_path::{FallibleBinaryCollectionPrunePath, FallibleCollectionPrunePath};
pub use trees::{FallibleBinaryTrees, FallibleTrees};

pub trait FallibleTreeCollectionIteratorBase<Value, Children, Err>:
    Iterator<Item = Result<Value, Err>>
where
    Self: Sized,
{
    #[doc = include_str!("../../doc_files/collection_path.md")]
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
    /// 1. [`FallibleTreeCollectionIterator::prune`] or [`FallibleBinaryTreeCollectionIterator::prune`]
    /// 2. [`prune_depth`](FallibleTreeCollectionIteratorBase::prune_depth)
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
    /// Note: This method works with fallible iterators that yield `Result<Value, Err>`.
    /// Errors encountered during iteration will be propagated through the results.
    #[must_use]
    fn prune_depth(
        self,
        depth_limit: usize,
    ) -> FallibleCollectionPruneDepth<Value, Children, Err, Self> {
        FallibleCollectionPruneDepth {
            inner: self,
            depth: depth_limit,
            value: Default::default(),
            children: Default::default(),
            err: Default::default(),
        }
    }

    /// map_tree is the tree-collection-based analog of [`Iterator::map`] for fallible iterators.
    /// It is named as such to avoid name conflicts with [`Iterator::map`]. Takes a collection of trees
    /// containing values that might be errors and maps each successful value to a new value across all trees.
    ///
    /// Takes a closure and creates a FallibleTreeCollectionIterator or FallibleBinaryTreeCollectionIterator which
    /// calls that closure on each successful element across all trees in the collection.
    ///
    /// map_tree() transforms one FallibleTreeCollectionIterator into another, by means of its argument:
    /// something that implements FnMut. It produces a new iterator which calls
    /// this closure on each successful element across all trees in the collection.
    ///
    /// If you are good at thinking in types, you can think of map_tree() like this: If
    /// you have a FallibleTreeCollectionIterator that gives you elements of some type A (wrapped in Result),
    /// and you want a FallibleTreeCollectionIterator of some other type B, you can use map_tree(),
    /// passing a closure that takes an A and returns a B.
    #[must_use]
    fn map_tree<F, Output>(self, f: F) -> CollectionMap<Value, Children, Err, Self, F, Output>
    where
        F: FnMut(Value) -> Output,
    {
        CollectionMap::new(self, f)
    }
}

pub trait FallibleTreeCollectionIterator<Value, Children, Err>:
    FallibleTreeCollectionIteratorBase<Value, Children, Err>
where
    Self: Sized,
{
    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter) for fallible iterators.
    /// Uses the given closure to determine if each subtree in these trees should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the trees for
    /// which this evaluates to true will be pruned out of the resulting trees.
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order. If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// Note: This method works with fallible iterators. If an error is encountered during iteration,
    /// it will be propagated through the result without calling the pruning closure.
    #[must_use]
    fn prune<F>(self, f: F) -> FallibleCollectionPrune<Value, Children, Err, Self, F>
    where
        F: FnMut(&Value) -> bool,
    {
        FallibleCollectionPrune::new(self, f)
    }

    /// Identical to [`prune`](FallibleTreeCollectionIterator::prune) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](FallibleTreeCollectionIteratorBase::current_path) for more details).
    ///
    /// This method works with fallible iterators. Errors encountered during iteration
    /// will be propagated through the results.
    #[must_use]
    fn prune_path<F>(self, f: F) -> FallibleCollectionPrunePath<Value, Children, Err, Self, F>
    where
        F: FnMut(&[usize], &Value) -> bool,
    {
        FallibleCollectionPrunePath::new(self, f)
    }

    /// A tree-collection-based analog to [`fold`](Iterator::fold) for fallible iterators.
    ///
    /// Folds every node in each tree of the collection into an accumulated value by applying an operation,
    /// returning the final result as an iterator over the accumulated values.
    ///
    /// fold_trees() takes one argument: a closure with two arguments: an 'accumulator' (the
    /// result of accumulating all children of the current node), and the current node's
    /// value. The closure returns the value that the accumulator should have for the
    /// subtree's parent's iteration.
    ///
    /// After applying this closure to every element of each tree, fold_trees() returns an
    /// iterator over each tree's accumulated result wrapped in a Result.
    ///
    /// Folding is useful whenever you have a collection of trees of something, and want to produce a
    /// list of values from them.
    ///
    /// Note: This method works with fallible iterators. If an error is encountered during iteration,
    /// the folding operation will be interrupted and the error will be returned.
    #[must_use]
    fn fold_trees<F, Output>(self, f: F) -> FallibleFold<Value, Children, Err, Self, Output, F>
    where
        F: FnMut(Vec<Output>, Value) -> Output,
    {
        FallibleFold::new(self, f)
    }

    /// trees() converts this FallibleTreeCollectionIterator into a standard
    /// Iterator which yields each Tree from the collection after applying all transformations.
    /// Each tree is wrapped in a Result to handle potential errors.
    #[must_use]
    fn trees(self) -> FallibleTrees<Value, Children, Err, Self> {
        FallibleTrees::new(self)
    }
}

pub trait FallibleBinaryTreeCollectionIterator<Value, Children, Err>:
    FallibleTreeCollectionIteratorBase<Value, Children, Err>
where
    Self: Sized,
{
    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter) for fallible binary tree iterators.
    /// Uses the given closure to determine if each subtree in these binary trees should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the trees for
    /// which this evaluates to true will be pruned out of the resulting trees.
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order. If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// Note: This method works with fallible iterators. If an error is encountered during iteration,
    /// it will be propagated through the result without calling the pruning closure.
    #[must_use]
    fn prune<F>(self, f: F) -> FallibleBinaryCollectionPrune<Value, Children, Err, Self, F>
    where
        F: FnMut(&Value) -> bool,
    {
        FallibleBinaryCollectionPrune::new(self, f)
    }

    /// Identical to [`prune`](FallibleBinaryTreeCollectionIterator::prune) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](FallibleTreeCollectionIteratorBase::current_path) for more details).
    ///
    /// This method works with fallible binary tree iterators. Errors encountered during iteration
    /// will be propagated through the results.
    #[must_use]
    fn prune_path<F>(self, f: F) -> FallibleBinaryCollectionPrunePath<Value, Children, Err, Self, F>
    where
        F: FnMut(&[usize], &Value) -> bool,
    {
        FallibleBinaryCollectionPrunePath::new(self, f)
    }

    /// A tree-collection-based analog to [`fold`](Iterator::fold) for fallible binary tree iterators.
    ///
    /// Folds every node in each binary tree of the collection into an accumulated value by applying an operation,
    /// returning the final result as an iterator over the accumulated values.
    ///
    /// fold_trees() takes one argument: a closure with two arguments: an 'accumulator' (the
    /// result of accumulating both children of the current node), and the current node's
    /// value. The closure returns the value that the accumulator should have for the
    /// subtree's parent's iteration.
    ///
    /// After applying this closure to every element of each tree, fold_trees() returns an
    /// iterator over each tree's accumulated result wrapped in a Result.
    ///
    /// Folding is useful whenever you have a collection of binary trees of something, and want to produce a
    /// list of values from them.
    ///
    /// Note: This method works with fallible iterators. If an error is encountered during iteration,
    /// the folding operation will be interrupted and the error will be returned.
    #[must_use]
    fn fold_trees<F, Output>(self, f: F) -> FallibleBinaryFold<Value, Children, Err, Self, Output, F>
    where
        F: FnMut([Option<Output>; 2], Value) -> Output,
    {
        FallibleBinaryFold::new(self, f)
    }

    /// trees() converts this FallibleBinaryTreeCollectionIterator into a standard
    /// Iterator which yields each BinaryTree from the collection after applying all transformations.
    /// Each binary tree is wrapped in a Result to handle potential errors.
    #[must_use]
    fn trees(self) -> FallibleBinaryTrees<Value, Children, Err, Self> {
        FallibleBinaryTrees::new(self)
    }
}
