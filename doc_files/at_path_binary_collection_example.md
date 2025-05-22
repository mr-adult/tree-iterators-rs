### Example Usage

```rust
use tree_iterators_rs::prelude::{BinaryTree, OwnedIntoIteratorOfBinaryTrees};

let trees = [
    BinaryTree {
        value: 0,
        left: None,
        right: None,
    },
    BinaryTree {
        value: 1,
        left: None,
        right: Some(Box::new(BinaryTree {
            value: 2,
            left: Some(Box::new(BinaryTree {
                value: 3,
                left: None,
                right: None,
            })),
            right: None,
        })),
    },
];

assert_eq!(Some(3), trees.at_path(&[1, 1, 0]).map(|tree| tree.value));
```
