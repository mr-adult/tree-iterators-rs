#[cfg(test)]
pub mod tests {
    extern crate std;
    use std::println;
    use alloc::string::String;
    use super::super::prelude::*;
    use alloc::collections::VecDeque;
    use alloc::boxed::Box;
    use alloc::vec;
    use super::{create_example_binary_tree, create_example_tree};

    #[test]
    fn bfs_example() {
        // Tree creation (see above documentation)
        let root = create_example_tree();

        let mut result = String::new();
        for value in root.bfs() {
            result.push_str(&value.to_string());
            result.push_str(", ");
        }

        // result: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        println!("{}", result);
    }

    #[test]
    fn bfs_iterator_example() {
        // Tree creation (see above documentation)
        let root = create_example_tree();

        let result = 
            root.bfs()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join(", ");

        // result: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        println!("{}", result);
    }

    #[test]
    fn bfs_equivalent_example() {
        // Tree creation (see above documentation)
        let root = create_example_tree();

        let mut result = String::new();

        let mut queue = VecDeque::new();
        queue.push_back(root);
        while queue.len() > 0 {
            if let Some(front) = queue.pop_front() {
                if let Some(children) = front.children {
                    for child in children {
                        queue.push_back(child);
                    }
                }

                result.push_str(&front.value.to_string());
                result.push_str(", ");
            }
        }

        // result: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        println!("{}", result);
    }

    #[test]
    fn dfs_preorder_example() {
        // Tree creation (see above documentation)
        let root = create_example_tree();

        let mut result = String::new();
        for value in root.dfs_preorder() {
            result.push_str(&value.to_string());
            result.push_str(", ");
        }

        // result: 0, 1, 3, 4, 2, 5, 6, 7, 8, 9, 10,
        println!("{}", result);
    }

    #[test]
    fn dfs_preorder_iterator_example() {
        // Tree creation (see above documentation)
        let root = create_example_tree();

        let result = 
            root.dfs_preorder()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join(", ");

        // result: 0, 1, 3, 4, 2, 5, 6, 7, 8, 9, 10,
        println!("{}", result);
    }

    #[test]
    fn dfs_preorder_equivalent_example() {
        // Tree creation (see above documentation)
        let root = create_example_tree();

        let mut result = String::new();
        
        let mut stack = vec![root];
        while stack.len() > 0 {
            if let Some(top) = stack.pop() {
                if let Some(mut children) = top.children {
                    children.reverse();
                    for child in children {
                        stack.push(child);
                    }
                }

                result.push_str(&top.value.to_string());
                result.push_str(", ");
            }
        }

        // result: 0, 1, 3, 4, 2, 5, 6, 7, 8, 9, 10,
        println!("{}", result);
    }

    #[test]
    fn dfs_inorder_example() {
        // Tree creation (see above documentation)
        let root = create_example_binary_tree();

        let mut result = String::new();
        for value in root.dfs_inorder() {
            result.push_str(&value.to_string());
            result.push_str(", ");
        }

        // result: 3, 1, 4, 0, 5, 2, 7, 9, 10, 8, 6,
        println!("{}", result);
    }

    #[test]
    fn dfs_inorder_iterator_example() {
        // Tree creation (see above documentation)
        let root = create_example_binary_tree();

        let result = root.dfs_preorder()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        // result: 3, 1, 4, 0, 5, 2, 7, 9, 10, 8, 6,
        println!("{}", result);
    }

    #[test]
    fn dfs_postorder_equivalent_example() {        
        fn dfs_postorder(node: TreeNode<usize>, result: &mut String) {
            if let Some(children) = node.children {
                for child in children {
                    dfs_postorder(child, result);
                }
            }

            result.push_str(", ");
            result.push_str(&node.value.to_string());
        }

        // Tree creation (see above documentation)
        let root = create_example_tree();

        let mut result = String::new();
        dfs_postorder(root, &mut result);

        // result: 3, 4, 1, 5, 10, 9, 8, 7, 6, 2, 0,
        println!("{}", result);
    }

    #[test]
    fn dfs_postorder_example() {
        // Tree creation (see above documentation)
        let root = create_example_tree();

        let mut result = String::new();
        for value in root.dfs_postorder() {
            result.push_str(&value.to_string());
            result.push_str(", ");
        }

        // result: 3, 4, 1, 5, 10, 9, 8, 7, 6, 2, 0,
        println!("{}", result);
    }

    #[test]
    fn dfs_postorder_iterator_example() {
        // Tree creation (see above documentation)
        let root = create_example_tree();

        let result = 
            root.dfs_postorder()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join(", ");

        // result: 3, 4, 1, 5, 10, 9, 8, 7, 6, 2, 0,
        println!("{}", result);
    }

    #[test]
    fn dfs_inorder_equivalent_example() {        
        fn dfs_inorder(node: Option<Box<BinaryTreeNode<usize>>>, result: &mut String) {
            match node {
                None => {}
                Some(node) => {
                    dfs_inorder(node.left, result);
                    result.push_str(&node.value.to_string());
                    result.push_str(", ");
                    dfs_inorder(node.right, result)
                }
            }            
        }

        // Tree creation (see above documentation)
        let root = create_example_binary_tree();

        let mut result = String::new();
        dfs_inorder(Some(Box::new(root)), &mut result);

        // result: 3, 1, 4, 0, 5, 2, 7, 9, 10, 8, 6,
        println!("{}", result);
    }

    #[test]
    fn dfs_preorder_leaves_immediate_call_example() {
        let root = create_example_tree();

        let result = root.dfs_preorder()
            .leaves()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        // result: 3, 4, 5, 10,
        println!("{}", result);
    }

    #[test]
    fn dfs_preorder_leaves_delayed_call_example() {
        let root = create_example_tree();

        let mut dfs_preorder = root.dfs_preorder();

        let mut results = Vec::new();
        // take the first 2 non-leaves before switching to a leaves-only iterator
        results.push(dfs_preorder.next().unwrap().to_string());
        results.push(dfs_preorder.next().unwrap().to_string());

        // once leaves is called, iteration switches to a depth-first postorder search
        for leaf in dfs_preorder.leaves() {
            results.push(leaf.to_string());
        }

        let result = results.join(", ");

        // result: 0, 1, 3, 4, 5, 10,
        println!("{}", result);
    }

    use alloc::string::ToString;
    use alloc::vec::Vec;
    use streaming_iterator::StreamingIterator;

    #[test]
    fn dfs_preorder_attach_ancestors_example() {
        let root = create_example_tree();
        let mut result = String::new();

        root.dfs_preorder_iter()
            .attach_ancestors()
            .filter(|slice| 
                slice.iter().all(|value| **value % 2 == 0)
            )
            .map(|slice| slice[slice.len() - 1])
            .for_each(|value| {
                result.push(' ');
                result.push_str(&value.to_string())
            });
        
        // result: 0 2 6
        println!("{}", result);
    }

    #[test]
    fn dfs_postorder_attach_ancestors_example() {
        let root = create_example_tree();
        let mut result = String::new();

        root.dfs_postorder_iter()
            .attach_ancestors()
            .filter(|slice| 
                slice.iter().all(|value| **value % 2 == 0)
            )
            .map(|slice| slice[slice.len() - 1])
            .for_each(|value| {
                result.push(' ');
                result.push_str(&value.to_string())
            });
        
        // result: 6 2 0
        println!("{}", result);
    }

    #[test]
    fn dfs_inorder_attach_ancestors_example() {
        let root = create_example_binary_tree();
        let mut result = String::new();

        root.dfs_inorder_iter()
            .attach_ancestors()
            .filter(|slice| 
                slice.iter().all(|value| **value % 2 == 0)
            )
            .map(|slice| slice[slice.len() - 1])
            .for_each(|value| {
                result.push(' ');
                result.push_str(&value.to_string())
            });
        
        // result: 6 2 0
        println!("{}", result);
    }

    #[test]
    fn bfs_attach_ancestors_example() {
        let root = create_example_tree();
        let mut result = String::new();

        root.bfs_iter()
            .attach_ancestors()
            .filter(|slice| 
                slice.iter().all(|value| **value % 2 == 0)
            )
            .map(|slice| slice[slice.len() - 1])
            .for_each(|value| {
                result.push(' ');
                result.push_str(&value.to_string())
            });
        
        // result: 0 2 6
        println!("{}", result);
    }

    mod custom_implemenation {
        use crate::prelude::*;
        use alloc::collections::LinkedList;

        struct LLTreeNode<T> {
            value: T,
            children: LinkedList<LLTreeNode<T>>
        }

        use alloc::collections::linked_list::IntoIter;

        impl<T> OwnedTreeNode for LLTreeNode<T> {
            type OwnedValue = T;
            type OwnedChildren = IntoIter<LLTreeNode<T>>;

            fn get_value_and_children(self) -> (Self::OwnedValue, Option<Self::OwnedChildren>) {
                (
                    self.value,
                    Some(self.children.into_iter())
                )
            }
        }

        use alloc::collections::linked_list::IterMut;

        impl<'a, T> MutBorrowedTreeNode<'a> for LLTreeNode<T> 
            where Self: 'a {

            type MutBorrowedValue = &'a mut T;
            type MutBorrowedChildren = IterMut<'a, LLTreeNode<T>>;

            fn get_value_and_children_iter_mut(&'a mut self) -> (Self::MutBorrowedValue, Option<Self::MutBorrowedChildren>) {
                (
                    &mut self.value,
                    Some(self.children.iter_mut())
                )
            }
        }

        use alloc::collections::linked_list::Iter;

        impl<'a, T> BorrowedTreeNode<'a> for LLTreeNode<T> 
            where Self: 'a {

            type BorrowedValue = &'a T;
            type BorrowedChildren = Iter<'a, LLTreeNode<T>>;

            fn get_value_and_children_iter(&'a self) -> (Self::BorrowedValue, Option<Self::BorrowedChildren>) {
                (
                    &self.value,
                    Some(self.children.iter())
                )
            }
        }
    }


}

use crate::prelude::*;
use alloc::{boxed::Box, vec};

pub fn create_example_binary_tree() -> BinaryTreeNode<usize> {
    BinaryTreeNode { 
        value: 0, 
        left: Some(
            Box::new(
                BinaryTreeNode {
                    value: 1,
                    left: Some(
                        Box::new(
                            BinaryTreeNode {
                                value: 3,
                                left: None,
                                right: None,
                            }
                        )
                    ),
                    right: Some(
                        Box::new(
                            BinaryTreeNode {
                                value: 4,
                                left: None,
                                right: None,
                            }
                        )
                    ),
                }
            )
        ), 
        right: Some(
            Box::new(
                BinaryTreeNode {
                    value: 2,
                    left: Some(
                        Box::new(
                            BinaryTreeNode {
                                value: 5,
                                left: None,
                                right: None,
                            }
                        )
                    ),
                    right: Some(
                        Box::new(
                            BinaryTreeNode {
                                value: 6,
                                left: Some(
                                    Box::new(
                                        BinaryTreeNode { 
                                            value: 7, 
                                            left: None, 
                                            right: Some(
                                                Box::new(
                                                    BinaryTreeNode { 
                                                        value: 8, 
                                                        left: Some(
                                                            Box::new(
                                                                BinaryTreeNode {
                                                                    value: 9,
                                                                    left: None,
                                                                    right: Some(
                                                                        Box::new(
                                                                            BinaryTreeNode { 
                                                                                value: 10, 
                                                                                left: None, 
                                                                                right: None 
                                                                            }
                                                                        )
                                                                    )
                                                                }
                                                            )
                                                        ), 
                                                        right: None 
                                                    }
                                                )
                                            ) 
                                        }
                                    )
                                ),
                                right: None,
                            }
                        )
                    )
                }
            )
        ) 
    }
}

pub fn create_example_tree() -> TreeNode<usize> {
    TreeNode {
        value: 0,
        children: Some(vec![
            TreeNode {
                value: 1,
                children: Some(vec![
                    TreeNode {
                        value: 3,
                        children: None
                    },
                    TreeNode {
                        value: 4,
                        children: None
                    }
                ])
            },
            TreeNode {
                value: 2,
                children: Some(vec![
                    TreeNode {
                        value: 5,
                        children: None
                    },
                    TreeNode {
                        value: 6,
                        children: Some(vec![
                            TreeNode {
                                value: 7,
                                children: Some(vec![
                                    TreeNode {
                                        value: 8,
                                        children: Some(vec![
                                            TreeNode {
                                                value: 9,
                                                children: Some(vec![
                                                    TreeNode {
                                                        value: 10,
                                                        children: None
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