# Length-Indexed Vectors in Rust

A experimental library providing a compile-time length-checking vector implementation.

## Example

Basic macro-based creation and indexing example:
```rust
extern crate vector;
use vector::index::*;

fn main() {
    let v = vector![1, 3, 4];
    assert_eq!(v.len(), 3);
    assert_eq!(v[_0], 1);
    assert_eq!(v[_1], 3);
    assert_eq!(v[_2], 4);
}
```

Accessing the `Vector` using an out-of-bounds index will fail:
```rust, compile_fail
extern crate vector;
use vector::index::*;

fn main() {
    let v = vector![1, 3, 4];
    assert_eq!(v[_3], 1); // compile fails!
}
```
