pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! dfs_postorder_next {
    ($get_value_and_children: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                if let Some(root) = self.root.take() {
                    let (value, children) = root.$get_value_and_children();
                    self.traversal_stack.push(children.into_iter());
                    self.item_stack.push(value);
                    continue;
                }

                loop {
                    if let Some(last) = self.traversal_stack.last_mut() {
                        if let Some(next) = last.next() {
                            let (value, children) = next.$get_value_and_children();
                            self.item_stack.push(value);
                            self.traversal_stack.push(children.into_iter());
                            continue;
                        }

                        self.traversal_stack.pop();
                    }

                    return self.item_stack.pop();
                }
            }
        }
    };
}

macro_rules! get_mut_context {
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

macro_rules! postorder_ancestors_streaming_iterator_impl {
    ($get_value_and_children: ident) => {
        fn advance(&mut self) {
            if let Some(next) = self.root.take() {
                let (value, children) = next.$get_value_and_children();
                self.traversal_stack.push(children.into_iter());
                self.item_stack.push(value);
            } else {
                self.item_stack.pop();
            }

            while let Some(top) = self.traversal_stack.last_mut() {
                if let Some(node) = top.next() {
                    let (value, children) = node.$get_value_and_children();

                    self.traversal_stack.push(children.into_iter());
                    self.item_stack.push(value);
                    continue;
                }

                self.traversal_stack.pop();
                break;
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

macro_rules! get_mut_ancestors {
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
pub(crate) use get_mut_ancestors;
pub(crate) use get_mut_context;
pub(crate) use postorder_ancestors_streaming_iterator_impl;
