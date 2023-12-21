# const-sorted-lut

Compile-time sorted `const` lookup tables that use binary search for lookup.

## Example

The following example shows how to use a 6-byte array as a key type via a newtype.

```rust
use const_sorted_lut::impl_const_sorted_lut;
use core::{borrow::Borrow, cmp::Ordering};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MyKey(pub [u8; 6]);

// This allows `&[u8; 6]` to be passed to `.get()`, not just `&MyKey`
impl Borrow<[u8; 6]> for MyKey {
    fn borrow(&self) -> &[u8; 6] {
        &self.0
    }
}

impl MyKey {
    pub const fn const_cmp(&self, rhs: &Self) -> Ordering {
        let mut i = 0;
        // no for-loops allowed in const fns
        while i < 6 {
            if self.0[i] < rhs.0[i] {
                return Ordering::Less;
            } else if self.0[i] > rhs.0[i] {
                return Ordering::Greater;
            }
            i += 1;
        }
        Ordering::Equal
    }
}

impl_const_sorted_lut!(MyKey);

pub const LUT: ConstSortedLut<u64, 3> = ConstSortedLut::new([
    LutEntry { key: MyKey([2u8; 6]), value: 1},
    LutEntry { key: MyKey([1u8; 6]), value: 2},
    LutEntry { key: MyKey([3u8; 6]), value: 3},
]);
```

## Usage

The `const` context results in some pretty rough constraints:

- `const` trait fns are not stable
- `Ord` methods are not `const`
- only `Copy` types can be safely moved and assigned in `const fn`s

To get around this, you must define the key (new)type with a method `pub const fn const_cmp(&self, other: &Self) -> Ordering`.

The key type must impl `Copy`.

You can then call `impl_const_sorted_lut!(MyNewType)`. This expands to the following code:

```rust ignore
// Note that you can only call impl_const_sorted_lut!() at most once
// in a mod since it defines these 2 structs below:

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstSortedLut<V, const N: usize> {
    keys: [MyNewType; N],
    values: [V; N],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LutEntry<V> {
    pub key: MyNewType,
    pub value: V,
}

impl<V: Copy, const N: usize> ConstSortedLut<V, N> {
    pub const fn new(mut entries: [LutEntry<V>; N]) -> Self {
        // ...
        // compile-time sorting and creation code here
        // ...
    }
}
```

You can then initialize LUTs with `ConstSortedLut::new()`.

No generics or lifetimes are allowed in the key type.

## Features

The following feature-flags implement `ConstSortedLut` for the corresponding primitive type in a `const_<primitive_type>` module:

- `str` - for `&'static str`
- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`

The newtypes are all simple newtypes named `ConstCmp`.

No features are enabled by default.

## Details

- the sorting uses bubble-sort due to the restrictions on `const fn`s. This will result in long compile times for large LUTs that are not already sorted.
- because `as_mut_ptr()` is not `const` in stable yet, `ConstSortedLut::new()` uses the following UB: `unsafe { core::mem::MaybeUninit::uninit().assume_init() }` to achieve a struct-of-arrays layout

## Testing

```sh
cargo test --features all
```
