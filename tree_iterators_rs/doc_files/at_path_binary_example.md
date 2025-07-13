### Example Usage

```rust
use tree_iterators_rs::prelude::{BinaryTree, OwnedBinaryTreeNode};

let tree = BinaryTree {
    value: 0,
    left: Some(Box::new(BinaryTree {
        value: 1,
        left: Some(Box::new(BinaryTree {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(BinaryTree {
            value: 3,
            left: None,
            right: None,
        }))
    })),
    right: None,
};

assert_eq!(Some(3), tree.at_path(&[0, 1]).map(|tree| tree.value));
```
