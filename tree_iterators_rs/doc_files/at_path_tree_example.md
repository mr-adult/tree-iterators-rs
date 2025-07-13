### Example Usage

```rust
use tree_iterators_rs::prelude::{Tree, OwnedTreeNode};

let tree = Tree {
    value: 0,
    children: vec![
        Tree {
            value: 1,
            children: vec![
                Tree {
                    value: 2,
                    children: vec![],
                },
                Tree {
                    value: 3,
                    children: vec![],
                }
            ]
        }
    ]
};

assert_eq!(Some(3), tree.at_path(&[0, 1]).map(|tree| tree.value));
```
