The list of all ancestors of the current node (including the current node).
Ex. given the following tree, some example ancestor lists would be:
- ancestors at 'A' = \[A\]
- ancestors at 'D' = \[A, D\]
- ancestors at 'H' = \[A, C, H\]
- ancestors at 'K' = \[A, D, K\]
- ancestors at 'Q' = \[A, D, M, N, O, Q\]
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