use core::marker::PhantomData;

use super::{BinaryTreeCollectionIterator, TreeCollectionIterator, TreeCollectionIteratorBase};

pub struct CollectionMap<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeCollectionIteratorBase<Value, Children>,
    F: FnMut(Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: InnerIter,
    f: F,
}

impl<Value, Children, InnerIter, F, Output> CollectionMap<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeCollectionIteratorBase<Value, Children>,
    F: FnMut(Value) -> Output,
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

impl<Value, Children, InnerIter, F, Output> Iterator
    for CollectionMap<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeCollectionIteratorBase<Value, Children>,
    F: FnMut(Value) -> Output,
{
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|value| (self.f)(value))
    }
}

impl<Value, Children, InnerIter, F, Output> TreeCollectionIteratorBase<Output, ()>
    for CollectionMap<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeCollectionIteratorBase<Value, Children>,
    F: FnMut(Value) -> Output,
{
    fn current_path(&self) -> &[usize] {
        self.inner.current_path()
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F, Output> TreeCollectionIterator<Output, ()>
    for CollectionMap<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeCollectionIterator<Value, Children>,
    F: FnMut(Value) -> Output,
{
}

impl<Value, Children, InnerIter, F, Output> BinaryTreeCollectionIterator<Output, ()>
    for CollectionMap<Value, Children, InnerIter, F, Output>
where
    InnerIter: BinaryTreeCollectionIterator<Value, Children>,
    F: FnMut(Value) -> Output,
{
}
