use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::{fallible_tree_collection_iterators::{FallibleBinaryTreeCollectionIterator, FallibleTreeCollectionIterator}};

pub struct FallibleFold<Value, Children, Err, Inner, Output, F>
where
    Inner: FallibleTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(Vec<Output>, Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Output>,
    phantom4: PhantomData<Err>,
    inner: Inner,
    f: F,
}

impl<Value, Children, Err, Inner, Output, F> FallibleFold<Value, Children, Err, Inner, Output, F>
where
    Inner: FallibleTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(Vec<Output>, Value) -> Output,
{
    pub(crate) fn new(inner: Inner, f: F) -> Self {
        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            phantom3: Default::default(),
            phantom4: Default::default(),
            inner,
            f,
        }
    }
}

impl<Value, Children, Err, Inner, Output, F> Iterator for FallibleFold<Value, Children, Err, Inner, Output, F>
where
    Inner: FallibleTreeCollectionIterator<Value, Children, Err>,
    F: FnMut(Vec<Output>, Value) -> Output,
{
    type Item = Result<Output, Err>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inversion_stack: Vec<Value> = Vec::new();
        let mut folded_so_far: Vec<Vec<Output>> = Vec::new();
        while let Some(item) = self.inner.next() {
            let item = match item {
                Err(err) => return Some(Err(err)),
                Ok(item) => item,
            };

            while folded_so_far.len() > self.inner.current_depth() {
                let items = folded_so_far.pop().unwrap();
                let value_to_fold = inversion_stack.pop().unwrap();
                let folded = (&mut self.f)(items, value_to_fold);
                if let Some(last) = folded_so_far.last_mut() {
                    last.push(folded);
                } else {
                    return Some(Ok(folded));
                }
            }

            inversion_stack.push(item);
            folded_so_far.push(Vec::new())
        }

        while !folded_so_far.is_empty() {
            let items = folded_so_far.pop().unwrap();
            let value_to_fold = inversion_stack.pop().unwrap();
            let folded = (&mut self.f)(items, value_to_fold);
            if let Some(last) = folded_so_far.last_mut() {
                last.push(folded);
            } else {
                return Some(Ok(folded));
            }
        }

        None
    }
}

pub struct FallibleBinaryFold<Value, Children, Err, Inner, Output, F>
where
    Inner: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
    F: FnMut([Option<Output>; 2], Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Output>,
    phantom4: PhantomData<Err>,
    inner: Inner,
    f: F,
}

impl<Value, Children, Err, Inner, Output, F> FallibleBinaryFold<Value, Children, Err, Inner, Output, F>
where
    Inner: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
    F: FnMut([Option<Output>; 2], Value) -> Output
{
    pub(crate) fn new(inner: Inner, f: F) -> Self {
        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            phantom3: Default::default(),
            phantom4: Default::default(),
            inner,
            f,
        }
    }
}

impl<Value, Children, Err, Inner, Output, F> Iterator for FallibleBinaryFold<Value, Children, Err, Inner, Output, F>
where
    Inner: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
    F: FnMut([Option<Output>; 2], Value) -> Output,
{
    type Item = Result<Output, Err>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inversion_stack: Vec<Value> = Vec::new();
        let mut folded_so_far: Vec<(usize, [Option<Output>; 2])> = Vec::new();
        while let Some(item) = self.inner.next() {
            let item = match item {
                Err(err) => return Some(Err(err)),
                Ok(item) => item,
            };

            while folded_so_far.len() > self.inner.current_depth() {
                let items = folded_so_far.pop().unwrap();
                let value_to_fold = inversion_stack.pop().unwrap();
                let folded = (&mut self.f)(items.1, value_to_fold);
                if let Some(last) = folded_so_far.last_mut() {
                    last.1[items.0] = Some(folded);
                } else {
                    return Some(Ok(folded));
                }
            }

            inversion_stack.push(item);
            folded_so_far.push((*self.inner.current_path().last().unwrap(), [None, None]))
        }

        while !folded_so_far.is_empty() {
            let items = folded_so_far.pop().unwrap();
            let value_to_fold = inversion_stack.pop().unwrap();
            let folded = (&mut self.f)(items.1, value_to_fold);
            if let Some(last) = folded_so_far.last_mut() {
                last.1[items.0] = Some(folded);
            } else {
                return Some(Ok(folded));
            }
        }

        None
    }
}
