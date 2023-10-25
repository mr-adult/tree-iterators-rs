pub mod owned;
pub mod mut_borrow;
pub mod borrow;

macro_rules! dfs_postorder_leaves_next {
    ($get_value_and_children: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match core::mem::take(&mut self.root) {
                    Some(next) => {
                        let (value, children) = next.$get_value_and_children();
                        match children {
                            None => { return Some(value); }
                            Some(children) => {
                                self.traversal_stack_top.push(children);
                                self.item_stack.push(value);
                            }
                        }
                    }
                    None => {
                        // keep track of if the children iterator was just added. 
                        // In some cases this becomes important.
                        let mut just_added = false;
                        loop {
                            let total_stack_len = self.traversal_stack_bottom.len() + self.traversal_stack_top.len();
                            if total_stack_len < 1 { return None; }
                            let top_stack_len = self.traversal_stack_top.len();
                            if top_stack_len > 0 {
                                match self.traversal_stack_top.get_mut(top_stack_len - 1) {
                                    None => { self.item_stack.pop(); }
                                    Some(next_iter) => {
                                        match next_iter.next() {
                                            None => {
                                                self.traversal_stack_top.pop();
                                                let popped = self.item_stack.pop();
                                                if just_added { return popped; }
                                            }
                                            Some(node) => {
                                                let (value, children) = node.$get_value_and_children();
                                                self.item_stack.push(value);
                                                match children {
                                                    None => { return self.item_stack.pop(); }
                                                    Some(children) => {
                                                        self.traversal_stack_top.push(children);
                                                        just_added = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                continue;
                            }
                            let bottom_stack_len = self.traversal_stack_bottom.len();
                            match self.traversal_stack_bottom.get_mut(bottom_stack_len - 1) {
                                None => {}
                                Some(next_iter) => {
                                    match next_iter.next() {
                                        None => {
                                            self.traversal_stack_bottom.pop();
                                        }
                                        Some(node) => {
                                            let (value, children) = node.$get_value_and_children();
                                            self.item_stack.push(value);
                                            match children {
                                                None => { return self.item_stack.pop(); }
                                                Some(children) => self.traversal_stack_top.push(children)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}

pub (crate) use dfs_postorder_leaves_next;