This method will panic if called after an element has already been yielded from
the iterator it is called on! This method attaches the ancestors of each node to
the node during iteration. This operation converts the current iterator into a
streaming iterator. For Breadth First Search iterators, this converts the
queue-based iterator into an iterative deepening iterator. This can have
performance impacts, as iterative deepening visits many of the nodes in the tree
more than once. The order in which elements are yielded remains unchanged, but
each will now be yielded with its ancestor stack attached. That means that for
our example tree, each element will be replaced by the following:

- 0 -> \[0\],
- 1 -> \[0, 1\],
- 2 -> \[0, 2\],
- 3 -> \[0, 1, 3\],
- 4 -> \[0, 1, 4\],
- 5 -> \[0, 2, 5\],
- 6 -> \[0, 2, 6\],
- 7 -> \[0, 2, 6, 7\],
- 8 -> \[0, 2, 6, 7, 8\],
- 9 -> \[0, 2, 6, 7, 8, 9\],
- 10 -> \[0, 2, 6, 7, 8, 9, 10\]

### Example Usage

One use case for this API is to filter down to only the values where all of the
ancestors and the current node are even numbers. This can be accomplished with
the following code.

```text
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
// Example usage:
use streaming_iterator::StreamingIterator;
use tree_iterators_rs::{
    prelude::*,
    examples::create_example_binary_tree
};

let root = create_example_binary_tree();
let mut result = String::new();

// any iterator method can be swapped in here
root.dfs_preorder()
    .attach_ancestors()
    .filter(|slice|
        slice.iter().all(|value| *value % 2 == 0)
    )
    .map(|slice| slice[slice.len() - 1])
    .for_each(|value| {
        result.push(' ');
        result.push_str(&value.to_string())
    });

println!("{}", result);
```

### More Technical Details

Because this operation transforms the iterator into a StreamingIterator, the
slices cannot be saved and used across loop iterations, as the slice points to
internal iterator memory and is altered with the .next() call. Each slice must
be collected into a Vec or other data structure by the caller to save them for
later. This operation will incur a performance penalty and this library does not
assume you want that performance penalty by default. Since this iterator is no
longer a Rust Iterator, for loops will no longer work. See details on how to
work around this in the
[streaming-iterator](https://crates.io/crates/streaming-iterator) crate.
