#![no_std]
#![doc = include_str!("../README.md")]
#![allow(clippy::type_complexity)]
#![allow(clippy::vec_init_then_push)]
extern crate alloc;
extern crate core;

pub mod bfs_iterators;
pub(crate) mod collection_iterators;
pub mod dfs_inorder_iterators;
pub mod dfs_postorder_iterators;
pub mod dfs_preorder_iterators;
pub mod examples;
pub mod leaves_iterators;
pub mod prelude;
mod tree_collection_iterators;
mod tree_context;
pub(crate) mod tree_iterators;
