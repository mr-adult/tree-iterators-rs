pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! dfs_postorder_leaves_next {
    ($get_value_and_children: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            // keep track of if the children iterator was just added.
            // In some cases this becomes important.
            let mut just_added = if let Some(root) = self.root.take() {
                let (value, children) = root.$get_value_and_children();
                self.traversal_stack_top.push(children.into_iter());
                self.item_stack.push(value);
                true
            } else {
                false
            };

            loop {
                if let Some(next_iter) = self.traversal_stack_top.last_mut() {
                    if let Some(node) = next_iter.next() {
                        let (value, children) = node.$get_value_and_children();
                        self.item_stack.push(value);
                        self.traversal_stack_top.push(children.into_iter());
                        just_added = true;
                    } else {
                        self.traversal_stack_top.pop();
                        let popped = self.item_stack.pop();
                        if just_added {
                            return popped;
                        }
                    }
                } else if let Some(next_iter) = self.traversal_stack_bottom.last_mut() {
                    if let Some(node) = next_iter.next() {
                        let (value, children) = node.$get_value_and_children();
                        self.item_stack.push(value);
                        self.traversal_stack_top.push(children.into_iter())
                    } else {
                        self.traversal_stack_bottom.pop();
                    }
                } else {
                    return None;
                }
            }
        }
    };
}

pub(crate) use dfs_postorder_leaves_next;
