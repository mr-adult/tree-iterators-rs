// use std::path::Iter;

// trait TreeNodeIter<'a>
//     where Self: Sized + 'a {
//     type Children: Iterator<Item = &'a Self>;
//     fn get_children_iter(&'a self) -> Self::Children;

//     fn dfs_preorder(&'a self) -> DFSPreorderIterator<Self> {
//         DFSPreorderIterator::new(self)
//     }
// }

// struct DFSPreorderIterator<'a, TNode> where TNode: TreeNodeIter<'a> {
//     traversal_stack: Vec<&'a TNode>,
// }

// impl<'a, TNode> DFSPreorderIterator<'a, TNode> where TNode: TreeNodeIter<'a> {
//     fn new(root: &'a TNode) -> DFSPreorderIterator<'a, TNode> {
//         DFSPreorderIterator {
//             traversal_stack: vec![root]
//         }
//     }
// }

// impl<'a, TNode> Iterator for DFSPreorderIterator<'a, TNode> where TNode: TreeNodeIter<'a> {
//     type Item = &'a TNode;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.traversal_stack.pop() {
//             None => None,
//             Some(el) => {
//                 let mut stack = Vec::new();
                
//                 for child in el.get_children_iter() {
//                     stack.push(child);
//                 }

//                 while let Some(child) = stack.pop() {
//                     self.traversal_stack.push(child);
//                 }
                
//                 Some(el)
//             }
//         }
//     }
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