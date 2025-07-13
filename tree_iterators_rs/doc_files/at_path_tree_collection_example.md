### Example Usage

```rust
use tree_iterators_rs::prelude::{Tree, OwnedIntoIteratorOfTrees};

let trees = vec![
    Tree {
        value: 0,
        children: vec![],
    },
    Tree {
        value: 1,
        children: vec![
            Tree {
                value: 2,
                children: vec![],
            },
            Tree {
                value: 3,
                children: vec![
                    Tree {
                        value: 4,
                        children: vec![],
                    }
                ],
            },
        ]
    }
];

assert_eq!(Some(4), trees.at_path(&[1, 1, 0]).map(|tree| tree.value));
```
