use super::FallibleTreeCollectionIterator;
use crate::{
    fallible_tree_collection_iterators::FallibleBinaryTreeCollectionIterator,
    prelude::{BinaryTree, Tree},
};
use alloc::{boxed::Box, vec::Vec};
use core::marker::PhantomData;

pub struct FallibleTrees<Value, Children, Err, Inner>
where
    Inner: FallibleTreeCollectionIterator<Value, Children, Err>,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Err>,
    inner: Inner,
    peeked: Option<Value>,
}

impl<Value, Children, Err, Inner> FallibleTrees<Value, Children, Err, Inner>
where
    Inner: FallibleTreeCollectionIterator<Value, Children, Err>,
{
    pub fn new(collection_iter: Inner) -> Self {
        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            phantom3: Default::default(),
            inner: collection_iter,
            peeked: None,
        }
    }
}

impl<Value, Children, Err, Inner> Iterator for FallibleTrees<Value, Children, Err, Inner>
where
    Inner: FallibleTreeCollectionIterator<Value, Children, Err>,
{
    type Item = Result<Tree<Value>, Err>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut keeping_stack: Vec<Tree<Value>> = Vec::new();
        loop {
            let item = if let Some(item) = self.peeked.take() {
                item
            } else if let Some(result) = self.inner.next() {
                match result {
                    Err(err) => return Some(Err(err)),
                    Ok(item) => item,
                }
            } else {
                break;
            };

            let current_depth = self.inner.current_path().len();
            while keeping_stack.len() >= current_depth {
                let popped = keeping_stack
                    .pop()
                    .expect("the keeping stack to always have an item");

                match keeping_stack.last_mut() {
                    None => {
                        self.peeked = Some(item);
                        return Some(Ok(popped));
                    }
                    Some(top) => top.children.push(popped),
                }
            }

            keeping_stack.push(Tree {
                value: item,
                children: Vec::new(),
            });
        }

        while !keeping_stack.is_empty() {
            let popped = keeping_stack
                .pop()
                .expect("the keeping stack to always have an item");

            match keeping_stack.last_mut() {
                None => return Some(Ok(popped)),
                Some(top) => top.children.push(popped),
            }
        }

        None
    }
}

pub struct FallibleBinaryTrees<Value, Children, Err, Inner>
where
    Inner: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    phantom3: PhantomData<Err>,
    inner: Inner,
    peeked: Option<Value>,
}

impl<Value, Children, Err, Inner> FallibleBinaryTrees<Value, Children, Err, Inner>
where
    Inner: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
{
    pub fn new(collection_iter: Inner) -> Self {
        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            phantom3: Default::default(),
            inner: collection_iter,
            peeked: None,
        }
    }
}

impl<Value, Children, Err, Inner> Iterator for FallibleBinaryTrees<Value, Children, Err, Inner>
where
    Inner: FallibleBinaryTreeCollectionIterator<Value, Children, Err>,
{
    type Item = Result<BinaryTree<Value>, Err>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut keeping_stack: Vec<(usize, BinaryTree<Value>)> = Vec::new();
        loop {
            let item = if let Some(item) = self.peeked.take() {
                item
            } else if let Some(result) = self.inner.next() {
                match result {
                    Err(err) => return Some(Err(err)),
                    Ok(item) => item,
                }
            } else {
                break;
            };

            let current_depth = self.inner.current_path().len();
            while keeping_stack.len() >= current_depth {
                let popped = keeping_stack
                    .pop()
                    .expect("the keeping stack to always have an item");

                match keeping_stack.last_mut() {
                    None => {
                        self.peeked = Some(item);
                        return Some(Ok(popped.1));
                    }
                    Some(top) => match popped.0 {
                        0 => top.1.left = Some(Box::new(popped.1)),
                        1 => top.1.right = Some(Box::new(popped.1)),
                        _ => panic!("only 0 and 1 are valid indexes of a BinaryTree"),
                    },
                }
            }

            keeping_stack.push((
                *self.inner.current_path().last().unwrap(),
                BinaryTree {
                    value: item,
                    left: None,
                    right: None,
                },
            ));
        }

        while !keeping_stack.is_empty() {
            let popped = keeping_stack
                .pop()
                .expect("the keeping stack to always have an item");

            match keeping_stack.last_mut() {
                None => return Some(Ok(popped.1)),
                Some(top) => match popped.0 {
                    0 => top.1.left = Some(Box::new(popped.1)),
                    1 => top.1.right = Some(Box::new(popped.1)),
                    _ => panic!("only 0 and 1 are valid indexes of a BinaryTree"),
                },
            }
        }

        None
    }
}
