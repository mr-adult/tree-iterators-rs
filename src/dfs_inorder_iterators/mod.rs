pub (crate) mod owned;
pub (crate) mod mut_borrow;
pub (crate) mod borrow;

macro_rules! dfs_inorder_next {
    ($get_value_and_left_right: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            self.moved = true;
            let mut current = None;
            while current.is_none() {
                current = match current {
                    Some(c) => Some(c),
                    None => {
                        if self.right_stack.len() == self.item_stack.len() { 
                            return self.item_stack.pop(); 
                        }
                        match self.right_stack.pop() {
                            Some(right) => right,
                            None => None,
                        }
                    }
                };
    
                if self.right_stack.len() == 0 { break; }
            }
    
            while let Some(current_val) = current {
                let (value, [left, right]) = current_val.$get_value_and_left_right();
    
                self.right_stack.push(right);
                self.item_stack.push(value);
                current = left;
            }
    
            self.item_stack.pop()
        }
    };
}

macro_rules! dfs_inorder_streaming_iterator_impl {
    ($get_value_and_left_right: ident) => {
        fn advance(&mut self) {
            let mut current = None;
            while current.is_none() {  
                if self.right_stack.len() == 0 { 
                    self.item_stack.clear();
                    break;
                }

                while self.status_stack.len() > 0 && self.status_stack[self.status_stack.len() - 1] == TraversalStatus::WentRight {
                    self.item_stack.pop();
                    self.status_stack.pop();
                }
    
                if self.status_stack.len() > 0 {
                    let len = self.status_stack.len();
                    match self.status_stack.get_mut(len - 1) {
                        Some(status) => {
                            if *status != TraversalStatus::ReturnedSelf {
                                *status = TraversalStatus::ReturnedSelf;
                                return;
                            }
                        }
                        None => {}
                    }
                }

                current = match current {
                    Some(c) => Some(c),
                    None => {
                        let len = self.item_stack.len();
                        if self.status_stack.len() > 0 {
                            self.status_stack[len - 1] = TraversalStatus::WentRight;
                        }
                        match self.right_stack.pop() {
                            Some(right) => right,
                            None => {
                                while self.status_stack.len() > 0 && self.status_stack[self.status_stack.len() - 1] == TraversalStatus::WentRight {
                                    self.item_stack.pop();
                                    self.status_stack.pop();
                                }
                                return;
                            }
                        }
                    }
                };
            }
    
            while let Some(current_val) = current {
                let (value, [left, right]) = current_val.$get_value_and_left_right();
    
                self.right_stack.push(right);
                self.item_stack.push(value);
                self.status_stack.push(TraversalStatus::WentLeft);
                current = left;
            }

            if self.status_stack.len() > 0 {
                let len = self.status_stack.len();
                self.status_stack[len - 1] = TraversalStatus::ReturnedSelf;
            }
        }
    
        fn get(&self) -> Option<&Self::Item> {
            if self.item_stack.len() > 0 {
                Some(self.item_stack.as_slice())
            } else {
                None
            }
        }
    };
}

/// Statuses during an in order tree traversal. This enum
/// should be treated as a state machine that can only flow
/// in one direction
/// WentLeft -> ReturnedSelf -> WentRight.
#[derive(PartialEq, Eq)]
pub (crate) enum TraversalStatus {
    WentLeft,
    ReturnedSelf,
    WentRight,
}

pub (crate) use dfs_inorder_next;
pub (crate) use dfs_inorder_streaming_iterator_impl;