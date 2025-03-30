pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! dfs_preorder_next {
    ($get_value_and_children: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            let next = self.root.take().or_else(|| loop {
                if let Some(top) = self.traversal_stack.last_mut() {
                    if let Some(value) = top.next() {
                        break Some(value);
                    }

                    self.traversal_stack.pop();
                } else {
                    break None;
                }
            });

            next.map(|node| {
                let (value, children) = node.$get_value_and_children();
                self.traversal_stack.push(children.into_iter());
                value
            })
        }
    };
}

macro_rules! preorder_ancestors_streaming_iterator_impl {
    ($get_value_and_children: ident) => {
        fn advance(&mut self) {
            let next = self.root.take().or_else(|| loop {
                if let Some(top) = self.traversal_stack.last_mut() {
                    if let Some(value) = top.next() {
                        break Some(value);
                    }

                    self.traversal_stack.pop();
                    self.item_stack.pop();
                } else {
                    break None;
                }
            });

            if let Some(next) = next {
                let (value, children) = next.$get_value_and_children();
                self.traversal_stack.push(children.into_iter());
                self.item_stack.push(value);
            }
        }

        fn get(&self) -> Option<&Self::Item> {
            if self.item_stack.len() > 0 {
                Some(&self.item_stack)
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
                Some(&mut self.item_stack)
            }
        }
    };
}

macro_rules! preorder_context_streaming_iterator_impl {
    ($get_value_and_children: ident) => {
        fn advance(&mut self) {
            if let Some(root) = self.root.take() {
                let (value, children) = root.$get_value_and_children();
                self.current_context.ancestors.push(value);
                self.current_context.children = core::mem::MaybeUninit::new(children);
                return;
            }

            let mut children = core::mem::MaybeUninit::uninit();
            core::mem::swap(&mut self.current_context.children, &mut children);
            self.traversal_stack
                .push(unsafe { children.assume_init() }.into_iter());
            self.current_context.path.push(usize::MAX);

            let next = loop {
                if let Some(top) = self.traversal_stack.last_mut() {
                    if let Some(value) = top.next() {
                        let last = self
                            .current_context
                            .path
                            .last_mut()
                            .expect("There to always be a value in the path list");
                        *last = last.wrapping_add(1);
                        break Some(value);
                    }

                    self.traversal_stack.pop();
                    self.current_context.ancestors.pop();
                    self.current_context.path.pop();
                } else {
                    break None;
                }
            };

            if let Some(next) = next {
                let (value, children) = next.$get_value_and_children();
                self.current_context.ancestors.push(value);
                self.current_context.children = core::mem::MaybeUninit::new(children);
            }
        }

        fn get(&self) -> Option<&Self::Item> {
            if self.current_context.ancestors().is_empty() {
                None
            } else {
                Some(&self.current_context)
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

pub(crate) use dfs_preorder_next;
pub(crate) use get_mut_ancestors;
pub(crate) use get_mut_context;
pub(crate) use preorder_ancestors_streaming_iterator_impl;
pub(crate) use preorder_context_streaming_iterator_impl;
