The path (in indexes) down to the current tree node.
If both the top level IntoIteratorOfTrees and the 
TreeNode's `Children` collection
(see
- [`BorrowedChildren`](crate::prelude::BorrowedTreeNode::BorrowedChildren),
- [`MutBorrowedChildren`](crate::prelude::MutBorrowedTreeNode::MutBorrowedChildren),
- or [`OwnedChildren`](crate::prelude::OwnedTreeNode::OwnedChildren))

is a [`Vec`] or some other indexable collection type, this
path can be followed to get back to the current node.

Ex. given the following trees, some example paths would be:
- path to 'A' = \[0\]
- path to 'K' = \[0, 2, 0\]
- path to 'Q' = \[0, 2, 2, 0, 0, 1\]
- path to 'V' = \[1, 0, 1\]
- path to 'X' = \[1, 1, 0\]
- path to 'Z' = \[1, 0, 2, 0, 0\]
```text
          A              R   <-- (The IntoIteratorOfTrees)
       /  |  \          / \
     B    C    D       S   T
  / | \ / | \ / | \  / | \ |
  E F G H I J K L M  U V W X
                  |      |
                  N      Y
                  |      |
                  O      Z
                 / \
                P   Q
```