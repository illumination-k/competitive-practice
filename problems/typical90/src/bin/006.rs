#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use num::*;
use num_traits::*;
use proconio::marker::*;
use proconio::{fastout, input};
use std::collections::*;
use std::ops::*;
use superslice::*;
use whiteread::parse_line;

use itertools::iproduct;
use itertools::Itertools;
use itertools_num::ItertoolsNum;

use competitive_internal_mod::format::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    const MOD: usize = 1_000_000_007;
    const UINF: usize = std::usize::MAX;
    const IINF: isize = std::isize::MAX;

    input! {
        n: usize, k: usize,
        s: Chars,
    }

    /*
    部分列に関するDP？ -> 数え上げじゃないから違いそう
    - https://qiita.com/drken/items/a207e5ae3ea2cf17f4bd
    naiveにやるとx桁目を決めるときに右からx-1個残したところの中で辞書順最小をとる、をk回繰り返せばできるが、TLEしそう
    何らかの前計算がいる？
    最小値がわかればいいので、RMQすればよさそう
    */

    // (index, val)
    let s_int = s
        .iter()
        .enumerate()
        .map(|(i, &c)| (c as usize, i))
        .collect_vec();

    let st: SegmentTree<Min<(usize, usize)>> = SegmentTree::from_slice(&s_int);

    let mut ans = vec![];
    let mut start = 0;
    for i in 0..k {
        debug!(ans);
        let end = n - (k - i);
        debug!(start, end);
        debug!(s[start..=end]);
        // 少なくともk-i+1桁残す必要がある
        let c = st.query(start..=end).0;
        ans.push(c.0 as u8 as char);
        start = c.1 + 1;
    }

    ans.iter().collect::<String>()
}

fn main() {
    println!("{}", solve().format());
}

pub trait Monoid: Sized {
    /// 単位元
    fn mempty() -> Self;

    /// op
    fn mappend(l: &Self, r: &Self) -> Self;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Min<T>(pub T);

impl<T: Copy + Ord + Bounded> Monoid for Min<T> {
    fn mempty() -> Self {
        Self(<T as Bounded>::max_value())
    }

    fn mappend(l: &Self, r: &Self) -> Self {
        Self(l.0.min(r.0))
    }
}
impl<T> From<T> for Min<T> {
    fn from(v: T) -> Self {
        Min(v)
    }
}
#[derive(Debug)]
pub struct SegmentTree<T> {
    len: usize,
    v: Vec<T>,
}

impl<T: Clone + Monoid> SegmentTree<T> {
    /// O(n).
    /// Construct segment tree for given size.
    pub fn new(n: usize) -> Self {
        let s: &[T] = &[];
        Self::init(n, s)
    }

    /// O(n).
    /// Construct segment tree from slice.
    pub fn from_slice(s: &[impl Into<T> + Clone]) -> Self {
        Self::init(s.len(), s)
    }

    fn init(len: usize, s: &[impl Into<T> + Clone]) -> Self {
        let n = len.next_power_of_two();
        let mut v = vec![T::mempty(); n * 2 - 1];
        for i in 0..s.len() {
            v[n - 1 + i] = s[i].clone().into();
        }

        let mut l = n / 2;
        let mut ofs = n - 1 - l;

        while l > 0 {
            for i in 0..l {
                let ix = ofs + i;
                v[ix] = T::mappend(&v[ix * 2 + 1], &v[ix * 2 + 2]);
            }
            l /= 2;
            ofs -= l;
        }

        Self { len, v }
    }

    /// O(1).
    /// Length of sequence.
    pub fn len(&self) -> usize {
        self.len
    }

    /// O(log n).
    /// Set v to `i`-th element.
    /// `s[i] = v`
    pub fn set(&mut self, i: usize, v: impl Into<T>) {
        let n = (self.v.len() + 1) / 2;
        let mut cur = n - 1 + i;
        self.v[cur] = v.into();
        while cur > 0 {
            cur = (cur - 1) / 2;
            self.v[cur] = T::mappend(&self.v[cur * 2 + 1], &self.v[cur * 2 + 2]);
        }
    }

    /// O(log n).
    /// mappend v to `i`-th element
    /// `s[i] = mappend(s[i], v)`
    pub fn mappend(&mut self, i: usize, v: impl Into<T>) {
        self.set(i, T::mappend(&self.get(i), &v.into()));
    }

    /// O(1).
    /// Get i-th element
    /// Equals to `query(i, i + 1)`
    pub fn get(&self, i: usize) -> T {
        let n = (self.v.len() + 1) / 2;
        self.v[n - 1 + i].clone()
    }

    /// O(log n).
    /// Query for `range`.
    /// Returns `T::mconcat(&s[range])`.
    ///
    /// # Examples
    ///
    /// ```
    /// use competitive::data_structures::monoid::Sum;
    /// use competitive::data_structures::segment_tree::SegmentTree;
    ///
    /// let mut st = SegmentTree::<Sum<i64>>::new(5);
    /// st.set(2, 3);
    /// assert_eq!(st.query(0..=2).0, 3);
    /// assert_eq!(st.query(0..2).0, 0);
    /// ```
    ///
    pub fn query(&self, range: impl RangeBounds<usize>) -> T {
        let l = match range.start_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => v + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(v) => v + 1,
            Bound::Excluded(v) => *v,
            Bound::Unbounded => self.len,
        };

        assert!(l <= r);
        assert!(r <= self.len);

        let n = (self.v.len() + 1) / 2;
        let mut l = n + l;
        let mut r = n + r;

        let mut ret_l = T::mempty();
        let mut ret_r = T::mempty();
        while l < r {
            if l & 1 != 0 {
                ret_l = T::mappend(&ret_l, &self.v[l - 1]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                ret_r = T::mappend(&self.v[r - 1], &ret_r);
            }
            l /= 2;
            r /= 2;
        }

        T::mappend(&ret_l, &ret_r)
    }
}

mod competitive_internal_mod {
    pub mod format {
        use std::vec::Vec;

        /// Trait of format for atcoder
        ///    
        /// bool -> Yes or No  
        /// vec![a, b ,c] -> "a\nb\nc"  
        /// vec![vec![0, 1], vec![1, 0]] -> "0 1\n1 0"  
        pub trait AtCoderFormat {
            fn format(&self) -> String;
        }

        macro_rules! impl_format {
            ($t: ty) => {
                impl AtCoderFormat for $t {
                    fn format(&self) -> String {
                        self.to_string()
                    }
                }

                impl AtCoderFormat for Vec<$t> {
                    fn format(&self) -> String {
                        self.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                    }
                }

                impl AtCoderFormat for Vec<Vec<$t>> {
                    fn format(&self) -> String {
                        self.iter()
                            .map(|x| {
                                x.iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()
                                    .join(" ")
                            })
                            .collect::<Vec<String>>()
                            .join("\n")
                    }
                }
            };
        }

        impl_format!(usize);
        impl_format!(u128);
        impl_format!(u64);
        impl_format!(u32);
        impl_format!(u16);
        impl_format!(u8);
        impl_format!(isize);
        impl_format!(i128);
        impl_format!(i64);
        impl_format!(i32);
        impl_format!(i16);
        impl_format!(i8);
        impl_format!(f32);
        impl_format!(f64);
        impl_format!(&str);
        impl_format!(String);
        impl_format!(char);

        impl AtCoderFormat for bool {
            fn format(&self) -> String {
                if self == &true {
                    "Yes".to_string()
                } else {
                    "No".to_string()
                }
            }
        }

        impl AtCoderFormat for Vec<bool> {
            fn format(&self) -> String {
                self.iter()
                    .map(|x| x.format())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        }
    }
}
