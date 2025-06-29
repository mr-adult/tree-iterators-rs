use core::marker::PhantomData;

use super::{
    FallibleBinaryTreeCollectionIterator, FallibleTreeCollectionIterator,
    FallibleTreeCollectionIteratorBase,
};

pub struct CollectionMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleTreeCollectionIteratorBase<Value, Children, Err>,
    F: FnMut(Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Err>,
    inner: InnerIter,
    f: F,
}

impl<Value, Children, Err, InnerIter, F, Output>
    CollectionMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleTreeCollectionIteratorBase<Value, Children, Err>,
    F: FnMut(Value) -> Output,
{
    pub(crate) fn new(iter: InnerIter, f: F) -> Self {
        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            phantom3: Default::default(),
            inner: iter,
            f,
        }
    }
}

impl<Value, Children, Err, InnerIter, F, Output> Iterator
    for CollectionMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleTreeCollectionIteratorBase<Value, Children, Err>,
    F: FnMut(Value) -> Output,
{
    type Item = Result<Output, Err>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|result| result.map(|value| (&mut self.f)(value)))
    }
}

impl<Value, Children, Err, InnerIter, F, Output> FallibleTreeCollectionIteratorBase<Output, (), Err>
    for CollectionMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleTreeCollectionIteratorBase<Value, Children, Err>,
    F: FnMut(Value) -> Output,
{
    fn current_path(&self) -> &[usize] {
        self.inner.current_path()
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, Err, InnerIter, F, Output> FallibleTreeCollectionIterator<Output, (), Err>
    for CollectionMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(Value) -> Output,
{
}

impl<Value, Children, Err, InnerIter, F, Output>
    FallibleBinaryTreeCollectionIterator<Output, (), Err>
    for CollectionMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(Value) -> Output,
{
}
