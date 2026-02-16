use core::marker::PhantomData;

use super::{BinaryTreeCollectionIterator, TreeCollectionIterator, TreeCollectionIteratorBase};

pub struct CollectionMapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeCollectionIteratorBase<Value, Children>,
    F: FnMut(&[usize], Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: InnerIter,
    f: F,
}

impl<Value, Children, InnerIter, F, Output> CollectionMapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeCollectionIteratorBase<Value, Children>,
    F: FnMut(&[usize], Value) -> Output,
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
    for CollectionMapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeCollectionIteratorBase<Value, Children>,
    F: FnMut(&[usize], Value) -> Output,
{
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: self.f cannot borrow path mutably since path is our internal state.
        let path = unsafe { &*(self.current_path() as *const [usize]) };
        self.inner.next().map(|value| (self.f)(path, value))
    }
}

impl<Value, Children, InnerIter, F, Output> TreeCollectionIteratorBase<Output, ()>
    for CollectionMapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeCollectionIteratorBase<Value, Children>,
    F: FnMut(&[usize], Value) -> Output,
{
    fn current_path(&self) -> &[usize] {
        self.inner.current_path()
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F, Output> TreeCollectionIterator<Output, ()>
    for CollectionMapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeCollectionIterator<Value, Children>,
    F: FnMut(&[usize], Value) -> Output,
{
}

impl<Value, Children, InnerIter, F, Output> BinaryTreeCollectionIterator<Output, ()>
    for CollectionMapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: BinaryTreeCollectionIterator<Value, Children>,
    F: FnMut(&[usize], Value) -> Output,
{
}
