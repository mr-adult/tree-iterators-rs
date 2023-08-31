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

macro_rules! bfs_advance_iterator {
    ($get_value_and_children: ident) => {
        fn advance_iterator(&mut self) -> ControlFlow<(), ()> {
            let mut next_iter = self.iterator_queue.get_mut(0);
            loop {
                match next_iter {
                    None => {
                        self.item_stack.clear();
                        return ControlFlow::Break(());
                    }
                    Some(children_opt) => {
                        match children_opt {
                            None => {
                                self.increase_depth_and_reset();
                                return ControlFlow::Continue(());
                            }
                            Some(children_opt) => {
                                if let Some(children) = children_opt {
                                    loop {
                                        match children.next() {
                                            None => {
                                                break;
                                            }
                                            Some(item) => {
                                                let (value, children) = item.$get_value_and_children();
                                                self.item_stack.push(value);
                                                self.iterator_queue.push_back(Some(children));
                                                self.is_in_middle_of_iterator = true;
                                                return ControlFlow::Break(());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
    
                let num_queues = self.traversal_queue_stack.len();
                self.traversal_queue_stack
                    .get_mut(num_queues - 1)
                    .expect("there to be at least 1 queue")
                    .push_back(None);
                self.iterator_queue.pop_front();
                next_iter = self.iterator_queue.get_mut(0);
                self.is_in_middle_of_iterator = false;
            }      
        }
    
        fn increase_depth_and_reset(&mut self) {
            self.current_depth += 1;
            self.iterator_queue.push_back(None);
            let items = std::mem::take(&mut self.item_stack);
            for (index, item) in items.into_iter().enumerate() {
                self.traversal_queue_stack.get_mut(index)
                    .expect("item stack to have a value at depth")
                    .push_back(Some(item));
            }
        }
    };
}

macro_rules! bfs_streaming_iterator_impl {
    () => {
        fn advance(&mut self) {
            loop {
                if self.is_in_middle_of_iterator {
                    let depth = self.item_stack.len() - 1;
                    match self.traversal_queue_stack.get_mut(depth) {
                        None => {
                            let mut new_vecdeque = VecDeque::new();
                            new_vecdeque.push_back(Some(self.item_stack.pop().expect("there to be an item in the item stack")));
                            self.traversal_queue_stack.push(new_vecdeque);
                        }
                        Some(vecdeque) => {
                            vecdeque.push_back(Some(self.item_stack.pop().expect("there to be an item in the item stack")));
                        }
                    }
    
                    match self.advance_iterator() {
                        ControlFlow::Break(_) => break,
                        ControlFlow::Continue(_) => continue,
                    }
                }
                let depth = self.traversal_queue_stack.len();
                match self.traversal_queue_stack
                        .get_mut(depth - 1)
                        .expect("stack to have an item")
                        .pop_front()
                        .expect("front of queue to always have a value") {
                    None => {
                        self.increase_depth_and_reset();
                    }
                    Some(item) => {
                        let depth = self.traversal_queue_stack.len();
                        if depth == self.current_depth {
                            self.traversal_queue_stack
                                .get_mut(depth - 1)
                                .expect("there to be a queue at index depth - 1")
                                .push_back(None);
                        }
            
                        if depth == self.current_depth - 1 {
                            self.item_stack.push(item);
                            self.advance_iterator();          
                            return;
                        }
            
                        if depth < self.current_depth {
                            self.item_stack.push(item);
                        } else if depth == self.current_depth {
                            self.item_stack.push(item);
                            return;
                        }
                    }
                }
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

pub(crate) use bfs_next;
pub(crate) use bfs_advance_iterator;
pub(crate) use bfs_streaming_iterator_impl;