pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! bfs_next {
    ($get_value_and_children: ident) => {
        fn bfs_next(&mut self) -> bool {
            if self.is_root {
                self.is_root = false;
                return true;
            }

            loop {
                if self.iterator_queue.is_empty() {
                    self.item_stack.clear();
                    return false;
                }

                if self.item_stack.len() == self.traversal_stack.len() + 2 {
                    self.pop_from_item_stack();
                }

                let iter = &mut self.iterator_queue[0];

                if let Some(next) = iter.next() {
                    let (value, children) = next.$get_value_and_children();
                    self.item_stack.push(value);

                    let mut peekable_children = children.into_iter().peekable();
                    let has_children = peekable_children.peek().is_some();

                    self.iterator_queue.push_back(peekable_children);
                    return has_children;
                }

                let top_of_traversal_stack = self
                    .traversal_stack
                    .last_mut()
                    .unwrap_or(&mut self.tree_cache);

                if !top_of_traversal_stack.children.is_empty() {
                    top_of_traversal_stack.children.push_front(None);
                } else {
                    // used up all the values, so just pop it
                    while let Some(last) = self.traversal_stack.last() {
                        if last.children.len() > 1 {
                            break;
                        }

                        self.traversal_stack.pop();
                        self.item_stack.pop();
                    }
                }

                self.advance_dfs();
                self.iterator_queue.pop_front();
            }
        }

        fn advance_dfs(&mut self) {
            let starting_depth = self.item_stack.len();
            loop {
                let tree_node = self
                    .traversal_stack
                    .last_mut()
                    .unwrap_or(&mut self.tree_cache);

                let child = if let Some(child) = tree_node.children.pop_front() {
                    child
                } else {
                    // Reclaim that memory
                    tree_node.children.clear();
                    tree_node.children.shrink_to_fit();
                    break;
                };

                if let Some(mut value) = child {
                    self.item_stack.push(unsafe { value.value.assume_init() });
                    value.value = core::mem::MaybeUninit::uninit();
                    let has_children = !value.children.is_empty();
                    self.traversal_stack.push(value);
                    if !has_children && self.item_stack.len() >= starting_depth {
                        break;
                    } else {
                        continue;
                    }
                }

                if !tree_node.children.is_empty() {
                    tree_node.children.push_back(None);
                } else {
                    // let the child be dropped from memory
                    tree_node.children.clear();
                    tree_node.children.shrink_to_fit();
                }

                if self.item_stack.len() > 1 {
                    let target = self.traversal_stack.last_mut().unwrap();
                    target.value = core::mem::MaybeUninit::new(self.item_stack.pop().unwrap());
                }

                if let Some(popped) = self.traversal_stack.pop() {
                    let parent = if let Some(last) = self.traversal_stack.last_mut() {
                        last
                    } else {
                        &mut self.tree_cache
                    };

                    parent.children.push_back(Some(popped));
                }
            }
        }

        fn pop_from_item_stack(&mut self) {
            let tree_node = match self.item_stack.len() {
                0 => panic!("item stack len should never be 0 or 1 here!"),
                1 => return,
                2 => &mut self.tree_cache,
                _ => self
                    .traversal_stack
                    .get_mut(self.item_stack.len() - 3)
                    .unwrap(),
            };

            tree_node.children.push_back(Some(TreeNodeVecDeque {
                value: core::mem::MaybeUninit::new(self.item_stack.pop().unwrap()),
                path_segment: 0,
                children: VecDeque::new(),
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
