use alloc::{boxed::Box, vec::Vec};
use streaming_iterator::StreamingIterator;

use crate::prelude::{BinaryTree, Tree, TreeContext};

mod map;
mod map_context;
pub use map::Map;
pub use map_context::MapContext;

mod prune;
mod prune_context;
mod prune_depth;
pub use prune::Prune;
pub use prune_context::PruneContext;
pub use prune_depth::PruneDepth;

#[allow(private_bounds)]
pub trait TreeContextIteratorBase<Value, Children>:
    crate::Sealed + StreamingIterator<Item = TreeContext<Value, Children>>
where
    Self: Sized,
{
    /// For use in tree_iterators_rs internals. Consumers should prefer using one of the following
    /// public APIs to accomplish their task:
    /// 1. [`prune`](TreeContextIteratorBase::prune)
    /// 2. [`prune_depth`](TreeContextIteratorBase::prune_depth)
    /// 3. [`prune_context`](TreeContextIteratorBase::prune_context)
    fn prune_current_subtree(&mut self);

    /// Prune is a tree-based analog to [`filter`](core::iter::Iterator::filter).
    /// Uses the given closure to determine if each subtree in this tree should be pruned.
    ///
    /// Given an element the closure must return true or false. Any nodes in the tree for
    /// which this evaluates to true will be pruned out of the resulting tree. If the root node is pruned,
    /// this will return [`None`].
    ///
    /// The callback is called on the nodes in a depth first preorder traversal order (see
    /// [`dfs_preorder`](crate::prelude::OwnedTreeNode::dfs_preorder) for more details). If a
    /// node is determined to be pruned, its entire subtree will be pruned without calling the
    /// callback on its descendent nodes.
    ///
    /// ### Basic usage:
    /// ```rust
    /// use tree_iterators_rs::prelude::{BinaryTree, OwnedBinaryTreeNode};
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
    /// println!("{:#?}", tree);
    /// ```
    /// The output for this code would be the following. A couple notes about this output:
    /// 1. the node with a value of '1' has been removed
    /// 2. the callback is never called on the node with a value of '3' since it is already
    ///    determined to be pruned once '1' has been evaluated.
    ///
    /// ```text
    /// 0
    /// 1
    /// 2
    /// Some(
    ///     BinaryTree {
    ///         value: 0,
    ///         left: None,
    ///         right: Some(
    ///             BinaryTree {
    ///                 value: 2,
    ///                 left: None,
    ///                 right: None,
    ///             },
    ///         ),
    ///     },
    /// )
    /// ```
    fn prune<F>(self, f: F) -> Prune<Value, Children, Self, F>
    where
        F: FnMut(&Value) -> bool,
    {
        Prune::new(self, f)
    }

    fn prune_context<F>(self, f: F) -> PruneContext<Value, Children, Self, F>
    where
        F: FnMut(&TreeContext<Value, Children>) -> bool,
    {
        PruneContext::new(self, f)
    }

    fn prune_depth(self, depth: usize) -> PruneDepth<Value, Children, Self> {
        PruneDepth {
            inner: self,
            depth,
            value: Default::default(),
            children: Default::default(),
        }
    }

    fn map_tree<F, Output>(self, f: F) -> Map<Value, Children, Self, F, Output>
    where
        F: FnMut(&Value) -> Output,
    {
        Map::new(self, f)
    }

    fn map_tree_context<F, Output>(self, f: F) -> MapContext<Value, Children, Self, F, Output>
    where
        F: FnMut(&TreeContext<Value, Children>) -> Output,
    {
        MapContext::new(self, f)
    }
}

pub trait TreeContextIterator<Value, Children>: TreeContextIteratorBase<Value, Children>
where
    Self: Sized,
{
    fn collect_tree(mut self) -> Option<Tree<Value>> {
        let mut keeping_stack: Vec<Tree<Value>> = Vec::new();
        while let Some(item) = self.next() {
            while keeping_stack.len() > item.depth() {
                let popped = keeping_stack
                    .pop()
                    .expect("the keeping stack to always have an item");

                let last_keeping_children = keeping_stack
                    .last_mut()
                    .expect("there to always be an item in the keeping stack.");

                last_keeping_children.children.push(popped);
            }

            let target = item
                .ancestors
                .last()
                .expect("ancestors to always be non-empty");
            core::mem::forget(unsafe { core::ptr::read(target as *const Value) });
            let new_owner = unsafe { core::ptr::read(target as *const Value) };

            keeping_stack.push(Tree {
                value: new_owner,
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

    fn fold_tree<F, Output>(mut self, mut f: F) -> Option<Output>
    where
        F: FnMut(Vec<Output>, Value) -> Output,
    {
        let mut inversion_stack: Vec<Value> = Vec::new();
        let mut folded_so_far: Vec<Vec<Output>> = Vec::new();
        while let Some(item) = self.next() {
            while folded_so_far.len() > (item.ancestors().len() - 1) {
                let items = folded_so_far.pop().unwrap();
                let value_to_fold = inversion_stack.pop().unwrap();
                let folded = f(items, value_to_fold);
                folded_so_far.last_mut().unwrap().push(folded);
            }

            let target = item
                .ancestors
                .last()
                .expect("ancestors to always be non-empty");
            core::mem::forget(unsafe { core::ptr::read(target as *const Value) });
            let new_owner = unsafe { core::ptr::read(target as *const Value) };
            inversion_stack.push(new_owner);
            folded_so_far.push(Vec::new())
        }

        while folded_so_far.len() > 1 {
            let items = folded_so_far.pop().unwrap();
            let value_to_fold = inversion_stack.pop().unwrap();
            let folded = f(items, value_to_fold);
            folded_so_far.last_mut().unwrap().push(folded);
        }

        if let Some(root) = inversion_stack.pop() {
            Some(f(folded_so_far.pop().unwrap_or_default(), root))
        } else {
            None
        }
    }
}

pub trait BinaryTreeContextIterator<Value, Children>:
    TreeContextIteratorBase<Value, Children>
where
    Self: Sized,
{
    fn collect_tree(mut self) -> Option<BinaryTree<Value>> {
        let mut keeping_stack: Vec<(usize, BinaryTree<Value>)> = Vec::new();
        while let Some(item) = self.next() {
            while keeping_stack.len() > item.depth() {
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

            let target = item
                .ancestors
                .last()
                .expect("ancestors to always be non-empty");
            core::mem::forget(unsafe { core::ptr::read(target as *const Value) });
            let new_owner = unsafe { core::ptr::read(target as *const Value) };

            let index = item.path().last().map(|i| *i).unwrap_or_default();

            keeping_stack.push((
                index,
                BinaryTree {
                    value: new_owner,
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

    fn fold_tree<F, Output>(mut self, mut f: F) -> Option<Output>
    where
        F: FnMut([Option<Output>; 2], Value) -> Output,
    {
        let mut inversion_stack = Vec::new();
        let mut folded_so_far = Vec::new();
        let mut paths = Vec::new();
        while let Some(item) = self.next() {
            while folded_so_far.len() > (item.ancestors().len() - 1) {
                let items = folded_so_far.pop().unwrap();
                let value_to_fold = inversion_stack.pop().unwrap();
                let path_segment = paths.pop().unwrap();
                let folded = f(items, value_to_fold);
                folded_so_far.last_mut().unwrap()[path_segment] = Some(folded);
            }

            let target = item
                .ancestors
                .last()
                .expect("ancestors to always be non-empty");
            core::mem::forget(unsafe { core::ptr::read(target as *const Value) });
            let new_owner = unsafe { core::ptr::read(target as *const Value) };
            inversion_stack.push(new_owner);
            folded_so_far.push(Default::default());
            paths.push(item.path().last().map(|i| *i).unwrap_or_default());
        }

        while folded_so_far.len() > 1 {
            let items = folded_so_far.pop().unwrap();
            let value_to_fold = inversion_stack.pop().unwrap();
            let path_segment = paths.pop().unwrap();
            let folded = f(items, value_to_fold);
            folded_so_far.last_mut().unwrap()[path_segment] = Some(folded);
        }

        if let Some(root) = inversion_stack.pop() {
            Some(f(folded_so_far.pop().unwrap_or_default(), root))
        } else {
            None
        }
    }
}
