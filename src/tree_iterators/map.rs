use core::marker::PhantomData;

use super::{BinaryTreeIterator, TreeIterator, TreeIteratorBase};

pub struct Map<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeIteratorBase<Value, Children>,
    F: FnMut(Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: InnerIter,
    f: F,
}

impl<Value, Children, InnerIter, F, Output> Map<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeIteratorBase<Value, Children>,
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

impl<Value, Children, InnerIter, F, Output> crate::Sealed
    for Map<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeIteratorBase<Value, Children>,
    F: FnMut(Value) -> Output,
{
}

impl<Value, Children, InnerIter, F, Output> Iterator for Map<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeIteratorBase<Value, Children>,
    F: FnMut(Value) -> Output,
{
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|value| (&mut self.f)(value))
    }
}

impl<Value, Children, InnerIter, F, Output> TreeIteratorBase<Output, ()>
    for Map<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeIteratorBase<Value, Children>,
    F: FnMut(Value) -> Output,
{
    fn current_path(&self) -> &[usize] {
        self.inner.current_path()
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F, Output> TreeIterator<Output, ()>
    for Map<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(Value) -> Output,
{
}

impl<Value, Children, InnerIter, F, Output> BinaryTreeIterator<Output, ()>
    for Map<Value, Children, InnerIter, F, Output>
where
    InnerIter: BinaryTreeIterator<Value, Children>,
    F: FnMut(Value) -> Output,
{
}
