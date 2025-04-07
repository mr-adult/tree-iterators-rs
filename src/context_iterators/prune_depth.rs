use core::marker::PhantomData;

use streaming_iterator::StreamingIterator;

use crate::prelude::TreeContext;

use super::{BinaryTreeContextIterator, TreeContextIterator, TreeContextIteratorBase};

pub struct PruneDepth<Value, Children, Inner>
where
    Inner: TreeContextIteratorBase<Value, Children>,
{
    pub(crate) value: PhantomData<Value>,
    pub(crate) children: PhantomData<Children>,
    pub(crate) inner: Inner,
    pub(crate) depth: usize,
}

extern crate std;
impl<Value, Children, Inner> StreamingIterator for PruneDepth<Value, Children, Inner>
where
    Value: std::fmt::Debug,
    Inner: TreeContextIteratorBase<Value, Children>,
{
    type Item = TreeContext<Value, Children>;

    fn advance(&mut self) {
        while let Some(item) = self.inner.next() {
            std::println!("{:?}", item.ancestors());
            if item.depth() <= self.depth {
                break;
            }

            self.inner.prune_current_subtree();
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        self.inner.get()
    }
}

impl<Value, Children, Inner> crate::Sealed for PruneDepth<Value, Children, Inner> where
    Inner: TreeContextIteratorBase<Value, Children>
{
}

impl<Value, Children, Inner> TreeContextIteratorBase<Value, Children>
    for PruneDepth<Value, Children, Inner>
where
    Value: std::fmt::Debug,
    Inner: TreeContextIteratorBase<Value, Children>,
{
    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree()
    }
}

impl<Value, Children, Inner> TreeContextIterator<Value, Children>
    for PruneDepth<Value, Children, Inner>
where
    Value: std::fmt::Debug,
    Inner: TreeContextIterator<Value, Children>,
{
}

impl<Value, Children, Inner> BinaryTreeContextIterator<Value, Children>
    for PruneDepth<Value, Children, Inner>
where
    Value: std::fmt::Debug,
    Inner: BinaryTreeContextIterator<Value, Children>,
{
}
