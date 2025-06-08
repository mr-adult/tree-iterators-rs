use core::marker::PhantomData;

use super::{FallibleBinaryTreeIterator, FallibleTreeIterator, FallibleTreeIteratorBase};

pub struct FallibleMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleTreeIteratorBase<Value, Children, Err>,
    F: FnMut(Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Err>,
    inner: InnerIter,
    f: F,
}

impl<Value, Children, Err, InnerIter, F, Output>
    FallibleMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleTreeIteratorBase<Value, Children, Err>,
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
    for FallibleMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleTreeIteratorBase<Value, Children, Err>,
    F: FnMut(Value) -> Output,
{
    type Item = Result<Output, Err>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|result| result.map(|value| (&mut self.f)(value)))
    }
}

impl<Value, Children, Err, InnerIter, F, Output> FallibleTreeIteratorBase<Output, (), Err>
    for FallibleMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleTreeIteratorBase<Value, Children, Err>,
    F: FnMut(Value) -> Output,
{
    fn current_path(&self) -> &[usize] {
        self.inner.current_path()
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, Err, InnerIter, F, Output> FallibleTreeIterator<Output, (), Err>
    for FallibleMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleTreeIterator<Value, Children, Err>,
    F: FnMut(Value) -> Output,
{
}

impl<Value, Children, Err, InnerIter, F, Output> FallibleBinaryTreeIterator<Output, (), Err>
    for FallibleMap<Value, Children, Err, InnerIter, F, Output>
where
    InnerIter: FallibleBinaryTreeIterator<Value, Children, Err>,
    F: FnMut(Value) -> Output,
{
}
