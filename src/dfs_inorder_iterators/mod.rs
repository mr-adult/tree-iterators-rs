pub (crate) mod owned;
pub (crate) mod mut_borrow;
pub (crate) mod borrow;

macro_rules! dfs_inorder_next {
    ($get_value_and_left_right: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
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

pub (crate) use dfs_inorder_next;