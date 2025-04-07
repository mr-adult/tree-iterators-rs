use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;

use crate::prelude::TreeContext;

use super::{BinaryTreeContextIterator, TreeContextIterator, TreeContextIteratorBase};

pub struct MapContext<Value, Children, InnerIter, F, Output>
where
    InnerIter: StreamingIterator<Item = TreeContext<Value, Children>>,
    F: FnMut(&TreeContext<Value, Children>) -> Output,
{
    inner: InnerIter,
    f: F,
    done: bool,
    current: TreeContext<Output, ()>,
}

impl<Value, Children, InnerIter, F, Output> MapContext<Value, Children, InnerIter, F, Output>
where
    InnerIter: StreamingIterator<Item = TreeContext<Value, Children>>,
    F: FnMut(&TreeContext<Value, Children>) -> Output,
{
    pub(crate) fn new(iter: InnerIter, f: F) -> Self {
        Self {
            inner: iter,
            f,
            done: false,
            current: TreeContext {
                path: Vec::new(),
                ancestors: Vec::new(),
                children: Some(()),
            },
        }
    }
}

impl<Value, Children, InnerIter, F, Output> StreamingIterator
    for MapContext<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeContextIteratorBase<Value, Children>,
    F: FnMut(&TreeContext<Value, Children>) -> Output,
{
    type Item = TreeContext<Output, ()>;

    fn advance(&mut self) {
        self.inner.advance();
        let inner_value = self.inner.get();
        if let Some(inner) = inner_value {
            while !self.current.ancestors().is_empty() && inner.depth() <= self.current.depth() {
                self.current.path.pop();
                self.current.ancestors.pop();
            }

            if let Some(&last_path_segment) = inner.path().last() {
                self.current.path.push(last_path_segment);
            }
            self.current.ancestors.push((&mut self.f)(inner));
        } else {
            self.done = true;
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.done {
            None
        } else {
            Some(&self.current)
        }
    }
}

impl<Value, Children, InnerIter, F, Output> crate::Sealed
    for MapContext<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeContextIteratorBase<Value, Children>,
    F: FnMut(&TreeContext<Value, Children>) -> Output,
{
}

impl<Value, Children, InnerIter, F, Output> TreeContextIteratorBase<Output, ()>
    for MapContext<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeContextIteratorBase<Value, Children>,
    F: FnMut(&TreeContext<Value, Children>) -> Output,
{
    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, InnerIter, F, Output> TreeContextIterator<Output, ()>
    for MapContext<Value, Children, InnerIter, F, Output>
where
    InnerIter: TreeContextIterator<Value, Children>,
    F: FnMut(&TreeContext<Value, Children>) -> Output,
{
}

impl<Value, Children, InnerIter, F, Output> BinaryTreeContextIterator<Output, ()>
    for MapContext<Value, Children, InnerIter, F, Output>
where
    InnerIter: BinaryTreeContextIterator<Value, Children>,
    F: FnMut(&TreeContext<Value, Children>) -> Output,
{
}
