// trait TreeNodeIterMut<'a> 
//     where Self: Sized + 'a {
//     type Children: Iterator<Item = &'a mut Self>;
//     fn get_children_iter_mut(&'a mut self) -> Self::Children;

//     fn dfs_preorder(&'a mut self) -> DFSPreorderIterator<Self> {
//         DFSPreorderIterator::new(self)
//     }
// }

// trait DFSIterator<'a, TNode> 
//     where TNode : TreeNodeIterMut<'a> {
    
//     fn get_traversal_root(&mut self) -> Option<&'a mut TNode>;
//     fn get_traversal_stack(&'a mut self) -> &'a mut Vec<TNode::Children>;
//     fn get_current_nodes(&'a mut self) -> &'a mut Vec<&'a mut TNode>;

//     fn try_to_push_iterator(&'a mut self) -> bool {
//         let root = self.get_traversal_root();
//         let stack = self.get_traversal_stack();
//         if let Some(root) = root {
//             stack.push(root.get_children_iter_mut());
//             return true;
//         }

//         let current_node = self.get_current_nodes().pop();
//         match current_node {
//             None => return false,
//             Some(mut node) => {
//                 let new_tree_level = node.get_children_iter_mut();
//                 self.get_traversal_stack().push(new_tree_level);
//                 return true;
//             }
//         }
//     }    

//     fn pop_empty_iterators_until_move(&mut self) {
//         while self.top_iterator_is_empty() {
//             self.get_traversal_stack().pop();
//         }
//     }

//     fn top_iterator_is_empty(&mut self) -> bool {
//         let traversal_stack = self.get_traversal_stack();
//         let stack_len = traversal_stack.len();
//         match traversal_stack.get_mut(stack_len - 1) {
//             None => return false,
//             Some(top_children_iter) => {
//                 match top_children_iter.next() {
//                     None => return true,
//                     Some(node) => {
//                         self.get_current_nodes().push(node);
//                         return false;
//                     }
//                 }
//             }
//         }
//     }
// }

// pub struct DFSPreorderIterator<'a, TNode> where TNode: TreeNodeIterMut<'a> {
//     traversal_root: Vec<&'a mut TNode>,
//     traversal_stack: Vec<TNode::Children>,
//     current_node_at_each_stack_level: Vec<&'a mut TNode>
// }

// impl<'a, TNode> DFSPreorderIterator<'a, TNode> 
//     where TNode: TreeNodeIterMut<'a> {
//     fn new(root: &'a mut TNode) -> DFSPreorderIterator<'a, TNode> {
//         DFSPreorderIterator {
//             traversal_root: vec![root],
//             traversal_stack: Vec::new(),
//             current_node_at_each_stack_level: Vec::new(),
//         }
//     }
// }

// impl<'a, TNode> Iterator for DFSPreorderIterator<'a, TNode> 
//     where TNode: TreeNodeIterMut<'a> {
    
//     type Item=&'a mut TNode;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.try_to_push_iterator();
//         if let Some(root) = self.traversal_root.pop() {
//             return Some(root);
//         }

//         self.pop_empty_iterators_until_move();

//         if self.traversal_stack.is_empty() {
//             return None;
//         }

//         return Some(self.get_current_nodes().pop().expect("Node to exist"));
//     }
// }

// impl<'a, TNode> DFSIterator<'a, TNode> for DFSPreorderIterator<'a, TNode> where TNode: TreeNodeIterMut<'a> {
//     fn get_traversal_root(&mut self) -> Option<&'a mut TNode> {
//         let traversal_root = self.traversal_root.pop();
//         traversal_root
//     }

//     fn get_traversal_stack<'b>(&'b mut self) -> &'b mut Vec<<TNode as TreeNodeIterMut<'a>>::Children> {
//         &mut self.traversal_stack
//     }

//     fn get_current_nodes(&'a mut self) -> &'a mut Vec<&'a mut TNode> {
//         &mut self.current_node_at_each_stack_level
//     }
// }

// pub struct DFSPostorderIterator {
// }

// trait Peek<T> {
//     fn peek(&self) -> Option<&T>;
// }

// #[cfg(test)]
// mod tests {
//     use super::TreeNodeIterMut;

//     struct TreeForTesting {
//         value: usize,
//         children: Vec<TreeForTesting>
//     }

//     impl<'a> TreeNodeIterMut<'a> for TreeForTesting {
//         type Children = std::slice::IterMut<'a, TreeForTesting>;
//         fn get_children_iter_mut(&'a mut self) -> Self::Children {
//             self.children.iter_mut()
//         }
//     }

//     #[test]
//     fn dfs_works() {
//         let mut tree = create_tree_for_testing();
//         let expected_order = vec![0,1,2,3,4,5,6,7,8,9,10];

//         for (i, node) in tree.dfs_preorder().enumerate() {
//             assert_eq!(expected_order[i], node.value)
//         }
//     }


//     fn create_tree_for_testing() -> TreeForTesting {
//         TreeForTesting {
//             value: 0,
//             children: vec![
//                 TreeForTesting {
//                     value: 1,
//                     children: vec![
//                         TreeForTesting {
//                             value: 3,
//                             children: Vec::new()
//                         },
//                         TreeForTesting {
//                             value: 4,
//                             children: Vec::new()
//                         }
//                     ]
//                 },
//                 TreeForTesting {
//                     value: 2,
//                     children: vec![
//                         TreeForTesting {
//                             value: 5,
//                             children: Vec::new()
//                         },
//                         TreeForTesting {
//                             value: 6,
//                             children: vec![
//                                 TreeForTesting {
//                                     value: 7,
//                                     children: vec![
//                                         TreeForTesting {
//                                             value: 8,
//                                             children: vec![
//                                                 TreeForTesting {
//                                                     value: 9,
//                                                     children: vec![
//                                                         TreeForTesting {
//                                                             value: 10,
//                                                             children: Vec::new()
//                                                         }
//                                                     ]
//                                                 }
//                                             ]
//                                         }
//                                     ]
//                                 }
//                             ]
//                         }
//                     ]
//                 }
//             ]
//         }
//     }
// }