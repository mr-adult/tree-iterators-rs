use core::marker::PhantomData;

use super::{BinaryTreeIterator, TreeIterator, TreeIteratorBase};

pub struct PruneDepth<Value, Children, Inner>
where
    Inner: TreeIteratorBase<Value, Children>,
{
    pub(crate) value: PhantomData<Value>,
    pub(crate) children: PhantomData<Children>,
    pub(crate) inner: Inner,
    pub(crate) depth: usize,
}

impl<Value, Children, Inner> Iterator for PruneDepth<Value, Children, Inner>
where
    Inner: TreeIteratorBase<Value, Children>,
{
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        while let Some(value) = self.inner.next() {
            if self.current_depth() == self.depth {
                self.prune_current_subtree();
            }

            return Some(value);
        }

        None
    }
}

impl<Value, Children, Inner> TreeIteratorBase<Value, Children>
    for PruneDepth<Value, Children, Inner>
where
    Inner: TreeIteratorBase<Value, Children>,
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

impl<Value, Children, Inner> TreeIterator<Value, Children> for PruneDepth<Value, Children, Inner> where
    Inner: TreeIterator<Value, Children>
{
}

impl<Value, Children, Inner> BinaryTreeIterator<Value, Children>
    for PruneDepth<Value, Children, Inner>
where
    Inner: BinaryTreeIterator<Value, Children>,
{
}
