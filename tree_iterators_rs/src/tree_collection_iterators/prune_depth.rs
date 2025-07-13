use core::marker::PhantomData;

use super::{BinaryTreeCollectionIterator, TreeCollectionIterator, TreeCollectionIteratorBase};

pub struct CollectionPruneDepth<Value, Children, Inner>
where
    Inner: TreeCollectionIteratorBase<Value, Children>,
{
    pub(crate) value: PhantomData<Value>,
    pub(crate) children: PhantomData<Children>,
    pub(crate) inner: Inner,
    pub(crate) depth: usize,
}

impl<Value, Children, Inner> Iterator for CollectionPruneDepth<Value, Children, Inner>
where
    Inner: TreeCollectionIteratorBase<Value, Children>,
{
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        if let Some(value) = self.inner.next() {
            if self.current_depth() == self.depth {
                self.prune_current_subtree();
            }

            return Some(value);
        }

        None
    }
}

impl<Value, Children, Inner> TreeCollectionIteratorBase<Value, Children>
    for CollectionPruneDepth<Value, Children, Inner>
where
    Inner: TreeCollectionIteratorBase<Value, Children>,
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

impl<Value, Children, Inner> TreeCollectionIterator<Value, Children>
    for CollectionPruneDepth<Value, Children, Inner>
where
    Inner: TreeCollectionIterator<Value, Children>,
{
}

impl<Value, Children, Inner> BinaryTreeCollectionIterator<Value, Children>
    for CollectionPruneDepth<Value, Children, Inner>
where
    Inner: BinaryTreeCollectionIterator<Value, Children>,
{
}
