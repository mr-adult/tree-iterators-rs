pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! dfs_preorder_next {
    ($get_value_and_children: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            match core::mem::take(&mut self.root) {
                Some(next) => {
                    let (value, children) = next.$get_value_and_children();
                    match children {
                        None => {}
                        Some(children) => self.traversal_stack.push(children),
                    }
                    return Some(value);
                }
                None => {
                    let next;
                    loop {
                        let stack_len = self.traversal_stack.len();
                        if stack_len == 0 {
                            next = None;
                            break;
                        }
                        match self.traversal_stack.get_mut(stack_len - 1) {
                            None => {
                                next = None;
                                break;
                            }
                            Some(top) => match top.next() {
                                None => {
                                    self.traversal_stack.pop();
                                }
                                Some(value) => {
                                    next = Some(value);
                                    break;
                                }
                            },
                        }
                    }
                    match next {
                        None => return None,
                        Some(node) => {
                            let (value, children) = node.$get_value_and_children();
                            match children {
                                None => {}
                                Some(children) => self.traversal_stack.push(children),
                            }
                            return Some(value);
                        }
                    }
                }
            }
        }
    };
}

macro_rules! advance_dfs {
    ($get_value_and_children: ident) => {
        fn advance_dfs(&mut self) {
            match core::mem::take(&mut self.root) {
                Some(next) => {
                    let (value, children) = next.$get_value_and_children();
                    match children {
                        None => {}
                        Some(children) => self.traversal_stack.push(children),
                    }

                    self.item_stack.push(value);
                    return;
                }
                None => {
                    let next;
                    loop {
                        let stack_len = self.traversal_stack.len();
                        if stack_len == 0 {
                            next = None;
                            break;
                        }
                        match self.traversal_stack.get_mut(stack_len - 1) {
                            None => {
                                next = None;
                                break;
                            }
                            Some(top) => {
                                if self.item_stack.len() > stack_len {
                                    self.item_stack.pop();
                                }
                                match top.next() {
                                    None => {
                                        self.traversal_stack.pop();
                                        self.item_stack.pop();
                                    }
                                    Some(value) => {
                                        next = Some(value);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    match next {
                        None => return,
                        Some(node) => {
                            let (value, children) = node.$get_value_and_children();
                            match children {
                                None => {}
                                Some(children) => self.traversal_stack.push(children),
                            }
                            self.item_stack.push(value);
                            return;
                        }
                    }
                }
            }
        }
    };
}

macro_rules! preorder_streaming_iterator_impl {
    () => {
        fn advance(&mut self) {
            self.advance_dfs()
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
                Some(&mut self.item_stack[..])
            }
        }
    };
}

pub(crate) use advance_dfs;
pub(crate) use dfs_preorder_next;
pub(crate) use get_mut;
pub(crate) use preorder_streaming_iterator_impl;
