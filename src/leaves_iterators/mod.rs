pub (crate) mod owned;
pub (crate) mod mut_borrow;
pub (crate) mod borrow;

macro_rules! dfs_preorder_next_with_children_check {
    ($get_value_and_children: ident, $value_type: ty) => {
        fn dfs_preorder_next(&mut self) -> Option<(bool, $value_type)> {
            match std::mem::take(&mut self.root) {
                Some(next) => {
                    let (value, children) = next.$get_value_and_children();
                    let mut has_children = false;
                    match children {
                        None => {}
                        Some(children) => {
                            let mut peekable = MakePeekableIterator::new(children);
                            has_children = peekable.peek().is_some();
                            self.traversal_stack.push(peekable);
                        }
                    }
                    return Some((has_children, value));
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
                                match top.next() {
                                    None => {
                                        self.traversal_stack.pop();
                                    }
                                    Some(value) => {
                                        next = Some(value);
                                        break;
                                    }
                                }
                            }
                        }
                    };
                    match next {
                        None => return None,
                        Some(node) => {
                            let (value, children) = node.$get_value_and_children();
                            let mut has_children = false;
                            match  children {
                                None => {}
                                Some(children) => {
                                    let mut peekable = MakePeekableIterator::new(children);
                                    has_children = peekable.peek().is_some();
                                    self.traversal_stack.push(peekable);
                                }
                            }
                            return Some((has_children, value));
                        }
                    }
                }
            }
        }
    };
}

pub (crate) use dfs_preorder_next_with_children_check;