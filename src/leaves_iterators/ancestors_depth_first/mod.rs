pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! streaming_leaves {
    ($get_value_and_children: ident) => {
        fn advance(&mut self) {
            if let Some(root) = self.root.take() {
                let (value, children) = root.$get_value_and_children();
                self.new_traversal_stack.push(children.into_iter());
                self.item_stack.push(value);
            } else {
                self.item_stack.pop();
            }

            let mut pushed_another_iterator = false;
            loop {
                if let Some(new_stack_last) = self.new_traversal_stack.last_mut() {
                    if let Some(node) = new_stack_last.next() {
                        let (value, children) = node.$get_value_and_children();
                        self.new_traversal_stack.push(children.into_iter());
                        self.item_stack.push(value);
                        pushed_another_iterator = true;
                        continue;
                    }
                }

                self.new_traversal_stack.pop();
                if pushed_another_iterator {
                    return;
                }

                if let Some(old_stack_last) = self.old_traversal_stack.last_mut() {
                    if let Some(node) = old_stack_last.next() {
                        let (value, children) = node.$get_value_and_children();
                        self.new_traversal_stack.push(children.into_iter());
                        self.item_stack.push(value);
                        pushed_another_iterator = true;
                        continue;
                    }
                }

                self.old_traversal_stack.pop();
                if pushed_another_iterator {
                    return;
                }

                self.item_stack.pop();
                if self.item_stack.is_empty() {
                    break;
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
