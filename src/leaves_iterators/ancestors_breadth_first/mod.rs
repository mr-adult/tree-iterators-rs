pub(crate) mod borrow;
pub(crate) mod mut_borrow;
pub(crate) mod owned;

macro_rules! bfs_next {
    ($get_value_and_children: ident) => {
        fn bfs_next(&mut self) -> bool {
            if self.is_root {
                self.is_root = false;
                let first_iter = self
                    .iterator_queue
                    .get_mut(0)
                    .expect("root node to have a children collection on the stack");
                return first_iter.is_some()
                    && first_iter
                        .as_mut()
                        .expect("to be Some() after .is_some() call")
                        .peek()
                        .is_some();
            }

            loop {
                match self.iterator_queue.get_mut(0) {
                    None => {
                        self.item_stack.clear();
                        return false;
                    }
                    Some(iter) => {
                        if let Some(iter) = iter {
                            if let Some(next) = iter.next() {
                                if self.item_stack.len() == self.traversal_stack.len() + 2 {
                                    self.pop_from_item_stack();
                                }
                                let (value, children) = next.$get_value_and_children();
                                self.item_stack.push(value);
                                let has_children;
                                let peekable_children = match children {
                                    None => {
                                        has_children = false;
                                        None
                                    }
                                    Some(iter) => {
                                        let mut peekable_children = iter.peekable();
                                        has_children = peekable_children.peek().is_some();
                                        Some(peekable_children)
                                    }
                                };

                                self.iterator_queue.push_back(peekable_children);
                                return has_children;
                            }
                        }

                        if self.item_stack.len() == self.traversal_stack.len() + 2 {
                            self.pop_from_item_stack();
                        }

                        let top_of_traversal_stack = if self.traversal_stack.len() == 0 {
                            &mut self.tree_cache
                        } else {
                            let stack_len = self.traversal_stack.len();
                            self.traversal_stack.get_mut(stack_len - 1).unwrap()
                        };

                        match &mut top_of_traversal_stack.children {
                            Some(children) => children.push_front(None),
                            // used up all the value, so just pop it
                            None => {
                                while self.traversal_stack.len() > 0
                                    && (self
                                        .traversal_stack
                                        .get(self.traversal_stack.len() - 1)
                                        .unwrap()
                                        .children
                                        .is_none()
                                        || self
                                            .traversal_stack
                                            .get(self.traversal_stack.len() - 1)
                                            .unwrap()
                                            .children
                                            .as_ref()
                                            .unwrap()
                                            .len()
                                            == 1)
                                {
                                    self.traversal_stack.pop();
                                    self.item_stack.pop();
                                }
                            }
                        }

                        self.advance_dfs();
                        self.iterator_queue.pop_front();
                    }
                }
            }
        }

        fn advance_dfs(&mut self) {
            let mut stack_len = self.traversal_stack.len();
            let starting_depth = self.item_stack.len();
            loop {
                let tree_node = if stack_len == 0 {
                    &mut self.tree_cache
                } else {
                    self.traversal_stack.get_mut(stack_len - 1).unwrap()
                };

                match tree_node.children.as_mut() {
                    None => break,
                    Some(children) => {
                        match children.pop_front() {
                            None => {
                                tree_node.children = None;
                                // just let the value get dropped
                            }
                            Some(child) => {
                                match child {
                                    None => {
                                        if children.len() != 0 {
                                            children.push_back(None);
                                        } else {
                                            tree_node.children = None;
                                            // let the child be dropped from memory
                                        }

                                        if self.item_stack.len() > 1 {
                                            let target = self
                                                .traversal_stack
                                                .get_mut(stack_len - 1)
                                                .unwrap();
                                            target.value = self.item_stack.pop();
                                        }

                                        if self.traversal_stack.len() == 0 {
                                            continue;
                                        }
                                        let popped = self.traversal_stack.pop().unwrap();
                                        stack_len -= 1;

                                        let parent = if stack_len < 1 {
                                            &mut self.tree_cache
                                        } else {
                                            self.traversal_stack.get_mut(stack_len - 1).unwrap()
                                        };

                                        parent.children.as_mut().unwrap().push_back(Some(popped));
                                    }
                                    Some(mut value) => {
                                        self.item_stack
                                            .push(core::mem::take(&mut value.value).unwrap());
                                        let has_children = !value.children.is_none();
                                        self.traversal_stack.push(value);
                                        stack_len += 1;
                                        if !has_children && self.item_stack.len() >= starting_depth
                                        {
                                            break;
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        fn pop_from_item_stack(&mut self) {
            if self.item_stack.len() == 1 {
                return;
            }
            let tree_node = match self.item_stack.len() {
                0 | 1 => panic!("item stack len should never be 0 or 1 here!"),
                2 => &mut self.tree_cache,
                _ => self
                    .traversal_stack
                    .get_mut(self.item_stack.len() - 3)
                    .unwrap(),
            };

            let children = match &mut tree_node.children {
                None => {
                    tree_node.children = Some(VecDeque::new());
                    tree_node.children.as_mut().unwrap()
                }
                Some(children) => children,
            };

            children.push_back(Some(TreeNodeVecDeque {
                value: Some(self.item_stack.pop().unwrap()),
                children: None,
            }));
        }
    };
}

macro_rules! get_mut {
    () => {
        fn get_mut(&mut self) -> Option<&mut Self::Item> {
            if self.item_stack.len() == 0 {
                None
            } else {
                Some(&mut self.item_stack[..])
            }
        }
    };
}

macro_rules! streaming_leaves {
    () => {
        fn advance(&mut self) {
            loop {
                if !self.bfs_next() {
                    break;
                }
            }
        }

        fn get(&self) -> Option<&Self::Item> {
            if self.item_stack.len() == 0 {
                None
            } else {
                Some(self.item_stack.as_slice())
            }
        }
    };
}

pub(crate) use bfs_next;
pub(crate) use get_mut;
pub(crate) use streaming_leaves;
