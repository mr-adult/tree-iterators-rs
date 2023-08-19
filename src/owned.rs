use std::collections::VecDeque;

pub trait TreeNode 
    where Self: Sized {
    
    type Value: Sized;
    type Children: Iterator<Item = Self>;
    fn decompose(self) -> (Self::Value, Option<Self::Children>);

    fn dfs_preorder(self) -> OwnedDFSPreorderIterator<Self> {
        OwnedDFSPreorderIterator::new(self)
    }

    fn dfs_postorder(self) -> OwnedDFSPostorderIterator<Self> {
        OwnedDFSPostorderIterator::new(self)
    }

    fn bfs(self) -> OwnedBFSIterator<Self> {
        OwnedBFSIterator::new(self)
    }
}

pub struct OwnedDFSPreorderIterator<Node>
    where Node: TreeNode {

    root: Option<Node>,
    traversal_stack: Vec<Node::Children>,
}

impl<Node> OwnedDFSPreorderIterator<Node> 
    where Node: TreeNode {
        
    fn new(root: Node) -> OwnedDFSPreorderIterator<Node> {
        OwnedDFSPreorderIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }

    fn pop_empty_iterators_until_move(&mut self) -> Option<Node> {
        loop {
            let stack_len = self.traversal_stack.len();
            if stack_len == 0 { return None; }
            match self.traversal_stack.get_mut(stack_len - 1) {
                None => return None,
                Some(top) => {
                    match top.next() {
                        None => {
                            self.traversal_stack.pop();
                        }
                        Some(value) => {
                            return Some(value);
                        }
                    }
                }
            }
        }
    }
}

impl<Node> Iterator for OwnedDFSPreorderIterator<Node> 
    where Node: TreeNode {
    
    type Item = Node::Value;
    
    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::take(&mut self.root) {
            Some(next) => {
                let (value, children) = next.decompose();
                match children {
                    None => {}
                    Some(children) => self.traversal_stack.push(children)
                }
                return Some(value);
            }
            None => {
                let next = self.pop_empty_iterators_until_move();
                match next {
                    None => return None,
                    Some(node) => {
                        let (value, children) = node.decompose();
                        match  children {
                            None => {}
                            Some(children) => self.traversal_stack.push(children),
                        }
                        return Some(value);
                    }
                }
            }
        }
    }
}

pub struct OwnedDFSPostorderIterator<Node> 
    where Node: TreeNode {

    root: Option<Node>,
    item_stack: Vec<Node::Value>,
    traversal_stack: Vec<Node::Children>
}

impl<Node> OwnedDFSPostorderIterator<Node> 
    where Node: TreeNode {

    fn new(root: Node) -> OwnedDFSPostorderIterator<Node> {
        OwnedDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }
}

impl<Node> Iterator for OwnedDFSPostorderIterator<Node> 
    where Node: TreeNode {

    type Item = Node::Value;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match std::mem::take(&mut self.root) {
                Some(next) => {
                    let (value, children) = next.decompose();
                    match children {
                        None => { return Some(value); }
                        Some(children) => {
                            self.traversal_stack.push(children);
                            self.item_stack.push(value);
                        }
                    }
                }
                None => {
                    loop {
                        let stack_len = self.traversal_stack.len();
                        if stack_len < 1 { return None; }
                        match self.traversal_stack.get_mut(stack_len - 1) {
                            None => return self.item_stack.pop(),
                            Some(next_iter) => {
                                match next_iter.next() {
                                    None => {
                                        self.traversal_stack.pop();
                                        return self.item_stack.pop();
                                    }
                                    Some(node) => {
                                        let (value, children) = node.decompose();
                                        self.item_stack.push(value);
                                        match children {
                                            None => { return self.item_stack.pop(); }
                                            Some(children) => self.traversal_stack.push(children)
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
}

pub struct OwnedBFSIterator<Node> 
    where Node: TreeNode {

    root: Option<Node>,
    traversal_queue: VecDeque<Node::Children>
}

impl<Node> OwnedBFSIterator<Node>
    where Node: TreeNode {

    fn new(root: Node) -> OwnedBFSIterator<Node> {
        OwnedBFSIterator { 
            root: Some(root), 
            traversal_queue: VecDeque::new() 
        }
    }
}

impl<Node> Iterator for OwnedBFSIterator<Node> 
    where Node: TreeNode {

    type Item = Node::Value;
    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::take(&mut self.root) {
            Some(root) => {
                let (value, children) = root.decompose();
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
                                    let (value, children) = next.decompose();
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
}

#[cfg(test)]
mod tests {
    use super::TreeNode;

    #[test]
    fn dfs_has_correct_order() {
        let expected = vec![0,1,3,4,2,5,6,7,8,9,10];
        for test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.dfs_preorder().enumerate() {
                assert_eq!(expected[i], value);
            }
        }
    }

    #[test]
    fn bfs_has_correct_order() {
        let expected = (0..=10).collect::<Vec<usize>>();
        for test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.bfs().enumerate() {
                assert_eq!(expected[i], value);
            }
        }
    }

    #[test]
    fn dfs_postorder_has_correct_order() {
        let expected = vec![3,4,1,5,10,9,8,7,6,2,0];
        for test_tree in create_trees_for_testing() {
            for (i, value) in test_tree.dfs_postorder().enumerate() {
                assert_eq!(expected[i], value);
            }
        }
    }

    #[derive(Clone)]
    struct TreeForTesting {
        value: usize,
        children: Option<Vec<TreeForTesting>>
    }

    impl TreeNode for TreeForTesting {
        type Value = usize;
        type Children = std::vec::IntoIter<TreeForTesting>;
        
        fn decompose(self) -> (Self::Value, Option<Self::Children>) {
            let children_iter = match self.children {
                Some(vec) => Some(vec.into_iter()),
                None => None
            };
            (self.value, children_iter)
        }
    }

    fn create_trees_for_testing() -> Vec<TreeForTesting> {
        vec![
            create_tree_for_testing(None),
            create_tree_for_testing(Some(Vec::new()))
        ]
    }

    fn create_tree_for_testing(empty_children_list: Option<Vec<TreeForTesting>>) -> TreeForTesting {
        TreeForTesting {
            value: 0,
            children: Some(vec![
                TreeForTesting {
                    value: 1,
                    children: Some(vec![
                        TreeForTesting {
                            value: 3,
                            children: empty_children_list.clone()
                        },
                        TreeForTesting {
                            value: 4,
                            children: empty_children_list.clone()
                        }
                    ])
                },
                TreeForTesting {
                    value: 2,
                    children: Some(vec![
                        TreeForTesting {
                            value: 5,
                            children: empty_children_list.clone()
                        },
                        TreeForTesting {
                            value: 6,
                            children: Some(vec![
                                TreeForTesting {
                                    value: 7,
                                    children: Some(vec![
                                        TreeForTesting {
                                            value: 8,
                                            children: Some(vec![
                                                TreeForTesting {
                                                    value: 9,
                                                    children: Some(vec![
                                                        TreeForTesting {
                                                            value: 10,
                                                            children: empty_children_list.clone()
                                                        }
                                                    ])
                                                }
                                            ])
                                        }
                                    ])
                                }
                            ])
                        }
                    ])
                }
            ])
        }
    }
}