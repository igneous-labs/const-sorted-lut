use const_sorted_lut::const_str::{ConstCmp, ConstSortedLut, LutEntry};

#[test]
fn u8_val() {
    const K0: &str = "d";
    const K1: &str = "abc";
    const K2: &str = "~!";
    const K3: &str = "efghijk";
    const V0: u8 = 0;
    const V1: u8 = 1;
    const V2: u8 = 2;
    const V3: u8 = 3;

    const LUT: ConstSortedLut<u8, 4> = ConstSortedLut::new([
        LutEntry {
            key: ConstCmp(K0),
            value: V0,
        },
        LutEntry {
            key: ConstCmp(K1),
            value: V1,
        },
        LutEntry {
            key: ConstCmp(K2),
            value: V2,
        },
        LutEntry {
            key: ConstCmp(K3),
            value: V3,
        },
    ]);

    for (k, v) in [(K0, V0), (K1, V1), (K2, V2), (K3, V3)] {
        assert_eq!(*LUT.get(k).unwrap(), v);
    }
}

fn f0(_a: &str) -> u8 {
    0
}

fn f1(_a: &str) -> u8 {
    1
}

fn f2(_a: &str) -> u8 {
    2
}

#[test]
fn fn_pointer_val() {
    const K0: &str = "sfdsaf3eqrfefrett43rtegetgtrgtry";
    const K1: &str = "sfdsaf3eqrfefrett43rtegetgtrgtrz";
    const K2: &str = "a";

    const LUT: ConstSortedLut<fn(&str) -> u8, 3> = ConstSortedLut::new([
        LutEntry {
            key: ConstCmp(K0),
            value: f0,
        },
        LutEntry {
            key: ConstCmp(K1),
            value: f1,
        },
        LutEntry {
            key: ConstCmp(K2),
            value: f2,
        },
    ]);

    // str arg not used by function
    assert_eq!(LUT.get(K0).unwrap()(K2), f0(K2));
    assert_eq!(LUT.get(K1).unwrap()(K2), f1(K2));
    assert_eq!(LUT.get(K2).unwrap()(K2), f2(K2));
}
