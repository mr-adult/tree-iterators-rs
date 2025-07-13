use super::TreeCollectionIterator;
use crate::prelude::{BinaryTree, BinaryTreeCollectionIterator, OwnedIntoIteratorOfTrees, Tree};
use alloc::{boxed::Box, vec::Vec};
use core::marker::PhantomData;

pub struct Trees<Value, Children, Inner>
where
    Inner: TreeCollectionIterator<Value, Children>,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: Inner,
    peeked: Option<Value>,
}

impl<Value, Children, Inner> Trees<Value, Children, Inner>
where
    Inner: TreeCollectionIterator<Value, Children>,
{
    pub fn new(collection_iter: Inner) -> Self {
        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            inner: collection_iter,
            peeked: None,
        }
    }
}

impl<Value, Children, Inner> Iterator for Trees<Value, Children, Inner>
where
    Inner: TreeCollectionIterator<Value, Children>,
{
    type Item = Tree<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut keeping_stack: Vec<Tree<Value>> = Vec::new();
        loop {
            let item = if let Some(item) = self.peeked.take() {
                item
            } else if let Some(item) = self.inner.next() {
                item
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
                        return Some(popped);
                    }
                    Some(top) => top.children.push(popped),
                }
            }

            keeping_stack.push(Tree {
                value: item,
                children: Vec::new(),
            });
        }

        while let Some(popped) = keeping_stack.pop() {
            match keeping_stack.last_mut() {
                None => return Some(popped),
                Some(top) => top.children.push(popped),
            }
        }

        None
    }
}

impl<Value, Children, Inner> OwnedIntoIteratorOfTrees<Tree<Value>> for Trees<Value, Children, Inner> where
    Inner: TreeCollectionIterator<Value, Children>
{
}

pub struct BinaryTrees<Value, Children, Inner>
where
    Inner: BinaryTreeCollectionIterator<Value, Children>,
{
    phantom1: PhantomData<Value>,
    phantom2: PhantomData<Children>,
    inner: Inner,
    peeked: Option<Value>,
}

impl<Value, Children, Inner> BinaryTrees<Value, Children, Inner>
where
    Inner: BinaryTreeCollectionIterator<Value, Children>,
{
    pub fn new(collection_iter: Inner) -> Self {
        Self {
            phantom1: Default::default(),
            phantom2: Default::default(),
            inner: collection_iter,
            peeked: None,
        }
    }
}

impl<Value, Children, Inner> Iterator for BinaryTrees<Value, Children, Inner>
where
    Inner: BinaryTreeCollectionIterator<Value, Children>,
{
    type Item = BinaryTree<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut keeping_stack: Vec<(usize, BinaryTree<Value>)> = Vec::new();
        loop {
            let item = if let Some(item) = self.peeked.take() {
                item
            } else if let Some(item) = self.inner.next() {
                item
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
                        return Some(popped.1);
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

        while let Some(popped) = keeping_stack.pop() {
            match keeping_stack.last_mut() {
                None => return Some(popped.1),
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
