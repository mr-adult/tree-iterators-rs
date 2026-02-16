use core::marker::PhantomData;

use super::{BinaryTreeIterator, TreeIterator, TreeIteratorBase};

pub struct MapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeIteratorBase<Value, Children>,
    F: FnMut(&[usize], Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: InnerIter,
    f: F,
}

impl<Value, Children, InnerIter, F, Output> MapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeIteratorBase<Value, Children>,
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
    for MapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeIteratorBase<Value, Children>,
    F: FnMut(&[usize], Value) -> Output,
{
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: self.f cannot borrow path mutably since path is our internal state.
        let path = self.current_path() as *const [usize];
        self.inner
            .next()
            .map(|value| (self.f)(unsafe { &*path }, value))
    }
}

impl<Value, Children, InnerIter, F, Output> TreeIteratorBase<Output, ()>
    for MapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeIteratorBase<Value, Children>,
    F: FnMut(&[usize], Value) -> Output,
{
    fn current_path(&self) -> &[usize] {
        self.inner.current_path()
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F, Output> TreeIterator<Output, ()>
    for MapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeIterator<Value, Children>,
    F: FnMut(&[usize], Value) -> Output,
{
}

impl<Value, Children, InnerIter, F, Output> BinaryTreeIterator<Output, ()>
    for MapPath<Value, Children, InnerIter, F, Output>
where
    InnerIter: BinaryTreeIterator<Value, Children>,
    F: FnMut(&[usize], Value) -> Output,
{
}
