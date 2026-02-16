use alloc::{boxed::Box, vec::Vec};

use crate::prelude::{BinaryTree, Tree};

mod map;
mod map_path;
pub use map::Map;
pub use map_path::MapPath;

mod prune;
mod prune_depth;
mod prune_path;
pub use prune::{BinaryPrune, Prune};
pub use prune_depth::PruneDepth;
pub use prune_path::{BinaryPrunePath, PrunePath};

pub trait TreeIteratorBase<Value, Children>: Iterator<Item = Value>
where
    Self: Sized,
{
    #[doc = include_str!("../../doc_files/path.md")]
    fn current_path(&self) -> &[usize];

    /// Gets the depth of the current node in the tree. This is zero-based,
    /// so the root node is at depth zero.
    ///
    /// Ex. given a tree like the following, the depths would be as labeled.
    /// ```text
    ///        0       <- depth: 0
    ///       / \
    ///      1   2     <- depth: 1
    ///     / \ / \
    ///    3  4 5  6   <- depth: 2
    ///           /
    ///          7     <- depth: 3
    ///           \
    ///            8   <- depth: 4
    ///           /
    ///          9     <- depth: 5
    ///           \
    ///           10   <- depth: 6
    /// ```
    fn current_depth(&self) -> usize {
        self.current_path().len()
    }

    /// For use in tree_iterators_rs internals. Consumers should prefer using one of the following
    /// public APIs to accomplish their task:
    /// 1. [`TreeIterator::prune`] or [`BinaryTreeIterator::prune`]
    /// 2. [`prune_depth`](TreeIteratorBase::prune_depth)
    fn prune_current_subtree(&mut self);

    /// Takes a depth and prunes the tree such that all nodes at a higher depth
    /// are pruned from the tree.
    ///
    /// Depth is zero-based, so the root node is at depth zero.
    ///
    /// Ex. given a tree like the following, the depths would be as labeled.
    /// ```text
    ///        0       <- depth: 0
    ///       / \
    ///      1   2     <- depth: 1
    ///     / \ / \
    ///    3  4 5  6   <- depth: 2
    ///           /
    ///          7     <- depth: 3
    ///           \
    ///            8   <- depth: 4
    ///           /
    ///          9     <- depth: 5
    ///           \
    ///           10   <- depth: 6
    /// ```
    ///
    /// Calling prune_depth with a depth of 2 would yield the following tree:
    /// ```text
    ///        0       <- depth: 0
    ///       / \
    ///      1   2     <- depth: 1
    ///     / \ / \
    ///    3  4 5  6   <- depth: 2
    /// ```
    ///
    /// ### Example Usage
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, OwnedTreeNode, TreeIterator, TreeIteratorBase};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![Tree {
    ///         value: 1,
    ///         children: vec![],
    ///     }],
    /// };
    ///
    /// let result =
    ///     tree.into_pipeline()
    ///         .prune_depth(0)
    ///         .collect_tree()
    ///         .expect("the root of the tree to remain un-pruned");
    ///
    /// assert_eq!(
    ///     Tree {
    ///         value: 0,
    ///         children: vec![],
    ///     },
    ///     result);
    /// ```
    #[must_use]
    fn prune_depth(self, depth_limit: usize) -> PruneDepth<Value, Children, Self> {
        PruneDepth {
            inner: self,
            depth: depth_limit,
            value: Default::default(),
            children: Default::default(),
        }
    }

    /// map_tree is the tree-based analog of [`Iterator::map`]. It is named
    /// as such to avoid name conflicts with [`Iterator::map`]. Takes a tree
    /// of values and maps each value to a new value.
    ///
    /// Takes a closure and creates a TreeIterator or BinaryTreeIterator which
    /// calls that closure on each element.
    ///
    /// map_tree() transforms one TreeIterator into another, by means of its argument:
    /// something that implements FnMut. It produces a new iterator which calls
    /// this closure on each element of the original iterator.
    ///
    /// If you are good at thinking in types, you can think of map_tree() like this: If
    /// you have a TreeIterator that gives you elements of some type A, and you want a
    /// TreeIterator of some other type B, you can use map_tree(), passing a closure that
    /// takes an A and returns a B.
    ///
    /// ### Basic Usage
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, OwnedTreeNode, TreeIterator, TreeIteratorBase};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![Tree {
    ///         value: 1,
    ///         children: vec![],
    ///     }],
    /// };
    ///
    /// let result =
    ///     tree.into_pipeline()
    ///         .map_tree(|value| value + 1)
    ///         .collect_tree()
    ///         .expect("the root of the tree to remain un-pruned");
    ///
    /// assert_eq!(
    ///     Tree {
    ///         value: 1,
    ///         children: vec![Tree {
    ///             value: 2,
    ///             children: vec![],
    ///         }],
    ///     },
    ///     result);
    /// ```
    #[must_use]
    fn map_tree<F, Output>(self, f: F) -> Map<Value, Children, Self, F, Output>
    where
        F: FnMut(Value) -> Output,
    {
        Map::new(self, f)
    }

    /// Identical to [`map_tree`](TreeIteratorBase::map_tree) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](TreeIteratorBase::current_path) for more details).
    #[must_use]
    fn map_path<F, Output>(self, f: F) -> MapPath<Value, Children, Self, F, Output>
    where
        F: FnMut(&[usize], Value) -> Output,
    {
        MapPath::new(self, f)
    }
}

pub trait TreeIterator<Value, Children>: TreeIteratorBase<Value, Children>
where
    Self: Sized,
{
    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the tree for
    /// which this evaluates to true will be pruned out of the resulting tree. If the root node
    /// is pruned, any subsequent calls to [`collect_tree`](TreeIterator::collect_tree)
    /// will yield [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, TreeIterator, OwnedTreeNode, TreeIteratorBase};
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
    /// let result = tree.into_pipeline()
    ///     .prune(|value| {
    ///         println!("{value:?}");
    ///         *value == 1
    ///     })
    ///     .collect_tree();
    ///
    /// assert_eq!(
    ///     Some(Tree {
    ///         value: 0,
    ///         children: vec![Tree {
    ///             value: 2,
    ///             children: vec![],
    ///         }],
    ///     }),
    ///     result);
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
    /// ```
    #[must_use]
    fn prune<F>(self, f: F) -> Prune<Value, Children, Self, F>
    where
        F: FnMut(&Value) -> bool,
    {
        Prune::new(self, f)
    }

    /// Identical to [`prune`](TreeIterator::prune) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](TreeIteratorBase::current_path) for more details).
    #[must_use]
    fn prune_path<F>(self, f: F) -> PrunePath<Value, Children, Self, F>
    where
        F: FnMut(&[usize], &Value) -> bool,
    {
        PrunePath::new(self, f)
    }

    /// Collects the current TreeIterator back into a Tree.
    ///
    /// If the TreeIterator is empty (usually due to pruning the root node), yields
    /// [`None`].
    ///
    /// ### Example Usage
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, OwnedTreeNode, TreeIterator};
    ///
    /// let tree = Tree {
    ///     value: 0,
    ///     children: vec![],
    /// };
    ///
    /// let result =
    ///     tree.into_pipeline()
    ///         .collect_tree()
    ///         .expect("the root of the tree to remain un-pruned");
    ///
    /// assert_eq!(
    ///     Tree {
    ///         value: 0,
    ///         children: vec![],
    ///     },
    ///     result);
    /// ```
    fn collect_tree(mut self) -> Option<Tree<Value>> {
        let mut keeping_stack: Vec<Tree<Value>> = Vec::new();
        while let Some(item) = self.next() {
            while keeping_stack.len() > self.current_depth() {
                let popped = keeping_stack
                    .pop()
                    .expect("the keeping stack to always have an item");

                let last_keeping_children = keeping_stack
                    .last_mut()
                    .expect("there to always be an item in the keeping stack.");

                last_keeping_children.children.push(popped);
            }

            keeping_stack.push(Tree {
                value: item,
                children: Vec::new(),
            });
        }

        while keeping_stack.len() > 1 {
            let popped = keeping_stack
                .pop()
                .expect("the keeping stack to always have an item");

            let last_keeping_children = keeping_stack
                .last_mut()
                .expect("there to always be an item in the keeping stack.");

            last_keeping_children.children.push(popped);
        }

        keeping_stack.pop()
    }

    /// A tree-based analog to [`fold`](Iterator::fold).
    ///
    /// Folds every node in the tree into an accumulated value by applying an operation,
    /// returning the final result.
    ///
    /// fold_tree() takes one arguments: a closure with two arguments: an ‘accumulator’ (the
    /// result of accumulating all children of the current node), and the current node's
    /// value. The closure returns the value that the accumulator should have for the
    /// subtree's parent's iteration.
    ///
    /// After applying this closure to every element of the tree, fold_tree() returns the
    /// accumulator.
    ///
    /// Folding is useful whenever you have a tree of something, and want to produce a
    /// single value from it.
    ///
    /// ### Basic Usage
    /// ```rust
    /// use tree_iterators_rs::prelude::{Tree, OwnedTreeNode, TreeIterator};
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
    /// let num_nodes_in_tree =
    ///     tree.into_pipeline()
    ///         .fold_tree(|children, value| {
    ///             let num_nodes_in_subtrees = children
    ///                 .into_iter()
    ///                 .sum::<usize>();
    ///
    ///             num_nodes_in_subtrees + 1
    ///         })
    ///         .expect("the root of the tree to remain un-pruned");
    ///
    /// assert_eq!(num_nodes_in_tree, 4);
    /// ```
    fn fold_tree<F, Output>(mut self, mut f: F) -> Option<Output>
    where
        F: FnMut(Vec<Output>, Value) -> Output,
    {
        let mut inversion_stack: Vec<Value> = Vec::new();
        let mut folded_so_far: Vec<Vec<Output>> = Vec::new();
        while let Some(item) = self.next() {
            while folded_so_far.len() > self.current_depth() {
                let items = folded_so_far.pop().unwrap();
                let value_to_fold = inversion_stack.pop().unwrap();
                let folded = f(items, value_to_fold);
                folded_so_far.last_mut().unwrap().push(folded);
            }

            inversion_stack.push(item);
            folded_so_far.push(Vec::new())
        }

        while folded_so_far.len() > 1 {
            let items = folded_so_far.pop().unwrap();
            let value_to_fold = inversion_stack.pop().unwrap();
            let folded = f(items, value_to_fold);
            folded_so_far.last_mut().unwrap().push(folded);
        }

        inversion_stack
            .pop()
            .map(|root| f(folded_so_far.pop().unwrap_or_default(), root))
    }

    /// Identical to [`fold_tree`](TreeIterator::fold_tree) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](TreeIteratorBase::current_path) for more details).
    fn fold_path<F, Output>(mut self, mut f: F) -> Option<Output>
    where
        F: FnMut(Vec<Output>, &[usize], Value) -> Output,
    {
        let mut inversion_stack: Vec<Value> = Vec::new();
        let mut folded_so_far: Vec<Vec<Output>> = Vec::new();
        let mut paths = Vec::new();
        while let Some(item) = self.next() {
            while folded_so_far.len() > self.current_depth() {
                let items = folded_so_far.pop().unwrap();
                let value_to_fold = inversion_stack.pop().unwrap();
                let folded = f(items, &paths, value_to_fold);
                paths.pop();
                folded_so_far.last_mut().unwrap().push(folded);
            }

            inversion_stack.push(item);
            folded_so_far.push(Vec::new());
            if paths.len() < self.current_depth() {
                paths.push(self.current_path().last().copied().unwrap());
            }
        }

        while folded_so_far.len() > 1 {
            let items = folded_so_far.pop().unwrap();
            let value_to_fold = inversion_stack.pop().unwrap();
            let folded = f(items, &paths, value_to_fold);
            paths.pop();
            folded_so_far.last_mut().unwrap().push(folded);
        }

        inversion_stack
            .pop()
            .map(|root: Value| f(folded_so_far.pop().unwrap_or_default(), &[], root))
    }
}

pub trait BinaryTreeIterator<Value, Children>: TreeIteratorBase<Value, Children>
where
    Self: Sized,
{
    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the tree for
    /// which this evaluates to true will be pruned out of the resulting tree. If the root node
    /// is pruned, any subsequent calls to [`collect_tree`](BinaryTreeIterator::collect_tree)
    /// will yield [`None`].
    ///
    /// The closure is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// closure on its descendent nodes.
    ///
    /// ### Basic usage:
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, BinaryTreeIterator, OwnedBinaryTreeNode, TreeIteratorBase};
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
    /// let result = tree.into_pipeline()
    ///     .prune(|value| {
    ///         println!("{value:?}");
    ///         *value == 1
    ///     })
    ///     .collect_tree();
    ///
    /// assert_eq!(
    ///     Some(BinaryTree {
    ///         value: 0,
    ///         left: None,
    ///         right: Some(Box::new(BinaryTree {
    ///             value: 2,
    ///             left: None,
    ///             right: None,
    ///         })),
    ///     }),
    ///     result);
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
    /// ```
    #[must_use]
    fn prune<F>(self, f: F) -> BinaryPrune<Value, Children, Self, F>
    where
        F: FnMut(&Value) -> bool,
    {
        BinaryPrune::new(self, f)
    }

    /// Identical to [`prune`](TreeIterator::prune) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](TreeIteratorBase::current_path) for more details).
    #[must_use]
    fn prune_path<F>(self, f: F) -> BinaryPrunePath<Value, Children, Self, F>
    where
        F: FnMut(&[usize], &Value) -> bool,
    {
        BinaryPrunePath::new(self, f)
    }

    /// Collects the current [`BinaryTreeIterator`] back into a BinaryTree.
    ///
    /// If the BinaryTreeIterator is empty (usually due to pruning the root node),
    /// yields [`None`].
    ///
    /// ### Example Usage
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, OwnedBinaryTreeNode, BinaryTreeIterator};
    ///
    /// let tree = BinaryTree {
    ///     value: 0,
    ///     left: None,
    ///     right: None,
    /// };
    ///
    /// let result =
    ///     tree.into_pipeline()
    ///         .collect_tree()
    ///         .expect("the root of the tree to remain un-pruned");
    ///
    /// assert_eq!(
    ///     BinaryTree {
    ///         value: 0,
    ///         left: None,
    ///         right: None,
    ///     },
    ///     result);
    /// ```
    fn collect_tree(mut self) -> Option<BinaryTree<Value>> {
        let mut keeping_stack: Vec<(usize, BinaryTree<Value>)> = Vec::new();
        while let Some(item) = self.next() {
            while keeping_stack.len() > self.current_depth() {
                let popped: (usize, BinaryTree<Value>) = keeping_stack
                    .pop()
                    .expect("the keeping stack to always have an item");

                let last_keeping_children = keeping_stack
                    .last_mut()
                    .expect("there to always be an item in the keeping stack.");

                match popped.0 {
                    0 => last_keeping_children.1.left = Some(Box::new(popped.1)),
                    1 => last_keeping_children.1.right = Some(Box::new(popped.1)),
                    _ => unreachable!(
                        "binary trees should only ever have paths that include 0's and 1's"
                    ),
                }
            }

            let index = self.current_path().last().copied().unwrap_or_default();

            keeping_stack.push((
                index,
                BinaryTree {
                    value: item,
                    left: None,
                    right: None,
                },
            ));
        }

        while keeping_stack.len() > 1 {
            let popped = keeping_stack
                .pop()
                .expect("the keeping stack to always have an item");

            let last_keeping_children = keeping_stack
                .last_mut()
                .expect("there to always be an item in the keeping stack.");

            match popped.0 {
                0 => last_keeping_children.1.left = Some(Box::new(popped.1)),
                1 => last_keeping_children.1.right = Some(Box::new(popped.1)),
                _ => unreachable!(
                    "binary trees should only ever have paths that include 0's and 1's"
                ),
            }
        }

        keeping_stack.pop().map(|tuple| tuple.1)
    }

    /// A tree-based analog to [`fold`](Iterator::fold).
    ///
    /// Folds every node in the tree into an accumulated value by applying an operation,
    /// returning the final result.
    ///
    /// fold() takes one arguments: a closure with two arguments: an ‘accumulator’ (the
    /// result of accumulating both children of the current node), and the current node's
    /// value. The closure returns the value that the accumulator should have for the
    /// subtree's parent's iteration.
    ///
    /// After applying this closure to every element of the iterator, fold() returns the
    /// accumulator.
    ///
    /// Folding is useful whenever you have a tree of something, and want to produce a
    /// single value from it.
    ///
    /// ### Basic Usage
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, OwnedBinaryTreeNode, BinaryTreeIterator};
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
    /// let num_nodes_in_tree =
    ///     tree.into_pipeline()
    ///         .fold_tree(|children, value| {
    ///             let num_nodes_in_subtrees = children
    ///                 .into_iter()
    ///                 .flat_map(|opt| opt)
    ///                 .sum::<usize>();
    ///
    ///             num_nodes_in_subtrees + 1
    ///         })
    ///         .expect("the root of the tree to remain un-pruned");
    ///
    /// assert_eq!(num_nodes_in_tree, 4);
    /// ```
    fn fold_tree<F, Output>(self, mut f: F) -> Option<Output>
    where
        F: FnMut([Option<Output>; 2], Value) -> Output,
    {
        // unlike the Tree implementation, there's no additional computational overhead for fold_path,
        // so just reuse it.
        self.fold_path(|acc, _, value| f(acc, value))
    }

    /// Identical to [`fold_tree`](BinaryTreeIterator::fold_tree) except that the closure is passed
    /// an additional parameter: the path of the current node in the tree (see
    /// [`current_path`](TreeIteratorBase::current_path) for more details).
    fn fold_path<F, Output>(mut self, mut f: F) -> Option<Output>
    where
        F: FnMut([Option<Output>; 2], &[usize], Value) -> Output,
    {
        let mut inversion_stack = Vec::new();
        let mut folded_so_far = Vec::new();
        let mut paths = Vec::new();
        while let Some(item) = self.next() {
            while folded_so_far.len() > self.current_depth() {
                let items = folded_so_far.pop().unwrap();
                let value_to_fold = inversion_stack.pop().unwrap();
                let folded = f(items, &paths, value_to_fold);
                let path_segment = paths.pop().unwrap();
                folded_so_far.last_mut().unwrap()[path_segment] = Some(folded);
            }

            inversion_stack.push(item);
            folded_so_far.push(Default::default());
            if paths.len() < self.current_depth() {
                paths.push(self.current_path().last().copied().unwrap());
            }
        }

        while folded_so_far.len() > 1 {
            let items = folded_so_far.pop().unwrap();
            let value_to_fold = inversion_stack.pop().unwrap();
            let folded = f(items, &paths, value_to_fold);
            let path_segment = paths.pop().unwrap();
            folded_so_far.last_mut().unwrap()[path_segment] = Some(folded);
        }

        inversion_stack
            .pop()
            .map(|root| f(folded_so_far.pop().unwrap_or_default(), &[], root))
    }
}
