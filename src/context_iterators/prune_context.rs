use streaming_iterator::StreamingIterator;

use crate::prelude::TreeContext;

use super::{BinaryTreeContextIterator, TreeContextIterator, TreeContextIteratorBase};

pub struct PruneContext<Value, Children, InnerIter, F>
where
    InnerIter: StreamingIterator<Item = TreeContext<Value, Children>>,
    F: FnMut(&TreeContext<Value, Children>) -> bool,
{
    inner: InnerIter,
    f: F,
}

impl<Value, Children, InnerIter, F> PruneContext<Value, Children, InnerIter, F>
where
    InnerIter: StreamingIterator<Item = TreeContext<Value, Children>>,
    F: FnMut(&TreeContext<Value, Children>) -> bool,
{
    pub(crate) fn new(iter: InnerIter, f: F) -> Self {
        Self { inner: iter, f }
    }
}

impl<Value, Children, InnerIter, F> StreamingIterator
    for PruneContext<Value, Children, InnerIter, F>
where
    InnerIter: TreeContextIteratorBase<Value, Children>,
    F: FnMut(&TreeContext<Value, Children>) -> bool,
{
    type Item = TreeContext<Value, Children>;

    fn advance(&mut self) {
        while let Some(item) = self.inner.next() {
            if !(&mut self.f)(item) {
                return;
            }

            self.prune_current_subtree();
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        self.inner.get()
    }
}

impl<Value, Children, InnerIter, F> crate::Sealed for PruneContext<Value, Children, InnerIter, F>
where
    InnerIter: TreeContextIteratorBase<Value, Children>,
    F: FnMut(&TreeContext<Value, Children>) -> bool,
{
}

impl<Value, Children, InnerIter, F> TreeContextIteratorBase<Value, Children>
    for PruneContext<Value, Children, InnerIter, F>
where
    InnerIter: TreeContextIteratorBase<Value, Children>,
    F: FnMut(&TreeContext<Value, Children>) -> bool,
{
    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F> TreeContextIterator<Value, Children>
    for PruneContext<Value, Children, InnerIter, F>
where
    InnerIter: TreeContextIterator<Value, Children>,
    F: FnMut(&TreeContext<Value, Children>) -> bool,
{
}

impl<Value, Children, InnerIter, F> BinaryTreeContextIterator<Value, Children>
    for PruneContext<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeContextIterator<Value, Children>,
    F: FnMut(&TreeContext<Value, Children>) -> bool,
{
}
