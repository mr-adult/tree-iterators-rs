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
            loop {
                match core::mem::take(&mut self.root) {
                    Some(next) => {
                        let (value, children) = next.$get_value_and_children();
                        self.traversal_stack.push(children.into_iter());
                        self.item_stack.push(value);
                        is_first_iteration = false;
                    }
                    None => loop {
                        let stack_len = self.traversal_stack.len();
                        if stack_len < 1 {
                            self.item_stack.pop();
                            return;
                        }
                        match self.traversal_stack.get_mut(stack_len - 1) {
                            None => {
                                self.item_stack.pop();
                                return;
                            }
                            Some(next_iter) => match next_iter.next() {
                                None => {
                                    if self.item_stack.len() > self.traversal_stack.len() {
                                        self.item_stack.pop();
                                    }
                                    self.traversal_stack.pop();
                                    return;
                                }
                                Some(node) => {
                                    let (value, children) = node.$get_value_and_children();
                                    if is_first_iteration {
                                        self.item_stack.pop();
                                    }
                                    self.traversal_stack.push(children.into_iter());
                                    self.item_stack.push(value);
                                }
                            },
                        }
                        is_first_iteration = false;
                    },
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

pub(crate) use dfs_postorder_next;
pub(crate) use get_mut;
pub(crate) use postorder_streaming_iterator_impl;
