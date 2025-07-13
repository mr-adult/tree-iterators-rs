use core::marker::PhantomData;

use alloc::vec::Vec;

use super::{BinaryTreeIterator, TreeIterator, TreeIteratorBase};

pub struct PrunePath<Value, Children, InnerIter, F>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(&[usize], &Value) -> bool,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: InnerIter,
    f: F,
    pruned_at_each_depth: Vec<usize>,
    current_path: Vec<usize>,
}

impl<Value, Children, InnerIter, F> PrunePath<Value, Children, InnerIter, F>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(&[usize], &Value) -> bool,
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

impl<Value, Children, InnerIter, F> Iterator for PrunePath<Value, Children, InnerIter, F>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(&[usize], &Value) -> bool,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.inner.next() {
            let inner_depth = self.inner.current_depth();

            if self.pruned_at_each_depth.len() < inner_depth {
                // Technically, this only needs to be a single .push() call on each collection,
                // but putting it in a loop will prevent panics if inner is implemented incorrectly.
                loop {
                    self.pruned_at_each_depth.push(0);
                    self.current_path.push(0);
                    if self.pruned_at_each_depth.len() == inner_depth {
                        break;
                    }
                }
            } else if self.pruned_at_each_depth.len() > inner_depth {
                self.pruned_at_each_depth.truncate(inner_depth);
                self.current_path.truncate(inner_depth);

                if let Some(last_path_segment) = self.current_path.last_mut() {
                    *last_path_segment += 1;
                }
            }

            if (self.f)(self.inner.current_path(), &item) {
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

        // Clean up memory just in case our caller keeps this object in memory for a while.
        self.current_path.clear();
        self.pruned_at_each_depth.clear();
        self.current_path.shrink_to_fit();
        self.pruned_at_each_depth.shrink_to_fit();
        None
    }
}

impl<Value, Children, InnerIter, F> TreeIteratorBase<Value, Children>
    for PrunePath<Value, Children, InnerIter, F>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(&[usize], &Value) -> bool,
{
    fn current_path(&self) -> &[usize] {
        &self.current_path
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F> TreeIterator<Value, Children>
    for PrunePath<Value, Children, InnerIter, F>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(&[usize], &Value) -> bool,
{
}

pub struct BinaryPrunePath<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeIterator<Value, Children>,
    F: FnMut(&[usize], &Value) -> bool,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: InnerIter,
    f: F,
}

impl<Value, Children, InnerIter, F> BinaryPrunePath<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeIterator<Value, Children>,
    F: FnMut(&[usize], &Value) -> bool,
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

impl<Value, Children, InnerIter, F> Iterator for BinaryPrunePath<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeIterator<Value, Children>,
    F: FnMut(&[usize], &Value) -> bool,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.inner.next() {
            let path = &self.inner.current_path();
            if (self.f)(path, &item) {
                self.prune_current_subtree();
                continue;
            }

            return Some(item);
        }

        None
    }
}

impl<Value, Children, InnerIter, F> TreeIteratorBase<Value, Children>
    for BinaryPrunePath<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeIterator<Value, Children>,
    F: FnMut(&[usize], &Value) -> bool,
{
    fn current_path(&self) -> &[usize] {
        self.inner.current_path()
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F> BinaryTreeIterator<Value, Children>
    for BinaryPrunePath<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeIterator<Value, Children>,
    F: FnMut(&[usize], &Value) -> bool,
{
}
