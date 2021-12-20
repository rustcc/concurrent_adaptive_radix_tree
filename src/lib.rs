//! Simd+concurrent implementation of adaptive radix tree.
//!
//! A common prefixes of the keys in a adaptive radix tree are represented by a shared path,
//! which is widely used in daily use of database and filesystem. The major difference between the radix tree and 
//! the adaptive radix tree is its variable size for each node based on the number of child elements, which grows 
//! while adding new entries. Hence, the adaptive radix tree leads to a better use of space without reducing its speed.
//!
//! See [Adaptive radix tree](https://en.wikipedia.org/wiki/Radix_tree) for more details.
//!
//! # Examples
//!
//! ```
//! use adaptive radix_tree::adaptive radixMap;
//!
//! let mut map = PatriciaMap::new();
//! map.insert("foo", 1);
//! map.insert("bar", 2);
//! map.insert("baz", 3);
//! assert_eq!(map.len(), 3);
//!
//! assert_eq!(map.get("foo"), Some(&1));
//! assert_eq!(map.get("bar"), Some(&2));
//! assert_eq!(map.get("baz"), Some(&3));
//! ```
