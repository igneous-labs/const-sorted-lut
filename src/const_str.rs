use core::{borrow::Borrow, cmp::Ordering};

use crate::impl_const_sorted_lut;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConstCmp(pub &'static str);

impl ConstCmp {
    pub const fn const_cmp(&self, rhs: &Self) -> Ordering {
        let mut i = 0;
        let l = self.0.as_bytes();
        let r = rhs.0.as_bytes();
        while i < l.len() {
            if i >= r.len() {
                return Ordering::Greater;
            }
            if l[i] < r[i] {
                return Ordering::Less;
            } else if l[i] > r[i] {
                return Ordering::Greater;
            }
            i += 1;
        }
        Ordering::Equal
    }
}

impl Borrow<str> for ConstCmp {
    fn borrow(&self) -> &str {
        self.0
    }
}

impl_const_sorted_lut!(ConstCmp);
