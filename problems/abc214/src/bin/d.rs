#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive_internal_mod::data_structures::union_find::*;
use competitive_internal_mod::format::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*, source::auto::AutoSource};
use std::{
    collections::*,
    io::{BufRead, BufReader},
    ops::*,
};
use superslice::*;
use utils::*;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

#[fastout]
fn run<R: BufRead>(source: AutoSource<R>) -> impl AtCoderFormat {
    input! {
        from source,
        n: usize,
        uvw: [(usize, usize, usize); n-1]
    }

    let uvw = uvw
        .iter()
        .cloned()
        .sorted_by(|a, b| a.2.cmp(&b.2))
        .map(|(u, v, w)| (u - 1, v - 1, w))
        .collect_vec();
    debug!(uvw);

    let mut un = UnionFind::new(n);
    let mut ans = 0;
    for &(u, v, w) in uvw.iter() {
        ans += un.size(u) * un.size(v) * w;
        un.union(u, v);
    }

    ans
}

fn main() {
    println!(
        "{}",
        run(AutoSource::new(BufReader::new(std::io::stdin()))).format()
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use competitive::test_utility::*;

    #[test]
    fn test_basic() {
        let input = "
        ";
        println!("ans: {}", run(AutoSource::from(input)).format());
    }
}

pub mod utils {
    macro_rules! debug {
        ($($a:expr),* $(,)*) => {
            #[cfg(debug_assertions)]
            eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
        };
    }

    macro_rules! chmin {
        ($base:expr, $($cmps:expr),+ $(,)*) => {{
            let cmp_min = min!($($cmps),+);
            if $base > cmp_min {
                $base = cmp_min;
                true;
            } else {
                false;
            }
        }};
    }

    macro_rules! chmax {
        ($base:expr, $($cmps:expr),+ $(,)*) => {{
            let cmp_max = max!($($cmps),+);
            if $base < cmp_max {
                $base = cmp_max;
                true;
            } else {
                false;
            }
        }};
    }

    macro_rules! min {
        ($a:expr $(,)*) => {{
            $a
        }};
        ($a:expr, $b:expr $(,)*) => {{
            std::cmp::min($a, $b)
        }};
        ($a:expr, $($rest:expr),+ $(,)*) => {{
            std::cmp::min($a, min!($($rest),+))
        }};
    }

    macro_rules! max {
        ($a:expr $(,)*) => {{
            $a
        }};
        ($a:expr, $b:expr $(,)*) => {{
            std::cmp::max($a, $b)
        }};
        ($a:expr, $($rest:expr),+ $(,)*) => {{
            std::cmp::max($a, max!($($rest),+))
        }};
    }

    pub(crate) use chmax;
    pub(crate) use chmin;
    pub(crate) use debug;
    pub(crate) use max;
    pub(crate) use min;
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

        impl AtCoderFormat for char {
            fn format(&self) -> String {
                self.to_string()
            }
        }

        impl AtCoderFormat for Vec<char> {
            fn format(&self) -> String {
                self.iter().collect::<String>()
            }
        }

        impl AtCoderFormat for Vec<Vec<char>> {
            fn format(&self) -> String {
                self.iter()
                    .map(|v| v.format())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        }

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
    pub mod data_structures {
        pub mod union_find {
            // from petgraph to customize UnionFind. rank -> size

            use num_traits::PrimInt;
            #[derive(Debug, Clone)]
            pub struct UnionFind<K> {
                // For element at index *i*, store the index of its parent; the representative itself
                // stores its own index. This forms equivalence classes which are the disjoint sets, each
                // with a unique representative.
                parent: Vec<K>,

                // size vector
                size: Vec<usize>,
                // group_num
                group_num: usize,
            }

            #[inline]
            unsafe fn get_unchecked<K>(xs: &[K], index: usize) -> &K {
                debug_assert!(index < xs.len());
                xs.get_unchecked(index)
            }

            #[inline]
            unsafe fn get_unchecked_mut<K>(xs: &mut [K], index: usize) -> &mut K {
                debug_assert!(index < xs.len());
                xs.get_unchecked_mut(index)
            }

            impl<K> UnionFind<K>
            where
                K: PrimInt + std::hash::Hash,
            {
                /// Create a new `UnionFind` of `n` disjoint sets.
                pub fn new(n: usize) -> Self {
                    let size = vec![1; n];
                    let parent = (0..n).map(|x| K::from(x).unwrap()).collect::<Vec<K>>();
                    let group_num = n;
                    Self {
                        parent,
                        size,
                        group_num,
                    }
                }

                /// Return the representative for `x`.
                ///
                /// **Panics** if `x` is out of bounds.
                pub fn find(&self, x: K) -> K {
                    assert!(x.to_usize().unwrap() < self.parent.len());
                    unsafe {
                        let mut x = x;
                        loop {
                            // Use unchecked indexing because we can trust the internal set ids.
                            let xparent = *get_unchecked(&self.parent, x.to_usize().unwrap());
                            if xparent == x {
                                break;
                            }
                            x = xparent;
                        }
                        x
                    }
                }

                /// Return the representative for `x`.
                ///
                /// Write back the found representative, flattening the internal
                /// datastructure in the process and quicken future lookups.
                ///
                /// **Panics** if `x` is out of bounds.
                pub fn find_mut(&mut self, x: K) -> K {
                    assert!(x.to_usize().unwrap() < self.parent.len());
                    unsafe { self.find_mut_recursive(x) }
                }

                unsafe fn find_mut_recursive(&mut self, mut x: K) -> K {
                    let mut parent = *get_unchecked(&self.parent, x.to_usize().unwrap());
                    while parent != x {
                        let grandparent = *get_unchecked(
                            &self.parent,
                            parent
                                .to_usize()
                                .expect("Cannot convert to usize. maybe negative number!"),
                        );
                        *get_unchecked_mut(
                            &mut self.parent,
                            x.to_usize()
                                .expect("Cannot convert to usize. maybe negative number!"),
                        ) = grandparent;
                        x = parent;
                        parent = grandparent;
                    }
                    x
                }

                /// Returns `true` if the given elements belong to the same set, and returns
                /// `false` otherwise.
                pub fn equiv(&self, x: K, y: K) -> bool {
                    self.find(x) == self.find(y)
                }

                /// Unify the two sets containing `x` and `y`.
                ///
                /// Return `false` if the sets were already the same, `true` if they were unified.
                ///
                /// **Panics** if `x` or `y` is out of bounds.
                pub fn union(&mut self, x: K, y: K) -> bool {
                    if x == y {
                        return false;
                    }
                    let xrep = self.find_mut(x);
                    let yrep = self.find_mut(y);

                    if xrep == yrep {
                        return false;
                    }

                    let xrepu = xrep
                        .to_usize()
                        .expect("Cannot convert to usize. maybe negative number!");
                    let yrepu = yrep
                        .to_usize()
                        .expect("Cannot convert to usize. maybe negative number!");
                    let xsize = self.size[xrepu];
                    let ysize = self.size[yrepu];

                    // The rank corresponds roughly to the depth of the treeset, so put the
                    // smaller set below the larger
                    if xsize > ysize {
                        self.parent[yrepu] = xrep;
                        self.size[xrepu] += ysize;
                    } else {
                        self.parent[xrepu] = yrep;
                        self.size[yrepu] += xsize;
                    }
                    self.group_num -= 1;
                    true
                }

                /// Return a vector mapping each element to its representative.
                pub fn into_labeling(mut self) -> Vec<K> {
                    // write in the labeling of each element
                    unsafe {
                        for ix in 0..self.parent.len() {
                            let k = *get_unchecked(&self.parent, ix);
                            let xrep = self.find_mut_recursive(k);
                            *self.parent.get_unchecked_mut(ix) = xrep;
                        }
                    }
                    self.parent
                }

                /// return group size of x
                pub fn size(&self, x: K) -> usize {
                    let xrep = self.find(x);
                    let xrepu = xrep
                        .to_usize()
                        .expect("Cannot convert to usize. maybe negative number!");

                    self.size[xrepu]
                }

                /// get group numbers
                pub fn get_group_number(&self) -> usize {
                    self.group_num
                }
            }
        }
    }
}
