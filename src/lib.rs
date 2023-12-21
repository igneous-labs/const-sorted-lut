#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(feature = "char")]
#[cfg_attr(docsrs, doc(cfg(feature = "char")))]
pub mod const_char;

#[cfg(feature = "str")]
#[cfg_attr(docsrs, doc(cfg(feature = "str")))]
pub mod const_str;

#[cfg(feature = "u8")]
#[cfg_attr(docsrs, doc(cfg(feature = "u8")))]
pub mod const_u8;

#[cfg(feature = "u16")]
#[cfg_attr(docsrs, doc(cfg(feature = "u16")))]
pub mod const_u16;

#[cfg(feature = "u32")]
#[cfg_attr(docsrs, doc(cfg(feature = "u32")))]
pub mod const_u32;

#[cfg(feature = "u64")]
#[cfg_attr(docsrs, doc(cfg(feature = "u64")))]
pub mod const_u64;

#[cfg(feature = "usize")]
#[cfg_attr(docsrs, doc(cfg(feature = "usize")))]
pub mod const_usize;

#[cfg(feature = "u128")]
#[cfg_attr(docsrs, doc(cfg(feature = "u128")))]
pub mod const_u128;

#[cfg(feature = "i8")]
#[cfg_attr(docsrs, doc(cfg(feature = "i8")))]
pub mod const_i8;

#[cfg(feature = "i16")]
#[cfg_attr(docsrs, doc(cfg(feature = "i16")))]
pub mod const_i16;

#[cfg(feature = "i32")]
#[cfg_attr(docsrs, doc(cfg(feature = "i32")))]
pub mod const_i32;

#[cfg(feature = "i64")]
#[cfg_attr(docsrs, doc(cfg(feature = "i64")))]
pub mod const_i64;

#[cfg(feature = "isize")]
#[cfg_attr(docsrs, doc(cfg(feature = "isize")))]
pub mod const_isize;

#[cfg(feature = "i128")]
#[cfg_attr(docsrs, doc(cfg(feature = "i128")))]
pub mod const_i128;

#[macro_export]
macro_rules! impl_const_sorted_lut {
    ($k: ty) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct ConstSortedLut<V, const N: usize> {
            keys: [$k; N],
            values: [V; N],
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct LutEntry<V> {
            pub key: $k,
            pub value: V,
        }

        impl<V: Copy, const N: usize> ConstSortedLut<V, N> {
            #[allow(clippy::manual_swap)]
            pub const fn new(mut entries: [LutEntry<V>; N]) -> Self {
                // bubble-sort
                // no for-loops allowed in const fns
                let mut i = 0;
                let mut j = 0;
                while i < N - 1 {
                    let mut swapped = false;
                    while j < N - i - 1 {
                        match entries[j].key.const_cmp(&entries[j + 1].key) {
                            core::cmp::Ordering::Equal => panic!("duplicate entries found"),
                            core::cmp::Ordering::Greater => {
                                // swap() isnt a const fn so have to do it manually
                                let temp = entries[j + 1];
                                entries[j + 1] = entries[j];
                                entries[j] = temp;

                                swapped = true;
                            }
                            core::cmp::Ordering::Less => (),
                        }
                        j += 1;
                    }
                    if !swapped {
                        break;
                    }
                    i += 1;
                    j = 0;
                }

                // as_mut_ptr() is not const in stable yet so we're gonna sprinkle
                // a little bit of UB here to get muh SoAs layout...
                let mut res: Self = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

                let mut i = 0;
                while i < N {
                    res.keys[i] = entries[i].key;
                    res.values[i] = entries[i].value;
                    i += 1;
                }
                res
            }

            pub fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V>
            where
                $k: core::borrow::Borrow<Q>,
            {
                use core::borrow::Borrow;

                let i = self.keys.binary_search_by(|p| p.borrow().cmp(key)).ok()?;
                Some(&self.values[i])
            }
        }
    };
}

#[allow(unused_macros)] // but this is used - cargo check bug?
macro_rules! impl_prim_newtype_const_sorted_lut {
    ($primtype: ty) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct ConstCmp(pub $primtype);

        impl ConstCmp {
            pub const fn const_cmp(&self, rhs: &Self) -> core::cmp::Ordering {
                if self.0 == rhs.0 {
                    core::cmp::Ordering::Equal
                } else if self.0 > rhs.0 {
                    core::cmp::Ordering::Greater
                } else {
                    core::cmp::Ordering::Less
                }
            }
        }

        impl core::borrow::Borrow<$primtype> for ConstCmp {
            fn borrow(&self) -> &$primtype {
                &self.0
            }
        }

        crate::impl_const_sorted_lut!(ConstCmp);
    };
}

#[allow(unused_imports)] // but this is used - cargo check bug?
pub(crate) use impl_prim_newtype_const_sorted_lut;
