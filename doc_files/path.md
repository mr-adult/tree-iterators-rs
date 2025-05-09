The path (in indexes) down to the current tree node.
If the TreeNode's `Children` collection
(see
- [`BorrowedChildren`](crate::prelude::BorrowedTreeNode::BorrowedChildren),
- [`MutBorrowedChildren`](crate::prelude::MutBorrowedTreeNode::MutBorrowedChildren),
- or [`OwnedChildren`](crate::prelude::OwnedTreeNode::OwnedChildren))

is a [`Vec`] or some other indexable collection type, this
path can be followed to get back to the current node.

Ex. given the following tree, some example paths would be:
- path to 'A' = \[\]
- path to 'D' = \[2\]
- path to 'H' = \[1, 0\]
- path to 'K' = \[2, 0\]
- path to 'Q' = \[2, 2, 0, 0, 1\]
```text
          A
       /  |  \
     B    C    D
  / | \ / | \ / | \
  E F G H I J K L M
                  |
                  N
                  |
                  O
                 / \
                P   Q
```