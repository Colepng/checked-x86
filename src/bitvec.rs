#![allow(clippy::derived_hash_with_manual_eq)]
#![allow(warnings)]
use core::{
    cmp::Ordering,
    ops::{Add, BitAnd, BitOr, Not, Rem, Shl, Shr, Sub},
};
use flux_rs::opaque;
use flux_rs::refined_by;
use flux_rs::sig;
use flux_rs::trusted;
use paste::paste;

macro_rules! bitvec_impl {
    ($width:literal) => {
        paste! {
            #[derive(Debug, Clone, Copy, Hash)]
            #[opaque]
            #[refined_by(x: bitvec<$width>)]
            #[repr(transparent)]
            pub struct [<BV $width>]([<u $width>]);

            #[trusted]
            impl PartialOrd for [<BV $width>] {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    self.0.partial_cmp(&other.0)
                }

                #[sig(fn(&[<BV $width>][@x], &[<BV $width>][@y]) -> bool[bv_ule(x, y)])]
                fn le(&self, other: &Self) -> bool {
                    self.0 <= other.0
                }

                #[sig(fn(&[<BV $width>][@x], &[<BV $width>][@y]) -> bool[bv_ult(x, y)])]
                fn lt(&self, other: &Self) -> bool {
                    self.0 < other.0
                }

                #[sig(fn(&[<BV $width>][@x], &[<BV $width>][@y]) -> bool[bv_uge(x, y)])]
                fn ge(&self, other: &Self) -> bool {
                    self.0 >= other.0
                }

                #[sig(fn(&[<BV $width>][@x], &[<BV $width>][@y]) -> bool[bv_ugt(x, y)])]
                fn gt(&self, other: &Self) -> bool {
                    self.0 > other.0
                }
            }

            #[trusted]
            impl [<BV $width>] {
                #[sig(fn ([<u $width>][@val]) -> [<BV $width>][[<bv_int_to_bv $width>](val)])]
                pub const fn new(value: [<u $width>]) -> [<BV $width>] {
                    [<BV $width>](value)
                }

                #[sig(fn([<BV $width>][@x], [<BV $width>][@y]) -> [<BV $width>][bv_add(x, y)])]
                pub fn wrapping_add(self, other: [<BV $width>]) -> [<BV $width>] {
                    [<BV $width>](self.0.wrapping_add(other.0))
                }
            }

            impl From<[<u $width>]> for [<BV $width>] {
                #[trusted]
                #[sig(fn([<u $width>][@val]) -> [<BV $width>][[<bv_int_to_bv $width>](val)])]
                fn from(value: [<u $width>]) -> [<BV $width>] {
                    [<BV $width>](value)
                }
            }

            impl Into<[<u $width>]> for [<BV $width>] {
                #[trusted]
                #[sig(fn([<BV $width>][@val]) -> [<u $width>][[<bv_bv $width _to_int>](val)])]
                fn into(self) -> [<u $width>] {
                    self.0
                }
            }

            impl Not for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@x]) -> [<BV $width>][bv_not(x)])]
                fn not(self) -> [<BV $width>] {
                    [<BV $width>](!self.0)
                }
            }

            impl BitAnd for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@x], [<BV $width>][@y]) -> [<BV $width>][bv_and(x, y)])]
                fn bitand(self, rhs: Self) -> [<BV $width>] {
                    [<BV $width>](self.0 & rhs.0)
                }
            }

            impl BitAnd<[<u $width>]> for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@x], [<u $width>][@y]) -> [<BV $width>][bv_and(x, [<bv_int_to_bv $width>](y))])]
                fn bitand(self, rhs: [<u $width>]) -> [<BV $width>] {
                    [<BV $width>](self.0 & rhs)
                }
            }

            impl BitOr for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@x], [<BV $width>][@y]) -> [<BV $width>][bv_or(x, y)])]
                fn bitor(self, rhs: Self) -> [<BV $width>] {
                    [<BV $width>](self.0 | rhs.0)
                }
            }

            impl BitOr<[<u $width>]> for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@x], [<u $width>][@y]) -> [<BV $width>][bv_or(x, [<bv_int_to_bv $width>](y))])]
                fn bitor(self, rhs: [<u $width>]) -> [<BV $width>] {
                    [<BV $width>](self.0 | rhs)
                }
            }

            impl Shl for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@x], [<BV $width>][@y]) -> [<BV $width>][bv_shl(x, y)])]
                fn shl(self, rhs: Self) -> [<BV $width>] {
                    [<BV $width>](self.0 << rhs.0)
                }
            }

            impl Shl<[<u $width>]> for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@x], [<u $width>][@y]) -> [<BV $width>][bv_shl(x, [<bv_int_to_bv $width>](y))])]
                fn shl(self, rhs: [<u $width>]) -> [<BV $width>] {
                    [<BV $width>](self.0 << rhs)
                }
            }

            impl Shr for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@x], [<BV $width>][@y]) -> [<BV $width>][bv_lshr(x, y)])]
                fn shr(self, rhs: Self) -> [<BV $width>] {
                    [<BV $width>](self.0 >> rhs.0)
                }
            }

            impl Shr<[<u $width>]> for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@x], [<u $width>][@y]) -> [<BV $width>][bv_lshr(x, [<bv_int_to_bv $width>](y))])]
                fn shr(self, rhs: [<u $width>]) -> [<BV $width>] {
                    [<BV $width>](self.0 >> rhs)
                }
            }

            impl Add for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@val1], [<BV $width>][@val2]) -> [<BV $width>][bv_add(val1, val2)])]
                fn add(self, rhs: Self) -> [<BV $width>] {
                    [<BV $width>](self.0 + rhs.0)
                }
            }

            impl Sub for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@val1], [<BV $width>][@val2]) -> [<BV $width>][bv_sub(val1, val2)])]
                fn sub(self, rhs: Self) -> [<BV $width>] {
                    [<BV $width>](self.0.wrapping_add(!rhs.0))
                }
            }

            impl Rem for [<BV $width>] {
                type Output = [<BV $width>];

                #[trusted]
                #[sig(fn([<BV $width>][@val1], [<BV $width>][@val2]) -> [<BV $width>][bv_urem(val1, val2)])]
                fn rem(self, rhs: Self) -> [<BV $width>] {
                    [<BV $width>](self.0 & rhs.0)
                }
            }

            #[trusted]
            impl PartialEq for [<BV $width>] {
                #[sig(fn(&[<BV $width>][@val1], &[<BV $width>][@val2]) -> bool[val1 == val2])]
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }

                #[sig(fn(&[<BV $width>][@val1], &[<BV $width>][@val2]) -> bool[val1 != val2])]
                fn ne(&self, other: &Self) -> bool {
                    self.0 != other.0
                }
            }

            #[trusted]
            impl PartialEq<[<u $width>]> for [<BV $width>] {
                #[sig(fn(&[<BV $width>][@val1], &[<u $width>][@val2]) -> bool[val1 == [<bv_int_to_bv $width>](val2)])]
                fn eq(&self, other: &[<u $width>]) -> bool {
                    self.0 == *other
                }

                #[sig(fn(&[<BV $width>][@val1], &[<u $width>][@val2]) -> bool[val1 != [<bv_int_to_bv $width>](val2)])]
                fn ne(&self, other: &[<u $width>]) -> bool {
                    self.0 != *other
                }
            }
        }
    };
}

bitvec_impl!(8);
bitvec_impl!(16);
bitvec_impl!(32);
bitvec_impl!(64);
