use core::marker::PhantomData;

use alloc::vec::Vec;

use super::{BinaryTreeCollectionIterator, TreeCollectionIterator, TreeCollectionIteratorBase};

pub struct CollectionPrune<Value, Children, InnerIter, F>
where
    InnerIter: TreeCollectionIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: InnerIter,
    f: F,
    pruned_at_each_depth: Vec<usize>,
    current_path: Vec<usize>,
}

impl<Value, Children, InnerIter, F> CollectionPrune<Value, Children, InnerIter, F>
where
    InnerIter: TreeCollectionIterator<Value, Children>,
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

impl<Value, Children, InnerIter, F> Iterator for CollectionPrune<Value, Children, InnerIter, F>
where
    InnerIter: TreeCollectionIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.inner.next() {
            let inner_path_len = self.inner.current_path().len();

            if self.pruned_at_each_depth.len() < inner_path_len {
                // Technically, this only needs to be a single .push() call on each collection,
                // but putting it in a loop will prevent panics if inner is implemented incorrectly.
                loop {
                    self.pruned_at_each_depth.push(0);
                    self.current_path.push(0);
                    if self.pruned_at_each_depth.len() == inner_path_len {
                        break;
                    }
                }
            } else if self.pruned_at_each_depth.len() > inner_path_len {
                self.pruned_at_each_depth.truncate(inner_path_len);
                self.current_path.truncate(inner_path_len);

                if let Some(last_path_segment) = self.current_path.last_mut() {
                    *last_path_segment += 1;
                }
            }

            if (&mut self.f)(&item) {
                self.prune_current_subtree();
                let current_depth = self.current_path().len();
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

impl<Value, Children, InnerIter, F> TreeCollectionIteratorBase<Value, Children>
    for CollectionPrune<Value, Children, InnerIter, F>
where
    InnerIter: TreeCollectionIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    fn current_path(&self) -> &[usize] {
        &self.current_path
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F> TreeCollectionIterator<Value, Children>
    for CollectionPrune<Value, Children, InnerIter, F>
where
    InnerIter: TreeCollectionIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
}

pub struct BinaryCollectionPrune<Value, Children, InnerIter, F>
where
    InnerIter: TreeCollectionIteratorBase<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: InnerIter,
    f: F,
}

impl<Value, Children, InnerIter, F> BinaryCollectionPrune<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeCollectionIterator<Value, Children>,
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

impl<Value, Children, InnerIter, F> Iterator
    for BinaryCollectionPrune<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeCollectionIterator<Value, Children>,
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

impl<Value, Children, InnerIter, F> TreeCollectionIteratorBase<Value, Children>
    for BinaryCollectionPrune<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeCollectionIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    fn current_path(&self) -> &[usize] {
        self.inner.current_path()
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F> BinaryTreeCollectionIterator<Value, Children>
    for BinaryCollectionPrune<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeCollectionIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
}
