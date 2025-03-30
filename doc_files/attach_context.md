This method will panic if called after an element has already been yielded from
the iterator it is called on! This method attaches the entire context of where a
node is in the tree to the node during iteration. This operation converts the
current iterator into a streaming iterator. For Breadth First Search iterators,
this converts the queue-based iterator into an iterative deepening iterator.
This can have performance impacts, as iterative deepening visits many of the
nodes in the tree more than once. The order in which elements are yielded
remains unchanged. The context provided includes:

1. the entire stack of ancestor values back up to the root node
2. the current node's children collection (except in the case of owned
   dfs_inorder and dfs_postorder APIs, which can't hold onto this information
   without causing performance issues by clone()'ing).
3. the path (list of indexes) to get to the current node.

### Example Usage

One use case for the `attach_context()` API is to loop through each node in the
tree and check if its subtree meets a condition. In this case, we are scanning
for all subtrees that contain the '10' node.

```ignore
Example Tree:
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
use tree_iterators_rs::examples::create_example_tree;
use tree_iterators_rs::prelude::*;
use streaming_iterator::StreamingIterator;

let root = create_example_tree();

let mut iter = root
    .dfs_preorder()
    .attach_context();

while let Some(node_context) = iter.next() {
    let current_node_is_10 = *node_context
        .ancestors()
        .last()
        .expect("ancestors() is guaranteed to be non-empty")
        == 10;

    let any_descendent_is_10 = node_context
        .children()
        .iter()
        .flat_map(|child| child.dfs_preorder_iter())
        .any(|descendent| *descendent == 10);

    let subtree_contains_10 = current_node_is_10 || any_descendent_is_10;

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
// [0, 2, 6, 7, 8, 9, 10] true
```

### More Technical Details

Because this operation transforms the iterator into a StreamingIterator, the
context cannot be saved and used across loop iterations, as the context contains
internal iterator state and is altered with the .next() call. Each slice must be
collected into a Vec or other data structure by the caller to save them for
later. This operation will incur a performance penalty and this library does not
assume you want that performance penalty by default. Since this iterator is no
longer a Rust Iterator, for loops will no longer work. See details on how to
work around this in the
[streaming-iterator](https://crates.io/crates/streaming-iterator) crate.
