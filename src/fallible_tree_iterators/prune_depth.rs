use core::marker::PhantomData;

use super::{FallibleBinaryTreeIterator, FallibleTreeIterator, FallibleTreeIteratorBase};

pub struct FalliblePruneDepth<Value, Children, Err, Inner>
where
    Inner: FallibleTreeIteratorBase<Value, Children, Err>,
{
    pub(crate) value: PhantomData<Value>,
    pub(crate) children: PhantomData<Children>,
    pub(crate) err: PhantomData<Err>,
    pub(crate) inner: Inner,
    pub(crate) depth: usize,
}

impl<Value, Children, Err, Inner> Iterator for FalliblePruneDepth<Value, Children, Err, Inner>
where
    Inner: FallibleTreeIteratorBase<Value, Children, Err>,
{
    type Item = Result<Value, Err>;

    fn next(&mut self) -> Option<Result<Value, Err>> {
        while let Some(value) = self.inner.next() {
            if self.current_depth() == self.depth {
                self.prune_current_subtree();
            }

            return Some(value);
        }

        None
    }
}

impl<Value, Children, Err, Inner> FallibleTreeIteratorBase<Value, Children, Err>
    for FalliblePruneDepth<Value, Children, Err, Inner>
where
    Inner: FallibleTreeIteratorBase<Value, Children, Err>,
{
    fn current_path(&self) -> &[usize] {
        // Since PruneDepth prunes everything at the
        // same depth level, its paths will always
        // match those on the inner iterator
        self.inner.current_path()
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree()
    }
}

impl<Value, Children, Err, Inner> FallibleTreeIterator<Value, Children, Err>
    for FalliblePruneDepth<Value, Children, Err, Inner>
where
    Inner: FallibleTreeIterator<Value, Children, Err>,
{
}

impl<Value, Children, Err, Inner> FallibleBinaryTreeIterator<Value, Children, Err>
    for FalliblePruneDepth<Value, Children, Err, Inner>
where
    Inner: FallibleBinaryTreeIterator<Value, Children, Err>,
{
}
