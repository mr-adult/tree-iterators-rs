pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! streaming_leaves {
    ($get_value_and_children: ident) => {
        fn advance(&mut self) {
            loop {
                match core::mem::take(&mut self.root) {
                    Some(next) => {
                        let (value, children) = next.$get_value_and_children();
                        self.new_traversal_stack.push(children);
                        self.item_stack.push(value);
                    }
                    None => {
                        let mut pushed_another_iterator = false;
                        loop {
                            let total_stack_len =
                                self.old_traversal_stack.len() + self.new_traversal_stack.len();
                            if total_stack_len < 1 {
                                self.item_stack.pop();
                                return;
                            }
                            let new_stack_len = self.new_traversal_stack.len();
                            if new_stack_len > 0 {
                                match self.new_traversal_stack.get_mut(new_stack_len - 1) {
                                    None => {
                                        self.item_stack.pop();
                                    }
                                    Some(next_iter) => match next_iter.next() {
                                        None => {
                                            if self.item_stack.len() > total_stack_len {
                                                self.item_stack.pop();
                                            }
                                            self.new_traversal_stack.pop();
                                            if pushed_another_iterator {
                                                return;
                                            }
                                        }
                                        Some(node) => {
                                            let (value, children) = node.$get_value_and_children();
                                            if self.item_stack.len() > total_stack_len {
                                                self.item_stack.pop();
                                            }
                                            self.new_traversal_stack.push(children);
                                            self.item_stack.push(value);
                                            pushed_another_iterator = true;
                                        }
                                    },
                                }
                                continue;
                            }
                            let old_traversal_stack_len = self.old_traversal_stack.len();
                            match self
                                .old_traversal_stack
                                .get_mut(old_traversal_stack_len - 1)
                            {
                                None => {
                                    self.item_stack.pop();
                                }
                                Some(next_iter) => match next_iter.next() {
                                    None => {
                                        if self.item_stack.len() > total_stack_len {
                                            self.item_stack.pop();
                                        }
                                        self.old_traversal_stack.pop();
                                    }
                                    Some(node) => {
                                        let (value, children) = node.$get_value_and_children();
                                        if self.item_stack.len() > total_stack_len {
                                            self.item_stack.pop();
                                        }
                                        self.new_traversal_stack.push(children);
                                        self.item_stack.push(value);
                                        pushed_another_iterator = true;
                                    }
                                },
                            }
                        }
                    }
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

pub(crate) use get_mut;
pub(crate) use streaming_leaves;
