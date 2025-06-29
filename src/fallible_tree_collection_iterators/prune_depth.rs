use core::marker::PhantomData;

use super::{
    FallibleBinaryTreeCollectionIterator, FallibleTreeCollectionIterator,
    FallibleTreeCollectionIteratorBase,
};

pub struct FallibleCollectionPruneDepth<Value, Children, Err, Inner>
where
    Inner: FallibleTreeCollectionIteratorBase<Value, Children, Err>,
{
    pub(crate) value: PhantomData<Value>,
    pub(crate) children: PhantomData<Children>,
    pub(crate) err: PhantomData<Err>,
    pub(crate) inner: Inner,
    pub(crate) depth: usize,
}

impl<Value, Children, Err, Inner> Iterator
    for FallibleCollectionPruneDepth<Value, Children, Err, Inner>
where
    Inner: FallibleTreeCollectionIteratorBase<Value, Children, Err>,
{
    type Item = Result<Value, Err>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.inner.next() {
            return Some(result.map(|value| {
                if self.current_depth() == self.depth {
                    self.prune_current_subtree();
                }

                value
            }));
        }

        None
    }
}

impl<Value, Children, Err, Inner> FallibleTreeCollectionIteratorBase<Value, Children, Err>
    for FallibleCollectionPruneDepth<Value, Children, Err, Inner>
where
    Inner: FallibleTreeCollectionIteratorBase<Value, Children, Err>,
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

impl<Value, Children, Err, Inner> FallibleTreeCollectionIterator<Value, Children, Err>
    for FallibleCollectionPruneDepth<Value, Children, Err, Inner>
where
    Inner: FallibleTreeCollectionIterator<Value, Children, Err>,
{
}

impl<Value, Children, Err, Inner> FallibleBinaryTreeCollectionIterator<Value, Children, Err>
    for FallibleCollectionPruneDepth<Value, Children, Err, Inner>
where
    Inner: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
{
}
