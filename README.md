# const-sorted-lut

Compile-time sorted `const` binary-search lookup tables.

## Example

The following example shows how to use a 6-byte array as a key type via a newtype.

```rust
use const_sorted_lut::impl_const_sorted_lut;
use core::{borrow::Borrow, cmp::Ordering};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MyKey(pub [u8; 6]);

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
    LutEntry { key: MyKey([2; 6]), value: 1u64},
    LutEntry { key: MyKey([1; 6]), value: 2u64},
    LutEntry { key: MyKey([3; 6]), value: 3u64},
]);

// This allows `&[u8; 6]` to be passed to `.get()`, not just `&MyKey`
impl Borrow<[u8; 6]> for MyKey {
    fn borrow(&self) -> &[u8; 6] {
        &self.0
    }
}

// This allows `&[u8]` to be passed to `.get()`, not just `&MyKey`
impl Borrow<[u8]> for MyKey {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

assert_eq!(*LUT.get(&[2; 6]).unwrap(), 1);
assert_eq!(*LUT.get_const_cmp(&MyKey([1; 6])).unwrap(), 2);
assert!(LUT.get(&[4; 6]).is_none());
```

## Usage

The `const` context results in some pretty rough constraints:

- `const` trait fns are not stable
- `Ord` methods are not `const`
- only `Copy` types can be safely moved and assigned in `const fn`s

To get around these, you must define the key (new)type with a method `pub const fn const_cmp(&self, other: &Self) -> Ordering`.

The key type must impl `Copy`, and no generics or lifetimes are allowed in the key type.

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
        // generated compile-time sorting and creation code
        // ...
    }

    pub fn get_const_cmp(&self, key: &$k) -> Option<&V> {
        let i = self.keys.binary_search_by(|p| p.const_cmp(key)).ok()?;
        Some(&self.values[i])
    }

    pub fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V>
    where
        MyNewType: core::borrow::Borrow<Q>,
    {
        use core::borrow::Borrow;
        let i = self.keys.binary_search_by(|p| p.borrow().cmp(key)).ok()?;
        Some(&self.values[i])
    }
}
```

You can then create LUTs with `ConstSortedLut::new()`.

## Features

The following feature-flags implement `ConstSortedLut` for the corresponding primitive type in the respective `const_cmp_<primitive_type>` module:

- `str` - for `&'static str`
- `char`
- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `u128`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `i128`

The key newtypes are all simple newtypes named `ConstCmp`.

No features are enabled by default.

## Details

- the sorting uses bubble-sort due to the restrictions on `const fn`s. This will result in long compile times for large LUTs that are not already sorted, but shorter times for LUTs that are already hand-sorted
- because `as_mut_ptr()` is not `const` in stable yet, `ConstSortedLut::new()` uses the following UB: `unsafe { core::mem::MaybeUninit::uninit().assume_init() }` to achieve a struct-of-arrays layout

## Testing

```sh
cargo test --features all
```
