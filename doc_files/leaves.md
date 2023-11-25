This method converts the current iterator into an iterator that will yield only the leaves of the tree. Iteration still proceeds in either a breadth first search (if called on a breadth first iterator) or depth first post-order search (if called on a depth first pre-, in-, or post-order iterator). This method is safe to call at any point during iteration and will never panic. A leaf is defined as: Any tree node that has no children. Given a tree of the following shape, this iterator would always yield values in the following order (regardless of iteration type, but this is not always the case): 3, 4, 5, 10

```ignore
       0
      / \
     1   2
    / \ / \
   3  4 5  6
          /
         7
          \
           8
          /
         9
          \
          10
```

```rust
// Example usage:
use tree_iterators_rs::{
    prelude::*,
    examples::create_example_binary_tree
};

let root = create_example_binary_tree();
for leaf in root.bfs_iter().leaves() {
    println!("{}", leaf);
}
```