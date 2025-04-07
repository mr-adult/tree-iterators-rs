use streaming_iterator::StreamingIterator;

use crate::prelude::TreeContext;

use super::{BinaryTreeContextIterator, TreeContextIterator, TreeContextIteratorBase};

pub struct Prune<Value, Children, InnerIter, F>
where
    InnerIter: StreamingIterator<Item = TreeContext<Value, Children>>,
    F: FnMut(&Value) -> bool,
{
    inner: InnerIter,
    f: F,
}

impl<Value, Children, InnerIter, F> Prune<Value, Children, InnerIter, F>
where
    InnerIter: StreamingIterator<Item = TreeContext<Value, Children>>,
    F: FnMut(&Value) -> bool,
{
    pub(crate) fn new(iter: InnerIter, f: F) -> Self {
        Self { inner: iter, f }
    }
}

impl<Value, Children, InnerIter, F> StreamingIterator for Prune<Value, Children, InnerIter, F>
where
    InnerIter: TreeContextIteratorBase<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    type Item = TreeContext<Value, Children>;

    fn advance(&mut self) {
        while let Some(item) = self.inner.next() {
            if !(&mut self.f)(
                item.ancestors()
                    .last()
                    .expect("ancestors is guaranteed to never be empty"),
            ) {
                return;
            }

            self.prune_current_subtree();
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        self.inner.get()
    }
}

impl<Value, Children, InnerIter, F> crate::Sealed for Prune<Value, Children, InnerIter, F>
where
    InnerIter: TreeContextIteratorBase<Value, Children>,
    F: FnMut(&Value) -> bool,
{
}

impl<Value, Children, InnerIter, F> TreeContextIteratorBase<Value, Children>
    for Prune<Value, Children, InnerIter, F>
where
    InnerIter: TreeContextIteratorBase<Value, Children>,
    F: FnMut(&Value) -> bool,
{
    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F> TreeContextIterator<Value, Children>
    for Prune<Value, Children, InnerIter, F>
where
    InnerIter: TreeContextIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
}

impl<Value, Children, InnerIter, F> BinaryTreeContextIterator<Value, Children>
    for Prune<Value, Children, InnerIter, F>
where
    InnerIter: BinaryTreeContextIterator<Value, Children>,
    F: FnMut(&Value) -> bool,
{
}
