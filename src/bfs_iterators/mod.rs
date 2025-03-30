pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! bfs_next {
    ($get_value_and_children: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(root) = self.root.take() {
                let (value, children) = root.$get_value_and_children();
                self.traversal_queue.push_back(children.into_iter());
                return Some(value);
            }

            loop {
                if let Some(next_queue) = self.traversal_queue.get_mut(0) {
                    if let Some(next) = next_queue.next() {
                        let (value, children) = next.$get_value_and_children();
                        self.traversal_queue.push_back(children.into_iter());
                        break Some(value);
                    }

                    self.traversal_queue.pop_front();
                } else {
                    break None;
                }
            }
        }
    };
}

macro_rules! bfs_streaming_iterator_impl {
    ($get_value_and_children: ident) => {
        fn advance(&mut self) {
            if self.is_root {
                self.is_root = false;
                return;
            }

            let mut children = core::mem::MaybeUninit::uninit();
            core::mem::swap(&mut children, &mut self.current_context.children);
            self.iterator_queue
                .push_back(unsafe { children.assume_init() }.into_iter());

            loop {
                let iter = if let Some(iter) = self.iterator_queue.get_mut(0) {
                    iter
                } else {
                    self.current_context.ancestors.clear();
                    break;
                };

                if let Some(next) = iter.next() {
                    if self.current_context.ancestors.len() == self.traversal_stack.len() + 2 {
                        self.pop_from_item_stack();
                    }

                    self.current_context.path.push(self.path_counter);
                    self.path_counter += 1;

                    let (value, children) = next.$get_value_and_children();
                    self.current_context.ancestors.push(value);
                    self.current_context.children = core::mem::MaybeUninit::new(children);
                    break;
                }

                self.path_counter = 0;
                if self.current_context.ancestors.len() == self.traversal_stack.len() + 2 {
                    self.pop_from_item_stack();
                }

                let top_of_traversal_stack = if self.traversal_stack.is_empty() {
                    &mut self.tree_cache
                } else {
                    self.traversal_stack.last_mut().unwrap()
                };

                if !top_of_traversal_stack.children.is_empty() {
                    top_of_traversal_stack.children.push_front(None);
                } else {
                    // used up all the values, so just pop it
                    while self.traversal_stack.len() > 0
                        && (self.traversal_stack.last().unwrap().children.len() <= 1)
                    {
                        self.traversal_stack.pop();
                        self.current_context.ancestors.pop();
                        self.current_context.path.pop();
                    }
                }

                self.advance_dfs();
                self.iterator_queue.pop_front();
            }
        }

        fn get(&self) -> Option<&Self::Item> {
            if self.current_context.ancestors.is_empty() {
                None
            } else {
                Some(&self.current_context)
            }
        }
    };
}

macro_rules! get_mut {
    () => {
        fn get_mut(&mut self) -> Option<&mut Self::Item> {
            if self.current_context.ancestors.is_empty() {
                None
            } else {
                Some(&mut self.current_context)
            }
        }
    };
}

macro_rules! bfs_advance_iterator {
    () => {
        fn advance_dfs(&mut self) {
            let starting_depth = self.current_context.ancestors.len();
            loop {
                let tree_node = self
                    .traversal_stack
                    .last_mut()
                    .unwrap_or(&mut self.tree_cache);

                let child = if let Some(child) = tree_node.children.pop_front() {
                    child
                } else {
                    // just let the value get dropped
                    tree_node.children.clear();
                    tree_node.children.shrink_to_fit();
                    break;
                };

                if let Some(mut value) = child {
                    self.current_context
                        .ancestors
                        .push(core::mem::take(&mut value.value).unwrap());
                    self.current_context.path.push(value.path_segment);
                    let has_children = !value.children.is_empty();
                    self.traversal_stack.push(value);
                    if !has_children && self.current_context.ancestors.len() >= starting_depth {
                        break;
                    } else {
                        continue;
                    }
                }

                if tree_node.children.is_empty() {
                    // reclaim that memory
                    tree_node.children.shrink_to_fit();
                } else {
                    // reserve a spot for the current value
                    // once we finish with it.
                    tree_node.children.push_back(None);
                }

                if self.current_context.ancestors.len() > 1 {
                    let target = self.traversal_stack.last_mut().unwrap();
                    target.value = self.current_context.ancestors.pop();
                    target.path_segment = self.current_context.path.pop().unwrap();
                }

                if let Some(popped) = self.traversal_stack.pop() {
                    let parent = if self.traversal_stack.len() < 1 {
                        &mut self.tree_cache
                    } else {
                        self.traversal_stack.last_mut().unwrap()
                    };

                    parent.children.push_back(Some(popped));
                }
            }
        }

        fn pop_from_item_stack(&mut self) {
            if self.current_context.ancestors.len() == 1 {
                return;
            }

            let tree_node = match self.current_context.ancestors.len() {
                0 | 1 => panic!("item stack len should never be 0 or 1 here!"),
                2 => &mut self.tree_cache,
                _ => self
                    .traversal_stack
                    .get_mut(self.current_context.ancestors.len() - 3)
                    .unwrap(),
            };

            tree_node.children.push_back(Some(TreeNodeVecDeque {
                value: Some(self.current_context.ancestors.pop().unwrap()),
                path_segment: self.current_context.path.pop().unwrap(),
                children: VecDeque::new(),
            }));
        }
    };
}

macro_rules! bfs_binary_streaming_iterator_impl {
    ($get_value_and_children: ident) => {
        fn advance(&mut self) {
            if self.is_root {
                self.is_root = false;
                return;
            }

            loop {
                let iter = if let Some(iter) = self.iterator_queue.get_mut(0) {
                    iter
                } else {
                    self.item_stack.clear();
                    return;
                };

                if let Some(next) = iter.next() {
                    if self.item_stack.len() == self.traversal_stack.len() + 2 {
                        self.pop_from_item_stack();
                    }

                    let (value, children) = next.$get_value_and_children();
                    self.item_stack.push(value);
                    self.iterator_queue.push_back(children.into_iter());
                    break;
                }

                if self.item_stack.len() == self.traversal_stack.len() + 2 {
                    self.pop_from_item_stack();
                }

                let top_of_traversal_stack = if self.traversal_stack.is_empty() {
                    &mut self.tree_cache
                } else {
                    self.traversal_stack.last_mut().unwrap()
                };

                if !top_of_traversal_stack.children.is_empty() {
                    top_of_traversal_stack.children.push_front(None);
                } else {
                    // used up all the values, so just pop it
                    while self.traversal_stack.len() > 0
                        && (self.traversal_stack.last().unwrap().children.len() <= 1)
                    {
                        self.traversal_stack.pop();
                        self.item_stack.pop();
                    }
                }

                self.advance_dfs();
                self.iterator_queue.pop_front();
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

macro_rules! get_mut_binary {
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

macro_rules! bfs_binary_advance_iterator {
    ($get_value_and_children: ident) => {
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
                    // just let the value get dropped
                    tree_node.children.clear();
                    tree_node.children.shrink_to_fit();
                    break;
                };

                if let Some(mut value) = child {
                    self.item_stack
                        .push(core::mem::take(&mut value.value).unwrap());
                    let has_children = !value.children.is_empty();
                    self.traversal_stack.push(value);
                    if !has_children && self.item_stack.len() >= starting_depth {
                        break;
                    } else {
                        continue;
                    }
                }

                if tree_node.children.is_empty() {
                    // reclaim that memory
                    tree_node.children.shrink_to_fit();
                } else {
                    // reserve a spot for the current value
                    // once we finish with it.
                    tree_node.children.push_back(None);
                }

                if self.item_stack.len() > 1 {
                    let target = self.traversal_stack.last_mut().unwrap();
                    target.value = self.item_stack.pop();
                }

                if let Some(popped) = self.traversal_stack.pop() {
                    let parent = if self.traversal_stack.len() < 1 {
                        &mut self.tree_cache
                    } else {
                        self.traversal_stack.last_mut().unwrap()
                    };

                    parent.children.push_back(Some(popped));
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

            tree_node.children.push_back(Some(TreeNodeVecDeque {
                value: Some(self.item_stack.pop().unwrap()),
                path_segment: 0,
                children: VecDeque::new(),
            }));
        }
    };
}

pub(crate) use bfs_advance_iterator;
pub(crate) use bfs_binary_advance_iterator;
pub(crate) use bfs_binary_streaming_iterator_impl;
pub(crate) use bfs_next;
pub(crate) use bfs_streaming_iterator_impl;
pub(crate) use get_mut;
pub(crate) use get_mut_binary;

#[derive(Debug, Clone)]
pub(crate) struct TreeNodeVecDeque<T> {
    pub(crate) value: Option<T>,
    pub(crate) path_segment: usize,
    pub(crate) children: alloc::collections::VecDeque<Option<Self>>,
}

impl<T> Default for TreeNodeVecDeque<T> {
    fn default() -> Self {
        Self {
            value: None,
            path_segment: 0,
            children: alloc::collections::VecDeque::new(),
        }
    }
}
