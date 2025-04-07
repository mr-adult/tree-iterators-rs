pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! dfs_preorder_next {
    ($get_value_and_children: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(root) = self.root.take() {
                let (value, children) = root.$get_value_and_children();
                self.traversal_stack.push(children.into_iter());
                return Some(value);
            }

            let next = loop {
                if let Some(top) = self.traversal_stack.last_mut() {
                    if let Some(value) = top.next() {
                        break Some(value);
                    }

                    self.traversal_stack.pop();
                } else {
                    break None;
                }
            };

            next.map(|node| {
                let (value, children) = node.$get_value_and_children();
                self.traversal_stack.push(children.into_iter());
                value
            })
        }
    };
}

macro_rules! dfs_preorder_next_with_path_tracking {
    ($get_value_and_children: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(root) = self.root.take() {
                let (value, children) = root.$get_value_and_children();
                self.on_deck_into_iterator = Some(children);
                return Some(value);
            }

            if let Some(children) = self.on_deck_into_iterator.take() {
                self.traversal_stack.push(children.into_iter());
                self.path.push(usize::MAX);
            }

            let next = loop {
                if let Some(top) = self.traversal_stack.last_mut() {
                    if let Some(value) = top.next() {
                        let last = self.path.last_mut().expect("path to have a value");
                        *last = last.wrapping_add(1);
                        break Some(value);
                    }

                    self.traversal_stack.pop();
                    self.path.pop();
                } else {
                    break None;
                }
            };

            next.map(|node| {
                let (value, children) = node.$get_value_and_children();
                self.on_deck_into_iterator = Some(children);
                value
            })
        }
    };
}

macro_rules! dfs_preorder_binary_next_with_path_tracking {
    ($get_value_and_children: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(root) = self.root.take() {
                let (value, children) = root.$get_value_and_children();
                self.on_deck_into_iterator = Some(children);
                return Some(value);
            }

            if let Some(children) = self.on_deck_into_iterator.take() {
                self.traversal_stack.push(children.into_iter());
                self.path.push(usize::MAX);
            }

            let next = 'outer: loop {
                if let Some(top) = self.traversal_stack.last_mut() {
                    while let Some(value) = top.next() {
                        let last = self.path.last_mut().expect("path to have a value");
                        *last = last.wrapping_add(1);

                        if let Some(value) = value {
                            break 'outer Some(value);
                        }
                    }

                    self.traversal_stack.pop();
                    self.path.pop();
                } else {
                    break None;
                }
            };

            next.map(|node| {
                let (value, children) = node.$get_value_and_children();
                self.on_deck_into_iterator = Some(children);
                value
            })
        }
    };
}

macro_rules! preorder_ancestors_streaming_iterator_impl {
    ($get_value_and_children: ident) => {
        fn advance(&mut self) {
            if let Some(root) = self.root.take() {
                let (value, children) = root.$get_value_and_children();
                self.traversal_stack.push(children.into_iter());
                self.item_stack.push(value);
                return;
            }

            let next = loop {
                if let Some(top) = self.traversal_stack.last_mut() {
                    if let Some(value) = top.next() {
                        break Some(value);
                    }

                    self.traversal_stack.pop();
                    self.item_stack.pop();
                } else {
                    break None;
                }
            };

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
                self.current_context.children = Some(children);
                return;
            }

            if self.current_context.ancestors.is_empty() {
                return;
            }

            if let Some(children) = self.current_context.children.take() {
                self.traversal_stack.push(children.into_iter());
                self.current_context.path.push(usize::MAX);
            } else {
                self.current_context.ancestors.pop();
                if let Some(&usize::MAX) = self.current_context.path.last() {
                    self.current_context.path.pop();
                }
            }

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
                self.current_context.children = Some(children);
            } else {
                self.current_context.ancestors.clear();
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

macro_rules! preorder_binary_context_streaming_iterator_impl {
    ($get_value_and_children: ident) => {
        fn advance(&mut self) {
            if let Some(root) = self.root.take() {
                let (value, children) = root.$get_value_and_children();
                self.current_context.ancestors.push(value);
                self.current_context.children = Some(children);
                return;
            }

            if self.current_context.ancestors.is_empty() {
                return;
            }

            if let Some(children) = self.current_context.children.take() {
                self.traversal_stack.push(children.into_iter());
                self.current_context.path.push(usize::MAX);
            } else {
                self.current_context.ancestors.pop();
                if let Some(&usize::MAX) = self.current_context.path.last() {
                    self.current_context.path.pop();
                }
            }

            let next = 'outer: loop {
                if let Some(top) = self.traversal_stack.last_mut() {
                    while let Some(value) = top.next() {
                        let last = self
                            .current_context
                            .path
                            .last_mut()
                            .expect("There to always be a value in the path list");
                        *last = last.wrapping_add(1);

                        if let Some(value) = value {
                            break 'outer Some(value);
                        }
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
                self.current_context.children = Some(children);
            } else {
                self.current_context.ancestors.clear();
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

pub(crate) use dfs_preorder_binary_next_with_path_tracking;
pub(crate) use dfs_preorder_next;
pub(crate) use dfs_preorder_next_with_path_tracking;
pub(crate) use get_mut_ancestors;
pub(crate) use get_mut_context;
pub(crate) use preorder_ancestors_streaming_iterator_impl;
pub(crate) use preorder_binary_context_streaming_iterator_impl;
pub(crate) use preorder_context_streaming_iterator_impl;
