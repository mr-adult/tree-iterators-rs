The children of the current node.

NOTE: If Children is an Iterator type, consuming items
from the iterator will result in them being disappearing from
tree traversal.

If Children is a collection type like [`Vec`] or
[`alloc::collections::LinkedList`] then the children can be
safely modified and read at will. Some caveats to this are:
1. if an item is added during depth first preorder traversal
or breadth first traversal, it is easy to create infinite loops
since that node will also be visited (and another node added).
It is recommended that you only add nodes to the tree during
depth first postorder traversals.

children can also act as a way to search sub-trees when a node
meets a condition.
```rust
use tree_iterators_rs::examples::create_example_tree;
use tree_iterators_rs::prelude::*;

let tree = create_example_tree();
todo!()
```