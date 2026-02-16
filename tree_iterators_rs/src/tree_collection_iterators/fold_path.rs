use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::prelude::{BinaryTreeCollectionIterator, TreeCollectionIterator};

pub struct FoldPath<Value, Children, Inner, F, Output>
where
    Inner: TreeCollectionIterator<Value, Children>,
    F: FnMut(Vec<Output>, &[usize], Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Output>,
    inner: Inner,
    f: F,
    lookahead: Option<Value>,
}

impl<Value, Children, Inner, F, Output> FoldPath<Value, Children, Inner, F, Output>
where
    Inner: TreeCollectionIterator<Value, Children>,
    F: FnMut(Vec<Output>, &[usize], Value) -> Output,
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

impl<Value, Children, Inner, F, Output> Iterator for FoldPath<Value, Children, Inner, F, Output>
where
    Inner: TreeCollectionIterator<Value, Children>,
    F: FnMut(Vec<Output>, &[usize], Value) -> Output,
{
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inversion_stack = Vec::new();
        let mut path = Vec::new();
        let mut folded_so_far = Vec::new();

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
                let folded = (self.f)(items, &path, value_to_fold);
                path.pop().unwrap();
                if let Some(last) = folded_so_far.last_mut() {
                    last.push(folded);
                } else {
                    self.lookahead = Some(item);
                    return Some(folded);
                }
            }

            inversion_stack.push(item);
            if path.len() == self.inner.current_depth() + 1 {
                *path.last_mut().unwrap() = self.inner.current_path().last().cloned().unwrap();
            } else {
                path.push(*self.inner.current_path().last().unwrap());
            }
            folded_so_far.push(Vec::new())
        }

        while let Some(items) = folded_so_far.pop() {
            let value_to_fold = inversion_stack.pop().unwrap();
            let folded = (self.f)(items, &path, value_to_fold);
            path.pop().unwrap();
            if let Some(last) = folded_so_far.last_mut() {
                last.push(folded);
            } else {
                return Some(folded);
            }
        }

        None
    }
}

pub struct BinaryFoldPath<Value, Children, Inner, F, Output>
where
    Inner: BinaryTreeCollectionIterator<Value, Children>,
    F: FnMut([Option<Output>; 2], &[usize], Value) -> Output,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Output>,
    inner: Inner,
    f: F,
    lookahead: Option<Value>,
}

impl<Value, Children, Inner, F, Output> BinaryFoldPath<Value, Children, Inner, F, Output>
where
    Inner: BinaryTreeCollectionIterator<Value, Children>,
    F: FnMut([Option<Output>; 2], &[usize], Value) -> Output,
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

impl<Value, Children, Inner, F, Output> Iterator
    for BinaryFoldPath<Value, Children, Inner, F, Output>
where
    Inner: BinaryTreeCollectionIterator<Value, Children>,
    F: FnMut([Option<Output>; 2], &[usize], Value) -> Output,
{
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inversion_stack: Vec<Value> = Vec::new();
        let mut path = Vec::new();
        let mut folded_so_far: Vec<[Option<Output>; 2]> = Vec::new();
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
                let folded = (self.f)(items, &path, value_to_fold);
                let path_segment = path.pop().unwrap();
                if let Some(last) = folded_so_far.last_mut() {
                    last[path_segment] = Some(folded);
                } else {
                    self.lookahead = Some(item);
                    return Some(folded);
                }
            }

            inversion_stack.push(item);
            path.push(*self.inner.current_path().last().unwrap());
            folded_so_far.push([None, None]);
        }

        while let Some(items) = folded_so_far.pop() {
            let value_to_fold = inversion_stack.pop().unwrap();
            let folded = (self.f)(items, &path, value_to_fold);
            let path_segment = path.pop().unwrap();
            if let Some(last) = folded_so_far.last_mut() {
                last[path_segment] = Some(folded);
            } else {
                return Some(folded);
            }
        }

        None
    }
}
