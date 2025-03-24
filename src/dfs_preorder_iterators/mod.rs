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

macro_rules! preorder_streaming_iterator_impl {
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

macro_rules! get_mut {
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

pub(crate) use dfs_preorder_next;
pub(crate) use get_mut;
pub(crate) use preorder_streaming_iterator_impl;
