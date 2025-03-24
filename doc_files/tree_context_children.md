The children of the current node. This will always be empty for depth first
postorder searches and will always be populated for depth first preorder and
breadth first searches.

NOTE: If Children is an Iterator type, consuming items from the iterator will
result in them disappearing from tree traversal.

If Children is a collection type like [`Vec`] or
[`alloc::collections::LinkedList`] then the children can be both safely modified
and read at will. Some caveats to this are:

1. if an item is added during traversal, it is easy to accidentally create
   infinite loops since the added node will also be visited. Tread cautiously.
2. removing an item from the collection may cause it to never be visited by the
   tree iterators.

children can also act as a way to search sub-trees when a node meets a
condition.

NOTE: Be sure to add a use statement for streaming_iterator::StreamingIterator
to pull in the filter, map, reduce, for_each, etc. methods from the
streaming_iterator crate.

```rust
use tree_iterators_rs::examples::create_example_tree;
use tree_iterators_rs::prelude::*;
use streaming_iterator::StreamingIterator;

let root = create_example_tree();

let mut subtree_contains_10;
let mut iter = root.dfs_preorder().attach_context();
while let Some(node_context) = iter.next() {
    subtree_contains_10 = node_context
        .children()
        .unwrap()
        .iter()
        .flat_map(|child| child.dfs_preorder_iter())
        .any(|descendent| *descendent == 10);

    println!("{:?} {}", node_context.ancestors(), subtree_contains_10);
}

// Results:
// [0] true
// [0, 1] false
// [0, 1, 3] false
// [0, 1, 4] false
// [0, 2] true
// [0, 2, 5] false
// [0, 2, 6] true
// [0, 2, 6, 7] true
// [0, 2, 6, 7, 8] true
// [0, 2, 6, 7, 8, 9] true
// [0, 2, 6, 7, 8, 9, 10] false
```
