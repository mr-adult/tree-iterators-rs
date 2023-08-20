#[cfg(test)]
mod tests {
    use super::super::prelude::{*, tests::create_tree_for_testing};
    use std::collections::VecDeque;

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

    fn create_example_tree() -> TreeNode<usize> {
        create_tree_for_testing(None)
    }
}

mod custom_implemenation {
    use crate::prelude::*;
    use std::collections::LinkedList;

    struct LLTreeNode<T> {
        value: T,
        children: LinkedList<LLTreeNode<T>>
    }

    use std::collections::linked_list::IntoIter;

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

    use std::collections::linked_list::IterMut;

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

    use std::collections::linked_list::Iter;

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