pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! dfs_inorder_next {
    ($get_value_and_children_binary: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            self.moved = true;
            let mut current = None;
            while current.is_none() {
                if self.right_stack.len() == self.item_stack.len() {
                    return self.item_stack.pop();
                }

                current = self.right_stack.pop().unwrap_or_default();

                if self.right_stack.is_empty() {
                    break;
                }
            }

            while let Some(current_val) = current {
                let (value, [left, right]) = current_val.$get_value_and_children_binary();

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
                if self.right_stack.is_empty() {
                    self.item_stack.clear();
                    break;
                }

                while let Some(TraversalStatus::WentRight) = self.status_stack.last() {
                    self.item_stack.pop();
                    self.status_stack.pop();
                }

                if let Some(last_status) = self.status_stack.last_mut() {
                    if !matches!(last_status, TraversalStatus::ReturnedSelf) {
                        *last_status = TraversalStatus::ReturnedSelf;
                        return;
                    }
                }

                if current.is_some() {
                    continue;
                }

                if let Some(last_status) = self.status_stack.last_mut() {
                    *last_status = TraversalStatus::WentRight;
                }

                if let Some(top_of_right_stack) = self.right_stack.pop() {
                    current = top_of_right_stack;
                    continue;
                }

                while let Some(TraversalStatus::WentRight) = self.status_stack.last() {
                    self.item_stack.pop();
                    self.status_stack.pop();
                }
                return;
            }

            while let Some(current_val) = current {
                let (value, [left, right]) = current_val.$get_value_and_left_right();

                self.right_stack.push(right);
                self.item_stack.push(value);
                self.status_stack.push(TraversalStatus::WentLeft);
                current = left;
            }

            if let Some(last_status) = self.status_stack.last_mut() {
                *last_status = TraversalStatus::ReturnedSelf;
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

/// Statuses during an in order tree traversal. This enum
/// should be treated as a state machine that can only flow
/// in one direction
/// WentLeft -> ReturnedSelf -> WentRight.
pub(crate) enum TraversalStatus {
    WentLeft,
    ReturnedSelf,
    WentRight,
}

pub(crate) use dfs_inorder_next;
pub(crate) use dfs_inorder_streaming_iterator_impl;
pub(crate) use get_mut;
