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
            if !self.last_iteration_was_just_a_pop {
                self.last_iteration_was_just_a_pop = true;
                self.item_stack.pop();
                return;
            }
    
            self.last_iteration_was_just_a_pop = false;
    
            let mut current = None;
            while current.is_none() {
                while self.has_gone_right_stack.len() > 0 && self.has_gone_right_stack[self.has_gone_right_stack.len() - 1] == true {
                    self.item_stack.pop();
                    self.has_gone_right_stack.pop();
                }
    
                current = match current {
                    Some(c) => Some(c),
                    None => {
                        match self.right_stack.pop() {
                            Some(right) => right,
                            None => None,
                        }
                    }
                };
    
                if self.has_gone_right_stack.len() > 0 {
                    self.has_gone_right_stack.pop();
                    self.has_gone_right_stack.push(true);
                }
    
                if self.right_stack.len() == 0 { break; }
            }
    
            while let Some(current_val) = current {
                let (value, [left, right]) = current_val.$get_value_and_left_right();
    
                self.right_stack.push(right);
                self.item_stack.push(value);
                self.has_gone_right_stack.push(false);
                current = left;
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

pub (crate) use dfs_inorder_next;
pub (crate) use dfs_inorder_streaming_iterator_impl;