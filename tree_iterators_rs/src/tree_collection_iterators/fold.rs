use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::prelude::{BinaryTreeCollectionIterator, TreeCollectionIterator};

pub struct Fold<Value, Children, Inner, F, Output>
where
    Inner: TreeCollectionIterator<Value, Children>,
    F: FnMut(Vec<Output>, Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Output>,
    inner: Inner,
    f: F,
    lookahead: Option<Value>,
}

impl<Value, Children, Inner, F, Output> Fold<Value, Children, Inner, F, Output>
where
    Inner: TreeCollectionIterator<Value, Children>,
    F: FnMut(Vec<Output>, Value) -> Output,
{
    pub(crate) fn new(inner: Inner, f: F) -> Self {
        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            phantom3: Default::default(),
            inner,
            f,
            lookahead: None,
        }
    }
}

impl<Value, Children, Inner, F, Output> Iterator for Fold<Value, Children, Inner, F, Output>
where
    Inner: TreeCollectionIterator<Value, Children>,
    F: FnMut(Vec<Output>, Value) -> Output,
{
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inversion_stack: Vec<Value> = Vec::new();
        let mut folded_so_far: Vec<Vec<Output>> = Vec::new();

        loop {
            let item = if let Some(item) = self.lookahead.take() {
                item
            } else if let Some(item) = self.inner.next() {
                item
            } else {
                break;
            };

            while folded_so_far.len() > self.inner.current_depth() {
                let items = folded_so_far.pop().unwrap();
                let value_to_fold = inversion_stack.pop().unwrap();
                let folded = (&mut self.f)(items, value_to_fold);
                if let Some(last) = folded_so_far.last_mut() {
                    last.push(folded);
                } else {
                    self.lookahead = Some(item);
                    return Some(folded);
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
                return Some(folded);
            }
        }

        None
    }
}

pub struct BinaryFold<Value, Children, Inner, F, Output>
where
    Inner: BinaryTreeCollectionIterator<Value, Children>,
    F: FnMut([Option<Output>; 2], Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Output>,
    inner: Inner,
    f: F,
    lookahead: Option<Value>,
}

impl<Value, Children, Inner, F, Output> BinaryFold<Value, Children, Inner, F, Output>
where
    Inner: BinaryTreeCollectionIterator<Value, Children>,
    F: FnMut([Option<Output>; 2], Value) -> Output,
{
    pub(crate) fn new(inner: Inner, f: F) -> Self {
        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            phantom3: Default::default(),
            inner,
            f,
            lookahead: None,
        }
    }
}

impl<Value, Children, Inner, F, Output> Iterator for BinaryFold<Value, Children, Inner, F, Output>
where
    Inner: BinaryTreeCollectionIterator<Value, Children>,
    F: FnMut([Option<Output>; 2], Value) -> Output,
{
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inversion_stack: Vec<Value> = Vec::new();
        let mut folded_so_far: Vec<(usize, [Option<Output>; 2])> = Vec::new();
        loop {
            let item = if let Some(item) = self.lookahead.take() {
                item
            } else if let Some(item) = self.inner.next() {
                item
            } else {
                break;
            };

            while folded_so_far.len() > self.inner.current_depth() {
                let items = folded_so_far.pop().unwrap();
                let value_to_fold = inversion_stack.pop().unwrap();
                let folded = (&mut self.f)(items.1, value_to_fold);
                if let Some(last) = folded_so_far.last_mut() {
                    last.1[items.0] = Some(folded);
                } else {
                    self.lookahead = Some(item);
                    return Some(folded);
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
                return Some(folded);
            }
        }

        None
    }
}
