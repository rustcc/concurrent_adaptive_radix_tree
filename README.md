# Concurrent Adaptive Radix Tree

Simd+concurrent implementation of adaptive radix tree.

SIMD is using rust std's portable SIMD, which is currently only available on nightly, and adopts to any instruction sets that LLVM simd supports.

```rust
use concurrent_adaptive_radix_tree::ARTMap;

let mut map = ARTMap::new();
map.insert("foo", 1);
map.insert("bar", 2);
map.insert("baz", 3);
assert_eq!(map.len(), 3);

assert_eq!(map.get("foo"), Some(&1));
assert_eq!(map.get("bar"), Some(&2));
assert_eq!(map.get("baz"), Some(&3));
```