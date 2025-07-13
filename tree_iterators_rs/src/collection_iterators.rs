macro_rules! owned_collection_iterator_impl {
    ($struct_name: ident, $inner_iterator: ident, $tree_trait: ident) => {
        pub struct $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: $tree_trait,
        {
            collection: IntoIter::IntoIter,
            tree_traversal_iterator: Option<$inner_iterator<IntoIter::Item>>,
        }

        impl<IntoIter> $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: $tree_trait,
        {
            pub(crate) fn new(into_iter: IntoIter) -> Self {
                Self {
                    collection: into_iter.into_iter(),
                    tree_traversal_iterator: None,
                }
            }
        }

        impl<IntoIter> Iterator for $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: $tree_trait,
        {
            type Item = <IntoIter::Item as $tree_trait>::OwnedValue;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    if let Some(iter) = self.tree_traversal_iterator.as_mut() {
                        let next = iter.next();
                        if next.is_some() {
                            return next;
                        }
                    }

                    self.tree_traversal_iterator = self
                        .collection
                        .next()
                        .map(|item| $inner_iterator::new(item));

                    self.tree_traversal_iterator.as_ref()?;
                }
            }
        }
    };
}

macro_rules! owned_collection_context_iterator_impl {
    ($struct_name: ident, $inner_iterator: ident, $source: ident) => {
       pub struct $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedTreeNode,
        {
            collection: IntoIter::IntoIter,
            index: usize,
            tree_traversal_iterator: Option<$inner_iterator<IntoIter::Item>>,
        }

        impl<IntoIter> $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedTreeNode,
        {
            fn new(source: $source<IntoIter>) -> Self {
                if source.tree_traversal_iterator.is_some() {
                    panic!("Cannot attach metadata to BFS collection iterator after .next() has been called.");
                }

                Self {
                    collection: source.collection,
                    index: usize::MAX,
                    tree_traversal_iterator: None,
                }
            }
        }

        impl<IntoIter> StreamingIterator for $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedTreeNode,
        {
            type Item = TreeContext<
                <IntoIter::Item as OwnedTreeNode>::OwnedValue,
                <IntoIter::Item as OwnedTreeNode>::OwnedChildren,
            >;

            fn advance(&mut self) {
                loop {
                    if let Some(iter) = self.tree_traversal_iterator.as_mut() {
                        let next = iter.next();
                        if next.is_some() {
                            return;
                        }
                    }

                    let next_tree_iterator = self
                        .collection
                        .next();

                    let mut path_with_index = if let Some(tree_iterator) = self.tree_traversal_iterator.as_ref() {
                        Vec::with_capacity(tree_iterator.current_context.path.capacity())
                    } else {
                        Vec::new()
                    };

                    self.index = self.index.wrapping_add(1);
                    path_with_index.push(self.index);

                    self.tree_traversal_iterator = next_tree_iterator
                        .map(|item| $inner_iterator::new(item, path_with_index));

                    if self.tree_traversal_iterator.is_none() {
                        return;
                    }
                }
            }

            fn get(&self) -> Option<&Self::Item> {
                self.tree_traversal_iterator
                    .as_ref()
                    .and_then(|iter| iter.get())
            }
        }

        impl<IntoIter> StreamingIteratorMut for $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedTreeNode,
        {
            fn get_mut(&mut self) -> Option<&mut Self::Item> {
                self.tree_traversal_iterator
                    .as_mut()
                    .and_then(|iter| iter.get_mut())
            }
        }
    };
}

macro_rules! owned_collection_context_no_children_iterator_impl {
    ($struct_name: ident, $inner_iterator: ident, $source: ident) => {
       pub struct $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedTreeNode,
        {
            collection: IntoIter::IntoIter,
            index: usize,
            tree_traversal_iterator: Option<$inner_iterator<IntoIter::Item>>,
        }

        impl<IntoIter> $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedTreeNode,
        {
            fn new(source: $source<IntoIter>) -> Self {
                if source.tree_traversal_iterator.is_some() {
                    panic!("Cannot attach metadata to BFS collection iterator after .next() has been called.");
                }

                Self {
                    collection: source.collection,
                    index: usize::MAX,
                    tree_traversal_iterator: None,
                }
            }
        }

        impl<IntoIter> StreamingIterator for $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedTreeNode,
        {
            type Item = TreeContext<
                <IntoIter::Item as OwnedTreeNode>::OwnedValue,
                ()
            >;

            fn advance(&mut self) {
                loop {
                    if let Some(iter) = self.tree_traversal_iterator.as_mut() {
                        let next = iter.next();
                        if next.is_some() {
                            return;
                        }
                    }

                    let next_tree_iterator = self
                        .collection
                        .next();

                    let mut path_with_index = if let Some(tree_iterator) = self.tree_traversal_iterator.as_ref() {
                        Vec::with_capacity(tree_iterator.current_context.path.capacity())
                    } else {
                        Vec::new()
                    };

                    self.index = self.index.wrapping_add(1);
                    path_with_index.push(self.index);

                    self.tree_traversal_iterator = next_tree_iterator
                        .map(|item| $inner_iterator::new(item, path_with_index));

                    if self.tree_traversal_iterator.is_none() {
                        return;
                    }
                }
            }

            fn get(&self) -> Option<&Self::Item> {
                self.tree_traversal_iterator
                    .as_ref()
                    .and_then(|iter| iter.get())
            }
        }

        impl<IntoIter> StreamingIteratorMut for $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedTreeNode,
        {
            fn get_mut(&mut self) -> Option<&mut Self::Item> {
                self.tree_traversal_iterator
                    .as_mut()
                    .and_then(|iter| iter.get_mut())
            }
        }
    };
}

macro_rules! owned_collection_binary_context_iterator_impl {
    ($struct_name: ident, $inner_iterator: ident, $source: ident) => {
        pub struct $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedBinaryTreeNode,
        {
            collection: IntoIter::IntoIter,
            index: usize,
            tree_traversal_iterator: Option<$inner_iterator<IntoIter::Item>>,
        }

        impl<IntoIter> $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedBinaryTreeNode,
        {
            fn new(source: $source<IntoIter>) -> Self {
                if source.tree_traversal_iterator.is_some() {
                    panic!("Cannot attach metadata after .next() has been called.");
                }

                Self {
                    collection: source.collection,
                    index: usize::MAX,
                    tree_traversal_iterator: None,
                }
            }
        }

        impl<IntoIter> StreamingIterator for $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedBinaryTreeNode,
        {
            type Item = TreeContext<
                <IntoIter::Item as OwnedBinaryTreeNode>::OwnedValue,
                [Option<IntoIter::Item>; 2],
            >;

            fn advance(&mut self) {
                loop {
                    if let Some(iter) = self.tree_traversal_iterator.as_mut() {
                        let next = iter.next();
                        if next.is_some() {
                            return;
                        }
                    }

                    let next_tree_iterator = self.collection.next();

                    let mut path_with_index =
                        if let Some(tree_iterator) = self.tree_traversal_iterator.as_ref() {
                            Vec::with_capacity(tree_iterator.current_context.path.capacity())
                        } else {
                            Vec::new()
                        };

                    self.index = self.index.wrapping_add(1);
                    path_with_index.push(self.index);

                    self.tree_traversal_iterator =
                        next_tree_iterator.map(|item| $inner_iterator::new(item, path_with_index));

                    if self.tree_traversal_iterator.is_none() {
                        return;
                    }
                }
            }

            fn get(&self) -> Option<&Self::Item> {
                self.tree_traversal_iterator
                    .as_ref()
                    .and_then(|iter| iter.get())
            }
        }

        impl<IntoIter> StreamingIteratorMut for $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedBinaryTreeNode,
        {
            fn get_mut(&mut self) -> Option<&mut Self::Item> {
                self.tree_traversal_iterator
                    .as_mut()
                    .and_then(|iter| iter.get_mut())
            }
        }
    };
}

macro_rules! owned_collection_binary_context_no_children_iterator_impl {
    ($struct_name: ident, $inner_iterator: ident, $source: ident) => {
        pub struct $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedBinaryTreeNode,
        {
            collection: IntoIter::IntoIter,
            index: usize,
            tree_traversal_iterator: Option<$inner_iterator<IntoIter::Item>>,
        }

        impl<IntoIter> $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedBinaryTreeNode,
        {
            fn new(source: $source<IntoIter>) -> Self {
                if source.tree_traversal_iterator.is_some() {
                    panic!("Cannot attach metadata after .next() has been called.");
                }

                Self {
                    collection: source.collection,
                    index: usize::MAX,
                    tree_traversal_iterator: None,
                }
            }
        }

        impl<IntoIter> StreamingIterator for $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedBinaryTreeNode,
        {
            type Item = TreeContext<<IntoIter::Item as OwnedBinaryTreeNode>::OwnedValue, ()>;

            fn advance(&mut self) {
                loop {
                    if let Some(iter) = self.tree_traversal_iterator.as_mut() {
                        let next = iter.next();
                        if next.is_some() {
                            return;
                        }
                    }

                    let next_tree_iterator = self.collection.next();

                    let mut path_with_index =
                        if let Some(tree_iterator) = self.tree_traversal_iterator.as_ref() {
                            Vec::with_capacity(tree_iterator.current_context.path.capacity())
                        } else {
                            Vec::new()
                        };

                    self.index = self.index.wrapping_add(1);
                    path_with_index.push(self.index);

                    self.tree_traversal_iterator =
                        next_tree_iterator.map(|item| $inner_iterator::new(item, path_with_index));

                    if self.tree_traversal_iterator.is_none() {
                        return;
                    }
                }
            }

            fn get(&self) -> Option<&Self::Item> {
                self.tree_traversal_iterator
                    .as_ref()
                    .and_then(|iter| iter.get())
            }
        }

        impl<IntoIter> StreamingIteratorMut for $struct_name<IntoIter>
        where
            IntoIter: IntoIterator,
            IntoIter::Item: OwnedBinaryTreeNode,
        {
            fn get_mut(&mut self) -> Option<&mut Self::Item> {
                self.tree_traversal_iterator
                    .as_mut()
                    .and_then(|iter| iter.get_mut())
            }
        }
    };
}

macro_rules! mut_borrowed_collection_iterator_impl {
    ($struct_name: ident, $inner_iterator: ident, $tree_trait: tt) => {
        pub struct $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a mut Node>,
            Node: $tree_trait<'a>,
        {
            collection: IntoIter::IntoIter,
            tree_traversal_iterator: Option<$inner_iterator<'a, Node>>,
        }

        impl<'a, IntoIter, Node> $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a mut Node>,
            Node: $tree_trait<'a>,
        {
            pub(crate) fn new(into_iter: IntoIter) -> Self {
                Self {
                    collection: into_iter.into_iter(),
                    tree_traversal_iterator: None,
                }
            }
        }

        impl<'a, IntoIter, Node> Iterator for $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a mut Node>,
            Node: $tree_trait<'a>,
        {
            type Item = Node::MutBorrowedValue;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    if let Some(iter) = self.tree_traversal_iterator.as_mut() {
                        let next = iter.next();
                        if next.is_some() {
                            return next;
                        }
                    }

                    self.tree_traversal_iterator = self
                        .collection
                        .next()
                        .map(|item| $inner_iterator::new(item));

                    self.tree_traversal_iterator.as_ref()?;
                }
            }
        }
    };
}

macro_rules! mut_borrowed_collection_context_iterator_impl {
    ($struct_name: ident, $inner_iterator: ident, $source: ident) => {
       pub struct $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a mut Node>,
            Node: MutBorrowedTreeNode<'a>,
        {
            collection: IntoIter::IntoIter,
            index: usize,
            tree_traversal_iterator: Option<$inner_iterator<'a, Node>>,
        }

        impl<'a, IntoIter, Node> $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a mut Node>,
            Node: MutBorrowedTreeNode<'a>,
        {
            fn new(source: $source<'a, IntoIter, Node>) -> Self {
                if source.tree_traversal_iterator.is_some() {
                    panic!("Cannot attach metadata to BFS collection iterator after .next() has been called.");
                }

                Self {
                    collection: source.collection,
                    index: usize::MAX,
                    tree_traversal_iterator: None,
                }
            }
        }

        impl<'a, IntoIter, Node> StreamingIterator for $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a mut Node>,
            Node: MutBorrowedTreeNode<'a>,
        {
            type Item = TreeContext<
                Node::MutBorrowedValue,
                Node::MutBorrowedChildren,
            >;

            fn advance(&mut self) {
                loop {
                    if let Some(iter) = self.tree_traversal_iterator.as_mut() {
                        let next = iter.next();
                        if next.is_some() {
                            return;
                        }
                    }

                    let next_tree_iterator = self
                        .collection
                        .next();

                    let mut path_with_index = if let Some(tree_iterator) = self.tree_traversal_iterator.as_ref() {
                        Vec::with_capacity(tree_iterator.current_context.path.capacity())
                    } else {
                        Vec::new()
                    };

                    self.index = self.index.wrapping_add(1);
                    path_with_index.push(self.index);

                    self.tree_traversal_iterator = next_tree_iterator
                        .map(|item| $inner_iterator::new(item, path_with_index));

                    if self.tree_traversal_iterator.is_none() {
                        return;
                    }
                }
            }

            fn get(&self) -> Option<&Self::Item> {
                self.tree_traversal_iterator
                    .as_ref()
                    .and_then(|iter| iter.get())
            }
        }

        impl<'a, IntoIter, Node> StreamingIteratorMut for $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a mut Node>,
            Node: MutBorrowedTreeNode<'a>,
        {
            fn get_mut(&mut self) -> Option<&mut Self::Item> {
                self.tree_traversal_iterator
                    .as_mut()
                    .and_then(|iter| iter.get_mut())
            }
        }
    };
}

macro_rules! borrowed_binary_collection_context_iterator_impl {
    ($struct_name: ident, $inner_iterator: ident, $source: ident) => {
       pub struct $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a Node>,
            Node: BorrowedBinaryTreeNode<'a>,
        {
            collection: IntoIter::IntoIter,
            index: usize,
            tree_traversal_iterator: Option<$inner_iterator<'a, Node>>,
        }

        impl<'a, IntoIter, Node> $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a Node>,
            Node: BorrowedBinaryTreeNode<'a>,
        {
            fn new(source: $source<'a, IntoIter, Node>) -> Self {
                if source.tree_traversal_iterator.is_some() {
                    panic!("Cannot attach metadata to BFS collection iterator after .next() has been called.");
                }

                Self {
                    collection: source.collection,
                    index: usize::MAX,
                    tree_traversal_iterator: None,
                }
            }
        }

        impl<'a, IntoIter, Node> StreamingIterator for $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a Node>,
            Node: BorrowedBinaryTreeNode<'a>,
        {
            type Item = TreeContext<
                Node::BorrowedValue,
                [Option<&'a Node>; 2]
            >;

            fn advance(&mut self) {
                loop {
                    if let Some(iter) = self.tree_traversal_iterator.as_mut() {
                        let next = iter.next();
                        if next.is_some() {
                            return;
                        }
                    }

                    let next_tree_iterator = self
                        .collection
                        .next();

                    let mut path_with_index = if let Some(tree_iterator) = self.tree_traversal_iterator.as_ref() {
                        Vec::with_capacity(tree_iterator.current_context.path.capacity())
                    } else {
                        Vec::new()
                    };

                    self.index = self.index.wrapping_add(1);
                    path_with_index.push(self.index);

                    self.tree_traversal_iterator = next_tree_iterator
                        .map(|item| $inner_iterator::new(item, path_with_index));

                    if self.tree_traversal_iterator.is_none() {
                        return;
                    }
                }
            }

            fn get(&self) -> Option<&Self::Item> {
                self.tree_traversal_iterator
                    .as_ref()
                    .and_then(|iter| iter.get())
            }
        }
    };
}

macro_rules! borrowed_collection_iterator_impl {
    ($struct_name: ident, $inner_iterator: ident, $tree_trait: tt) => {
        pub struct $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a Node>,
            Node: $tree_trait<'a>,
        {
            collection: IntoIter::IntoIter,
            tree_traversal_iterator: Option<$inner_iterator<'a, Node>>,
        }

        impl<'a, IntoIter, Node> $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a Node>,
            Node: $tree_trait<'a>,
        {
            pub(crate) fn new(into_iter: IntoIter) -> Self {
                Self {
                    collection: into_iter.into_iter(),
                    tree_traversal_iterator: None,
                }
            }
        }

        impl<'a, IntoIter, Node> Iterator for $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a Node>,
            Node: $tree_trait<'a>,
        {
            type Item = Node::BorrowedValue;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    if let Some(iter) = self.tree_traversal_iterator.as_mut() {
                        let next = iter.next();
                        if next.is_some() {
                            return next;
                        }
                    }

                    self.tree_traversal_iterator = self
                        .collection
                        .next()
                        .map(|item| $inner_iterator::new(item));

                    self.tree_traversal_iterator.as_ref()?;
                }
            }
        }
    };
}

macro_rules! borrowed_collection_context_iterator_impl {
    ($struct_name: ident, $inner_iterator: ident, $source: ident) => {
       pub struct $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a Node>,
            Node: BorrowedTreeNode<'a>,
        {
            collection: IntoIter::IntoIter,
            index: usize,
            tree_traversal_iterator: Option<$inner_iterator<'a, Node>>,
        }

        impl<'a, IntoIter, Node> $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a Node>,
            Node: BorrowedTreeNode<'a>,
        {
            fn new(source: $source<'a, IntoIter, Node>) -> Self {
                if source.tree_traversal_iterator.is_some() {
                    panic!("Cannot attach metadata to BFS collection iterator after .next() has been called.");
                }

                Self {
                    collection: source.collection,
                    index: usize::MAX,
                    tree_traversal_iterator: None,
                }
            }
        }

        impl<'a, IntoIter, Node> StreamingIterator for $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a Node>,
            Node: BorrowedTreeNode<'a>,
        {
            type Item = TreeContext<
                Node::BorrowedValue,
                Node::BorrowedChildren,
            >;

            fn advance(&mut self) {
                loop {
                    if let Some(iter) = self.tree_traversal_iterator.as_mut() {
                        let next = iter.next();
                        if next.is_some() {
                            return;
                        }
                    }

                    let next_tree_iterator = self
                        .collection
                        .next();

                    let mut path_with_index = if let Some(tree_iterator) = self.tree_traversal_iterator.as_ref() {
                        Vec::with_capacity(tree_iterator.current_context.path.capacity())
                    } else {
                        Vec::new()
                    };

                    self.index = self.index.wrapping_add(1);
                    path_with_index.push(self.index);

                    self.tree_traversal_iterator = next_tree_iterator
                        .map(|item| $inner_iterator::new(item, path_with_index));

                    if self.tree_traversal_iterator.is_none() {
                        return;
                    }
                }
            }

            fn get(&self) -> Option<&Self::Item> {
                self.tree_traversal_iterator
                    .as_ref()
                    .and_then(|iter| iter.get())
            }
        }
    };
}

macro_rules! mut_borrowed_binary_collection_context_iterator_impl {
    ($struct_name: ident, $inner_iterator: ident, $source: ident) => {
       pub struct $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a mut Node>,
            Node: MutBorrowedBinaryTreeNode<'a>,
        {
            collection: IntoIter::IntoIter,
            index: usize,
            tree_traversal_iterator: Option<$inner_iterator<'a, Node>>,
        }

        impl<'a, IntoIter, Node> $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a mut Node>,
            Node: MutBorrowedBinaryTreeNode<'a>,
        {
            fn new(source: $source<'a, IntoIter, Node>) -> Self {
                if source.tree_traversal_iterator.is_some() {
                    panic!("Cannot attach metadata to BFS collection iterator after .next() has been called.");
                }

                Self {
                    collection: source.collection,
                    index: usize::MAX,
                    tree_traversal_iterator: None,
                }
            }
        }

        impl<'a, IntoIter, Node> StreamingIterator for $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a mut Node>,
            Node: MutBorrowedBinaryTreeNode<'a>,
        {
            type Item = TreeContext<
                Node::MutBorrowedValue,
                [Option<&'a mut Node>; 2]
            >;

            fn advance(&mut self) {
                loop {
                    if let Some(iter) = self.tree_traversal_iterator.as_mut() {
                        let next = iter.next();
                        if next.is_some() {
                            return;
                        }
                    }

                    let next_tree_iterator = self
                        .collection
                        .next();

                    let mut path_with_index = if let Some(tree_iterator) = self.tree_traversal_iterator.as_ref() {
                        Vec::with_capacity(tree_iterator.current_context.path.capacity())
                    } else {
                        Vec::new()
                    };

                    self.index = self.index.wrapping_add(1);
                    path_with_index.push(self.index);

                    self.tree_traversal_iterator = next_tree_iterator
                        .map(|item| $inner_iterator::new(item, path_with_index));

                    if self.tree_traversal_iterator.is_none() {
                        return;
                    }
                }
            }

            fn get(&self) -> Option<&Self::Item> {
                self.tree_traversal_iterator
                    .as_ref()
                    .and_then(|iter| iter.get())
            }
        }

        impl<'a, IntoIter, Node> StreamingIteratorMut for $struct_name<'a, IntoIter, Node>
        where
            IntoIter: IntoIterator<Item = &'a mut Node>,
            Node: MutBorrowedBinaryTreeNode<'a>,

        {
            fn get_mut(&mut self) -> Option<&mut Self::Item> {
                self.tree_traversal_iterator
                    .as_mut()
                    .and_then(|iter| iter.get_mut())
            }
        }
    };
}

pub(crate) use borrowed_binary_collection_context_iterator_impl;
pub(crate) use borrowed_collection_context_iterator_impl;
pub(crate) use borrowed_collection_iterator_impl;
pub(crate) use mut_borrowed_binary_collection_context_iterator_impl;
pub(crate) use mut_borrowed_collection_context_iterator_impl;
pub(crate) use mut_borrowed_collection_iterator_impl;
pub(crate) use owned_collection_binary_context_iterator_impl;
pub(crate) use owned_collection_binary_context_no_children_iterator_impl;
pub(crate) use owned_collection_context_iterator_impl;
pub(crate) use owned_collection_context_no_children_iterator_impl;
pub(crate) use owned_collection_iterator_impl;
