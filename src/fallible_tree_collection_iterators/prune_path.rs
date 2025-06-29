use core::marker::PhantomData;

use alloc::vec::Vec;

use super::{
    FallibleBinaryTreeCollectionIterator, FallibleTreeCollectionIterator,
    FallibleTreeCollectionIteratorBase,
};

pub struct FallibleCollectionPrunePath<Value, Children, Err, InnerIter, F>
where
    InnerIter: FallibleTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(&[usize], &Value) -> bool,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Err>,
    inner: InnerIter,
    f: F,
    pruned_at_each_depth: Vec<usize>,
    current_path: Vec<usize>,
}

impl<Value, Children, Err, InnerIter, F>
    FallibleCollectionPrunePath<Value, Children, Err, InnerIter, F>
where
    InnerIter: FallibleTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(&[usize], &Value) -> bool,
{
    pub(crate) fn new(iter: InnerIter, f: F) -> Self {
        let mut pruned_at_each_depth = Vec::new();
        pruned_at_each_depth.push(0);
        let mut current_path = Vec::new();
        current_path.push(0);

        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            phantom3: Default::default(),
            inner: iter,
            f,
            pruned_at_each_depth,
            current_path,
        }
    }
}

impl<Value, Children, Err, InnerIter, F> Iterator
    for FallibleCollectionPrunePath<Value, Children, Err, InnerIter, F>
where
    InnerIter: FallibleTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(&[usize], &Value) -> bool,
{
    type Item = Result<Value, Err>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.inner.next() {
            let item = match item {
                Ok(item) => item,
                Err(err) => return Some(Err(err)),
            };

            let inner_path_len = self.inner.current_path().len();

            if self.pruned_at_each_depth.len() < inner_path_len {
                // Technically, this only needs to be a single .push() call on each collection,
                // but putting it in a loop will prevent panics if inner is implemented incorrectly.
                loop {
                    self.pruned_at_each_depth.push(0);
                    self.current_path.push(0);
                    if self.pruned_at_each_depth.len() == inner_path_len {
                        break;
                    }
                }
            } else if self.pruned_at_each_depth.len() > inner_path_len {
                self.pruned_at_each_depth.truncate(inner_path_len);
                self.current_path.truncate(inner_path_len);

                if let Some(last_path_segment) = self.current_path.last_mut() {
                    *last_path_segment += 1;
                }
            }

            if (&mut self.f)(self.inner.current_path(), &item) {
                self.prune_current_subtree();
                let current_path_len = self.current_path().len();
                if current_path_len > 0 {
                    let pruned_at_current_depth =
                        &mut self.pruned_at_each_depth[current_path_len - 1];
                    *pruned_at_current_depth += 1;
                }
                continue;
            }

            return Some(Ok(item));
        }

        // Clean up memory just in case our caller keeps this object in memory for a while.
        self.current_path.clear();
        self.pruned_at_each_depth.clear();
        self.current_path.shrink_to_fit();
        self.pruned_at_each_depth.shrink_to_fit();
        None
    }
}

impl<Value, Children, Err, InnerIter, F> FallibleTreeCollectionIteratorBase<Value, Children, Err>
    for FallibleCollectionPrunePath<Value, Children, Err, InnerIter, F>
where
    InnerIter: FallibleTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(&[usize], &Value) -> bool,
{
    fn current_path(&self) -> &[usize] {
        &self.current_path
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, Err, InnerIter, F> FallibleTreeCollectionIterator<Value, Children, Err>
    for FallibleCollectionPrunePath<Value, Children, Err, InnerIter, F>
where
    InnerIter: FallibleTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(&[usize], &Value) -> bool,
{
}

pub struct FallibleBinaryCollectionPrunePath<Value, Children, Err, InnerIter, F>
where
    InnerIter: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(&[usize], &Value) -> bool,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Err>,
    inner: InnerIter,
    f: F,
}

impl<Value, Children, Err, InnerIter, F>
    FallibleBinaryCollectionPrunePath<Value, Children, Err, InnerIter, F>
where
    InnerIter: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(&[usize], &Value) -> bool,
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

impl<Value, Children, Err, InnerIter, F> Iterator
    for FallibleBinaryCollectionPrunePath<Value, Children, Err, InnerIter, F>
where
    InnerIter: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(&[usize], &Value) -> bool,
{
    type Item = Result<Value, Err>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.inner.next() {
            match item {
                Err(err) => return Some(Err(err)),
                Ok(item) => {
                    let path = &self.inner.current_path();
                    if (&mut self.f)(&path, &item) {
                        self.prune_current_subtree();
                        continue;
                    }

                    return Some(Ok(item));
                }
            }
        }

        None
    }
}

impl<Value, Children, Err, InnerIter, F> FallibleTreeCollectionIteratorBase<Value, Children, Err>
    for FallibleBinaryCollectionPrunePath<Value, Children, Err, InnerIter, F>
where
    InnerIter: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(&[usize], &Value) -> bool,
{
    fn current_path(&self) -> &[usize] {
        self.inner.current_path()
    }

    fn prune_current_subtree(&mut self) {
        self.inner.prune_current_subtree();
    }
}

impl<Value, Children, Err, InnerIter, F> FallibleBinaryTreeCollectionIterator<Value, Children, Err>
    for FallibleBinaryCollectionPrunePath<Value, Children, Err, InnerIter, F>
where
    InnerIter: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(&[usize], &Value) -> bool,
{
}
