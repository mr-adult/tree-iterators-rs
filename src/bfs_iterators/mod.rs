pub (crate) mod owned;
pub (crate) mod mut_borrow;
pub (crate) mod borrow;

macro_rules! bfs_next {
    ($get_value_and_children: ident) => {
        fn next(&mut self) -> Option<Self::Item> {
            match std::mem::take(&mut self.root) {
                Some(root) => {
                    let (value, children) = root.$get_value_and_children();
                    match children {
                        None => {}
                        Some(children) => {
                            self.traversal_queue.push_back(children);
                        }
                    }
                    return Some(value);
                }
                None => {
                    loop {
                        let next_queue_opt = self.traversal_queue.get_mut(0);
                        match next_queue_opt {
                            None => return None,
                            Some(next_queue) => {
                                match next_queue.next() {
                                    None => {
                                        self.traversal_queue.pop_front();
                                        continue;
                                    }
                                    Some(next) => {
                                        let (value, children) = next.$get_value_and_children();
                                        match children {
                                            None => {}
                                            Some(children) => self.traversal_queue.push_back(children)                                   }
                                        return Some(value);
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

pub(crate) use bfs_next;