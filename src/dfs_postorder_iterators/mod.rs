pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! dfs_postorder_next {
    ($get_value_and_children: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match core::mem::take(&mut self.root) {
                    Some(next) => {
                        let (value, children) = next.$get_value_and_children();
                        self.traversal_stack.push(children.into_iter());
                        self.item_stack.push(value);
                    }
                    None => loop {
                        let stack_len = self.traversal_stack.len();
                        if stack_len < 1 {
                            return None;
                        }
                        match self.traversal_stack.get_mut(stack_len - 1) {
                            None => return self.item_stack.pop(),
                            Some(next_iter) => match next_iter.next() {
                                None => {
                                    self.traversal_stack.pop();
                                    return self.item_stack.pop();
                                }
                                Some(node) => {
                                    let (value, children) = node.$get_value_and_children();
                                    self.item_stack.push(value);
                                    self.traversal_stack.push(children.into_iter());
                                }
                            },
                        }
                    },
                }
            }
        }
    };
}

macro_rules! postorder_streaming_iterator_impl {
    ($get_value_and_children: ident) => {
        fn advance(&mut self) {
            let mut is_first_iteration = true;
            if let Some(next) = self.root.take() {
                let (value, children) = next.$get_value_and_children();
                self.traversal_stack.push(children.into_iter());
                self.current_context.ancestors.push(value);
                self.current_context.path.push(usize::MAX);
                is_first_iteration = false;
            }

            loop {
                if let Some(top) = self.traversal_stack.last_mut() {
                    if let Some(node) = top.next() {
                        // Path is not populated on the first pass over just the root node.
                        if let Some(last) = self.current_context.path.last_mut() {
                            *last = last.wrapping_add(1);
                        }

                        let (value, children) = node.$get_value_and_children();
                        if is_first_iteration {
                            self.current_context.ancestors.pop();
                        }

                        self.traversal_stack.push(children.into_iter());
                        self.current_context.ancestors.push(value);
                        self.current_context.path.push(usize::MAX);
                        is_first_iteration = false;
                        continue;
                    }

                    if self.current_context.ancestors.len() > self.traversal_stack.len() {
                        self.current_context.ancestors.pop();
                    }

                    self.traversal_stack.pop();
                    self.current_context.path.pop();
                    return;
                } else {
                    self.current_context.ancestors.pop();
                    self.current_context.path.pop();
                    return;
                }
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

macro_rules! postorder_binary_streaming_iterator_impl {
    ($get_value_and_children: ident) => {
        fn advance(&mut self) {
            let mut is_first_iteration = true;
            if let Some(next) = self.root.take() {
                let (value, children) = next.$get_value_and_children();
                self.traversal_stack.push(children.into_iter());
                self.item_stack.push(value);
                is_first_iteration = false;
            }

            loop {
                if let Some(top) = self.traversal_stack.last_mut() {
                    if let Some(node) = top.next() {
                        let (value, children) = node.$get_value_and_children();
                        if is_first_iteration {
                            self.item_stack.pop();
                        }

                        self.traversal_stack.push(children.into_iter());
                        self.item_stack.push(value);
                        is_first_iteration = false;
                        continue;
                    }

                    if self.item_stack.len() > self.traversal_stack.len() {
                        self.item_stack.pop();
                    }
                    self.traversal_stack.pop();
                    return;
                } else {
                    self.item_stack.pop();
                    return;
                }
            }
        }

        fn get(&self) -> Option<&Self::Item> {
            if self.item_stack.len() > 0 {
                Some(&self.item_stack.as_slice())
            } else {
                None
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

pub(crate) use dfs_postorder_next;
pub(crate) use get_mut;
pub(crate) use get_mut_binary;
pub(crate) use postorder_binary_streaming_iterator_impl;
pub(crate) use postorder_streaming_iterator_impl;
