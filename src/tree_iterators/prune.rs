use core::marker::PhantomData;

use alloc::vec::Vec;

use super::{BinaryTreeIterator, TreeIterator, TreeIteratorBase};

pub struct Prune<Value, Children, InnerIter, F>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: InnerIter,
    f: F,
    pruned_at_each_depth: Vec<usize>,
    current_path: Vec<usize>,
}

impl<Value, Children, InnerIter, F> Prune<Value, Children, InnerIter, F>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    pub(crate) fn new(iter: InnerIter, f: F) -> Self {
        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            inner: iter,
            f,
            pruned_at_each_depth: Vec::new(),
            current_path: Vec::new(),
        }
    }
}

impl<Value, Children, InnerIter, F> Iterator for Prune<Value, Children, InnerIter, F>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.inner.next() {
            let inner_depth = self.inner.current_depth();

            if self.pruned_at_each_depth.len() < inner_depth {
                self.pruned_at_each_depth.push(0);
                self.current_path.push(0);
            }

            let mut matched_up_to_depth = 0;
            let inner_path = self.inner.current_path();

            loop {
                if matched_up_to_depth >= inner_path.len() {
                    self.pruned_at_each_depth.truncate(matched_up_to_depth);
                    self.current_path.truncate(matched_up_to_depth);
                    break;
                }

                let current_path_at_depth = self.current_path[matched_up_to_depth];
                let pruned_at_depth = self.pruned_at_each_depth[matched_up_to_depth];
                let inner_path_at_depth = inner_path[matched_up_to_depth];
                if (current_path_at_depth + pruned_at_depth) != inner_path_at_depth {
                    self.pruned_at_each_depth.truncate(matched_up_to_depth);
                    self.current_path.truncate(matched_up_to_depth);
                    break;
                }

                matched_up_to_depth += 1;
            }

            for depth in matched_up_to_depth..inner_depth {
                let inner_path_at_depth = inner_path[depth];
                if self.pruned_at_each_depth.len() == depth {
                    self.pruned_at_each_depth.push(0);
                }
                let pruned_at_depth = self.pruned_at_each_depth[depth];
                self.current_path
                    .push(inner_path_at_depth - pruned_at_depth);
            }

            if (&mut self.f)(&item) {
                self.prune_current_subtree();
                let current_depth = self.current_depth();
                if current_depth > 0 {
                    let pruned_at_current_depth = &mut self.pruned_at_each_depth[current_depth - 1];
                    *pruned_at_current_depth += 1;
                }
                continue;
            }

            return Some(item);
        }

        self.current_path.clear();
        self.current_path.shrink_to_fit();
        None
    }
}

impl<Value, Children, InnerIter, F> TreeIteratorBase<Value, Children>
    for Prune<Value, Children, InnerIter, F>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    fn current_path(&self) -> &[usize] {
        &self.current_path
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F> TreeIterator<Value, Children>
    for Prune<Value, Children, InnerIter, F>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
}

pub struct BinaryPrune<Value, Children, InnerIter, F>
where
    InnerIter: TreeIteratorBase<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: InnerIter,
    f: F,
}

impl<Value, Children, InnerIter, F> BinaryPrune<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    pub(crate) fn new(iter: InnerIter, f: F) -> Self {
        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            inner: iter,
            f,
        }
    }
}

impl<Value, Children, InnerIter, F> Iterator for BinaryPrune<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.inner.next() {
            if (&mut self.f)(&item) {
                self.prune_current_subtree();
                continue;
            }

            return Some(item);
        }

        None
    }
}

impl<Value, Children, InnerIter, F> TreeIteratorBase<Value, Children>
    for BinaryPrune<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    fn current_path(&self) -> &[usize] {
        self.inner.current_path()
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F> BinaryTreeIterator<Value, Children>
    for BinaryPrune<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
}
