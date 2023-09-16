# Tree Iterators

tree-iterators-rs is a library built to provide you with the utilities/iterators to easily work with tree data structures in Rust. It provides a default implementation for the majority use cases. These include:
1. TreeNode<T> - This struct has a vec-based heap-allocated list of children. [documentation](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/struct.TreeNode.html)
2. BinaryTreeNode<T> - This struct contains an optional boxed reference to a left and right node. [documentation](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/struct.BinaryTreeNode.html)

This crate is written such that you can build your own implementations as well using other collection types.

### Benefits

This crate uses no unsafe code!

The largest benefit of using this library is that the various tree iterators are interchangeable in the for/while loop syntax. This means you can write your code to use a breadth-first-search today and trivially swap it out for a depth-first pre or postorder search tomorrow without significantly changing the structure of your code. Syntactically, this library is also able to hide a lot of complexity and provide a friendly interface.

The other benefit of this library is its tight integration with the rest of Rust's iterators. Because each API returns an iterator, you can proceed to use the iterator APIs like filter, map, and reduce.

## Getting Started

The easiest way to get started is to simply add this crate as a dependency and add a using statement to pull in its prelude (tree_iterators_rs::prelude::*). You can then create your data structure using the BinaryTreeNode or TreeNode struct provided and build on top of it. These structs provide default implementations of all functionality in this crate. The methods that come attached to these structs include the following (all of which can be found in [this file](https://github.com/mr-adult/tree-iterators-rs/blob/main/src/prelude.rs)):
- Owned Iterator APIs - these take ownership of the TreeNode similarly to an into_iter() call.
	- [bfs()](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.OwnedTreeNode.html#method.bfs)
	- [dfs_preorder()](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.OwnedTreeNode.html#method.dfs_preorder)
	- [dfs_postorder()](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.OwnedTreeNode.html#method.dfs_postorder)
	- [dfs_inorder()](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.OwnedBinaryTreeNode.html#method.dfs_inorder) - this iterator is only available on BinaryTreeNode
- Mutable borrow APIs - these do not take ownership and behave similarly to an iter_mut() call.
	- [bfs_iter_mut()](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.MutBorrowedTreeNode.html#method.bfs_iter_mut)
	- [dfs_preorder_iter_mut()](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.MutBorrowedTreeNode.html#method.dfs_preorder_iter_mut)
	- [dfs_postorder_iter_mut()](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.MutBorrowedTreeNode.html#method.dfs_postorder_iter_mut)
	- [dfs_inorder_iter_mut](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.MutBorrowedBinaryTreeNode.html#method.dfs_inorder_iter_mut) - this iterator is only available on BinaryTreeNode
- Borrow APIs - these do not take ownership and behave similarly to an iter() call.
	- [bfs_iter()](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.BorrowedTreeNode.html#method.bfs_iter)
	- [dfs_preorder_iter()](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.BorrowedTreeNode.html#method.dfs_preorder_iter)
	- [dfs_postorder_iter()](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.BorrowedTreeNode.html#method.dfs_postorder_iter)
	- [dfs_inorder_iter()](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.BorrowedBinaryTreeNode.html#method.dfs_inorder_iter) - this iterator is only available on BinaryTreeNode
- In addition, all of these APIs have an additional method (attach_ancestors()) that can be chained. This method changes the iterator to return a slice of references to the current tree value, as well as all of its parent values. For more details, see Examples.


## Examples

For all of the following examples, we will use the tree structure as follows. It is complicated enough to convey the tree iterators' behavior thoroughly without being overwhelming. Code to build this tree using both the BinaryTreeNode and TreeNode structs is provided below.

```mermaid
flowchart TD
A[0] --> B{1}
A --> C{2}
B --> D{3}
B --> E{4}
D --> Z{None}
D --> Y{None}
E --> X{None}
E --> W{None}
C --> F{5}
F --> V{None}
F --> U{None}
C --> G{6}
G --> H{7}
G --> T{None}
H --> S{None}
H --> I{8}
I --> J{9}
I --> R{None}
J --> K{10}
J --> Q{None}
K --> P{None}
K --> O{None}
``` 
Using BinaryTreeNode
```rust
use tree_iterators_rs::prelude::*;

fn create_example_binary_tree() -> BinaryTreeNode<usize> {
	BinaryTreeNode { 
		value: 0, 
		left: Some(
			Box::new(
				BinaryTreeNode {
					value: 1,
					left: Some(
						Box::new(
							BinaryTreeNode {
								value: 3,
								left: None,
								right: None,
							}
						)
					),
					right: Some(
						Box::new(
							BinaryTreeNode {
								value: 4,
								left: None,
								right: None,
							}
						)
					),
				}
			)
		), 
		right: Some(
			Box::new(
				BinaryTreeNode {
					value: 2,
					left: Some(
						Box::new(
							BinaryTreeNode {
								value: 5,
								left: None,
								right: None,
							}
						)
					),
					right: Some(
						Box::new(
							BinaryTreeNode {
								value: 6,
								left: Some(
									Box::new(
										BinaryTreeNode { 
											value: 7, 
											left: None, 
											right: Some(
												Box::new(
													BinaryTreeNode { 
														value: 8, 
														left: Some(
															Box::new(
																BinaryTreeNode {
																	value: 9,
																	left: None,
																	right: Some(
																		Box::new(
																			BinaryTreeNode { 
																				value: 10, 
																				left: None, 
																				right: None 
																			}
																		)
																	)
																}
															)
														), 
														right: None 
													}
												)
											) 
										}
									)
								),
								right: None,
							}
						)
					)
				}
			)
		) 
	}
}
```
Using TreeNode
```rust
use tree_iterators_rs::prelude::*;

fn create_example_tree() -> TreeNode<usize> {
	TreeNode {
		value: 0,
		children: Some(vec![
			TreeNode {
				value: 1,
				children: Some(vec![
					TreeNode {
						value: 3,
						children: None
					},
					TreeNode {
						value: 4,
						children: None
					}
				])
			},
			TreeNode {
				value: 2,
				children: Some(vec![
					TreeNode {
						value: 5,
						children: None
					},
					TreeNode {
						value: 6,
						children: Some(vec![
							TreeNode {
								value: 7,
								children: Some(vec![
									TreeNode {
										value: 8,
										children: Some(vec![
											TreeNode {
												value: 9,
												children: Some(vec![
													TreeNode {
														value: 10,
														children: None
													}
												])
											}
										])
									}
								])
							}
						])
					}
				])
			}
		])
	}
}
```

### Breadth-First-Search (BFS)

All 3 APIs' usages are identical other than their borrowing model, so only one example will be given.
```rust
// Tree creation (see above documentation)
let root = create_example_tree();

let mut result = String::new();
for value in root.bfs() {
	result.push_str(&value.to_string());
	result.push_str(", ");
}

// result: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
println!("{}", result);
```

This code could also be written using Rust's iterator APIs:
```rust
// Tree creation (see above documentation)
let root = create_example_tree();

let result = 
	root.bfs()
		.map(|val| val.to_string())
		.collect::<Vec<String>>()
		.join(", ");

// result: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
println!("{}", result);
```

An equivalent set of code not using this crate would look like this:
```rust
// Tree creation (see above documentation)
let root = create_example_tree();

let mut result = String::new();

let mut queue = VecDeque::new();
queue.push_back(root);
while queue.len() > 0 {
	if let Some(front) = queue.pop_front() {
		if let Some(children) = front.children {
			for child in children {
				queue.push_back(child);
			}
		}

		result.push_str(&front.value.to_string());
		result.push_str(", ");
	}
}

// result: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
println!("{}", result);
```

### Depth-First Preorder Search (DFS Preorder)

Similarly to the BFS example, all 3 APIs' usages are identical other than their borrowing model, so only one example will be given.

```rust
// Tree creation (see above documentation)
let root = create_example_tree();

let mut result = String::new();
for value in root.dfs_preorder() {
	result.push_str(&value.to_string());
	result.push_str(", ");
}

// result: 0, 1, 3, 4, 2, 5, 6, 7, 8, 9, 10,
println!("{}", result);
```

This code could also be written using Rust's iterator APIs:
```rust
// Tree creation (see above documentation)
let root = create_example_tree();

let result = 
	root.dfs_preorder()
		.map(|val| val.to_string())
		.collect::<Vec<String>>()
		.join(", ");

// result: 0, 1, 3, 4, 2, 5, 6, 7, 8, 9, 10,
println!("{}", result);
```

An equivalent set of code not using this crate would look like this:
```rust
// Tree creation (see above documentation)
let root = create_example_tree();

let mut result = String::new();

let mut stack = vec![root];
while stack.len() > 0 {
	if let Some(top) = stack.pop() {
		if let Some(mut children) = top.children {
			children.reverse();
			for child in children {
				stack.push(child);
			}
		}

		result.push_str(&top.value.to_string());
		result.push_str(", ");
	}
}

// result: 0, 1, 3, 4, 2, 5, 6, 7, 8, 9, 10,
println!("{}", result);
```

### Binary Trees Only - Depth First In Order Search (DFS In Order)

Similarly to the other examples, all 3 APIs' usages are identical other than their borrowing model, so only one example will be given.
```rust
// Tree creation (see above documentation)
let root = create_example_binary_tree();

let mut result = String::new();
for value in root.dfs_inorder() {
	result.push_str(&value.to_string());
	result.push_str(", ");
}

// result: 3, 1, 4, 0, 5, 2, 7, 9, 10, 8, 6,
println!("{}", result);
```
This code could also be written using Rust's iterator APIs:
```rust
// Tree creation (see above documentation)
let root = create_example_binary_tree();

let mut result = String::new();
for value in root.dfs_inorder() {
	result.push_str(&value.to_string());
	result.push_str(", ");
}

// result: 3, 1, 4, 0, 5, 2, 7, 9, 10, 8, 6,
println!("{}", result);
```
An equivalent set of code not using this crate would look like the following. It is important to note that the dfs_inorder API does not use recursion and thus does not incur the overhead of stack frames that this example does. For purposes of simplicity in the example, recursion was chosen. Rust's ownership model is difficult to work around with this traversal type and would make the example more complex than necessary.
```rust
fn dfs_inorder(node: Option<Box<BinaryTreeNode<usize>>>, result: &mut String) {
	match node {
		None => {}
		Some(node) => {
			dfs_inorder(node.left, result);
			result.push_str(&node.value.to_string());
			result.push_str(", ");
			dfs_inorder(node.right, result)
		}
	}            
}

// Tree creation (see above documentation)
let root = create_example_binary_tree();

let mut result = String::new();
dfs_inorder(Some(Box::new(root)), &mut result);

// result: 3, 1, 4, 0, 5, 2, 7, 9, 10, 8, 6,
println!("{}", result);
```

### Depth First Postorder Search (DFS Postorder)

Similarly to the BFS and DFS preorder examples, all 3 APIs' usages are identical other than their borrowing model, so only one example will be given.
```rust
// Tree creation (see above documentation)
let root = create_example_tree();

let mut result = String::new();
for value in root.dfs_postorder() {
	result.push_str(&value.to_string());
	result.push_str(", ");
}

// result: 3, 4, 1, 5, 10, 9, 8, 7, 6, 2, 0,
println!("{}", result);
```
This code could also be written using Rust's iterator APIs:
```rust
// Tree creation (see above documentation)
let root = create_example_tree();

let result = 
	root.dfs_postorder()
		.map(|val| val.to_string())
		.collect::<Vec<String>>()
		.join(", ");

// result: 3, 4, 1, 5, 10, 9, 8, 7, 6, 2, 0,
println!("{}", result);
```

An equivalent set of code not using this crate would look like the following. It is important to note that the dfs_postorder API does not use recursion and thus does not incur the overhead of stack frames that this example does. For purposes of simplicity in the example, recursion was chosen. Rust's ownership model is difficult to work around with this traversal type and would make the example more complex than necessary.
```rust
fn dfs_postorder(node: TreeNode<usize>, result: &mut String) {
	if let Some(children) = node.children {
		for child in children {
			dfs_postorder(child, result);
		}
	}

	result.push_str(", ");
	result.push_str(&node.value.to_string());
}

// Tree creation (see above documentation)
let root = create_example_tree();

let mut result = String::new();
dfs_postorder(root, &mut result);

// result: 3, 4, 1, 5, 10, 9, 8, 7, 6, 2, 0,
println!("{}", result);
```

## Attach Ancestors

attach_ancestors() is a method that can be called after any of the above APIs to change the iterator structure into one that returns a slice of all ancestors and the current value in the tree. If one of these is called, the (now streaming) iterator will yield a slice where the item at index 0 is the root value, the item at index len() - 1 is the current value, and everything in between is the other ancestors. As an example, when we are at the value of 10 in our traversal, the slice will look like this: \[0, 2, 6, 7, 8, 9, 10\].

For example, we can use this API to filter down to only the values where all of the ancestors are even numbers in the example tree. 

NOTE: Be sure to add a use  statement for streaming_iterator::StreamingIterator to pull in the filter, map, reduce, for_each, etc. methods.

```rust
use streaming_iterator::StreamingIterator;

let root = create_example_tree();
let mut result = String::new();

root.dfs_preorder_iter()
	.attach_ancestors()
	.filter(|slice| 
		slice.iter().all(|value| **value % 2 == 0)
	)
	.map(|slice| slice[slice.len() - 1])
	.for_each(|value| {
		result.push(' ');
		result.push_str(&value.to_string())
	});

// result: 0 2 6
println!("{}", result);
```

We can do the same with the in order iterator:

```rust
let root = create_example_binary_tree();
let mut result = String::new();

root.dfs_inorder_iter()
	.attach_ancestors()
	.filter(|slice| 
		slice.iter().all(|value| **value % 2 == 0)
	)
	.map(|slice| slice[slice.len() - 1])
	.for_each(|value| {
		result.push(' ');
		result.push_str(&value.to_string())
	});

// result: 0 2 6
println!("{}", result);
```

We can do the same with the postorder iterator to get the result in reverse:

```rust
let root = create_example_tree();
let mut result = String::new();

root.dfs_postorder_iter()
	.attach_ancestors()
	.filter(|slice| 
		slice.iter().all(|value| **value % 2 == 0)
	)
	.map(|slice| slice[slice.len() - 1])
	.for_each(|value| {
		result.push(' ');
		result.push_str(&value.to_string())
	});

// result: 6 2 0
println!("{}", result);
```

And we can do the same with a breadth-first search. This just so happens to yield the same result as the depth first preorder iterator:

```rust
let root = create_example_tree();
let mut result = String::new();

root.bfs_iter()
	.attach_ancestors()
	.filter(|slice| 
		slice.iter().all(|value| **value % 2 == 0)
	)
	.map(|slice| slice[slice.len() - 1])
	.for_each(|value| {
		result.push(' ');
		result.push_str(&value.to_string())
	});

// result: 0 2 6
println!("{}", result);
```

## Custom Tree Node Implementations

This crates' APIs are powered by 3 traits. The traits include:
1. [OwnedTreeNode](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.OwnedTreeNode.html)/[OwnedBinaryTreeNode](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.OwnedBinaryTreeNode.html) - this trait powers the bfs(), dfs_preorder(), and dfs_postorder() APIs.
2. [MutBorrowedTreeNode](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.MutBorrowedTreeNode.html)/[MutBorrowedBinaryTreeNode](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.MutBorrowedBinaryTreeNode.html) - this trait powers the bfs_iter_mut(), dfs_preorder_iter_mut() and dfs_postorder() APIs.
3. [BorrowedTreeNode](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.BorrowedTreeNode.html)/[BorrowedBinaryTreeNode](https://docs.rs/tree_iterators_rs/latest/tree_iterators_rs/prelude/trait.BorrowedBinaryTreeNode.html) - this trait powers the bfs_iter(), dfs_preorder_iter(), and dfs_postorder_iter() APIs.

You may pick and choose which of the traits you implement to suit your needs. They are identical apart from some naming conventions and the ownership of the value returned from their required method implementation.

### Example

As an example, we can implement another TreeNode variant that uses a Linked List to hold its children. All code here is MIT licensed and you are free to directly copy or modify it as you see fit subject to the conditions of the license.
```rust
use crate::prelude::*;
use std::collections::LinkedList;

struct LLTreeNode<T> {
    value: T,
    children: LinkedList<LLTreeNode<T>>
}
```

This is a good start, but realistically we want it to implement all of the tree traversal APIs available to us. 

#### Owned implementation

We can start with the OwnedTreeNode implementation as follows. Since we chose to use a LinkedList as our children property, we have to wrap its iterator in a Some() Option variant. The hardest part of implementing this is figuring out the type of LinkedList's into_iter() method return type.
```rust
use std::collections::linked_list::IntoIter;

impl<T> OwnedTreeNode for LLTreeNode<T> {
    type OwnedValue = T;
    type OwnedChildren = IntoIter<LLTreeNode<T>>;

    fn get_value_and_children(self) -> (Self::OwnedValue, Option<Self::OwnedChildren>) {
        (
            self.value,
            Some(self.children.into_iter())
        )
    }
}
```
Now that we have implemented OwnedTreeNode, our type has the bfs(), dfs_preorder(), and dfs_postorder() methods.

#### Mutable Borrow Implementation

The mutable borrow implementation is very similar to the owned one. The only difference is that the 'Value' associated type changed to a mutable reference and we are calling iter_mut() instead of into_iter().
```rust
use std::collections::linked_list::IterMut;

impl<'a, T> MutBorrowedTreeNode<'a> for LLTreeNode<T> 
    where Self: 'a {

    type MutBorrowedValue = &'a mut T;
    type MutBorrowedChildren = IterMut<'a, LLTreeNode<T>>;

    fn get_value_and_children_iter_mut(&'a mut self) -> (Self::MutBorrowedValue, Option<Self::MutBorrowedChildren>) {
        (
            &mut self.value,
            Some(self.children.iter_mut())
        )
    }
}
```
Now that we have implemented MutBorrowedTreeNode, our type has the bfs_iter_mut(), dfs_preorder_iter_mut(), and dfs_postorder_iter_mut() methods.

#### Borrow Implementation

The borrow implementation is also very similar to the owned one. The only difference is that the 'Value' associated type changed to an immutable reference and we are calling iter() instead of into_iter().
```rust
use std::collections::linked_list::Iter;

impl<'a, T> BorrowedTreeNode<'a> for LLTreeNode<T> 
    where Self: 'a {

    type BorrowedValue = &'a T;
    type BorrowedChildren = Iter<'a, LLTreeNode<T>>;

    fn get_value_and_children_iter(&'a self) -> (Self::BorrowedValue, Option<Self::BorrowedChildren>) {
        (
            &self.value,
            Some(self.children.iter())
        )
    }
}
```
Now that we have implemented BorrowedTreeNode, our type has the bfs_iter(), dfs_preorder_iter(), and dfs_postorder_iter() methods.