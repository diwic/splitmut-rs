# SplitMut 

A Rust library for safely retrieving multiple mutable values within the same collection.

[API Documentation](http://diwic.github.io/rs-docs/splitmut/index.html)
[Crates.io](http://crates.io/crates/splitmut)

`get2_mut`, `get3_mut` and `get4_mut` return a tuple or 2, 3 or 4 values, each one of them being
one of:

 * `Ok(&mut V)`
 * `Err(SplitMutError::NoValue)` in case there was no value for the key (i e, when your usual `get_mut` would have returned `None`)
 * `Err(SplitMutError::SameValue)` in case the same value has already been returned earlier in the tuple. 

Add `use splitmut::SplitMut` to your code have these functions implemented for mutable slices, Vec, VecDeque, HashMap and BTreeMap. 

# Example

```rust
extern crate splitmut;

use std::collections::HashMap;
use splitmut::{SplitMut, SplitMutError};

// Create a hashmap
let mut h = HashMap::new();
h.insert(1, "Hello");
h.insert(2, "world");

// Swap two values easily
{
    let (m1, m2) = h.get2_mut(&1, &2);
    std::mem::swap(m1.unwrap(), m2.unwrap());
}
assert_eq!(h.get(&1), Some(&"world"));
assert_eq!(h.get(&2), Some(&"Hello"));

// Show error handling
let (m0, m1a, m1b) = h.get3_mut(&0, &1, &1);
// No value for the key "0"
assert_eq!(m0, Err(SplitMutError::NoValue));
// First value for the key "1" is returned successfully
assert_eq!(m1a, Ok(&mut "world"));
// Second value for the key "1" returns an error
assert_eq!(m1b, Err(SplitMutError::SameValue));
```

