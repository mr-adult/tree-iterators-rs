pub mod borrow;
pub mod mut_borrow;
pub mod owned;

macro_rules! bfs_next {
    ($get_value_and_children: ident, $value_type: ty) => {
        fn bfs_next(&mut self) -> Option<(bool, $value_type)> {
            match core::mem::take(&mut self.root) {
                Some(root) => {
                    let (value, children) = root.$get_value_and_children();

                    let mut peekable = children.peekable();
                    let has_children = peekable.peek().is_some();
                    self.new_traversal_queue.push_back(peekable);
                    return Some((has_children, value));
                }
                None => loop {
                    let old_next_queue_opt = self.old_traversal_queue.get_mut(0);
                    match old_next_queue_opt {
                        None => {
                            let new_next_queue_opt = self.new_traversal_queue.get_mut(0);
                            match new_next_queue_opt {
                                None => return None,
                                Some(next_queue) => match next_queue.next() {
                                    None => {
                                        self.new_traversal_queue.pop_front();
                                        continue;
                                    }
                                    Some(next) => {
                                        let (value, children) = next.$get_value_and_children();

                                        let mut peekable = children.peekable();
                                        let has_children = peekable.peek().is_some();
                                        self.new_traversal_queue.push_back(peekable);

                                        return Some((has_children, value));
                                    }
                                },
                            }
                        }
                        Some(next_queue) => match next_queue.next() {
                            None => {
                                self.old_traversal_queue.pop_front();
                                continue;
                            }
                            Some(next) => {
                                let (value, children) = next.$get_value_and_children();

                                let mut peekable = children.peekable();
                                let has_children = peekable.peek().is_some();
                                self.new_traversal_queue.push_back(peekable);

                                return Some((has_children, value));
                            }
                        },
                    }
                },
            }
        }
    };
}

macro_rules! next {
    () => {
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let value = self.bfs_next();
                match value {
                    None => return None,
                    Some(value) => {
                        if value.0 {
                            continue;
                        }
                        return Some(value.1);
                    }
                }
            }
        }
    };
}

pub(crate) use bfs_next;
pub(crate) use next;
