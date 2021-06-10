#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use num::*;
use num_traits::*;
use petgraph::graph::UnGraph;
use proconio::marker::*;
use proconio::{fastout, input};
use std::collections::*;
use std::ops::*;
use superslice::*;
use whiteread::parse_line;

use itertools::iproduct;
use itertools::Itertools;
use itertools_num::ItertoolsNum;

use competitive_internal_mod::data_structures::union_find::*;
use competitive_internal_mod::format::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

/*
A_xi + A_yi + Ziが偶数

Z_iが偶数なら、A_xi, A_yiは同じ数
Z_iが奇数なら、A_xi, A_yiは違う数

- 同じやつはグループ分けすれば一発で全部わかる。
- 違うやつグループもグループに魔法で一発。
- ヒントがないものは必ず魔法を使う必要がある。

1. 同じグループのものを同じラベルにする(UnionFind)。
2. 違うグループのものをそのラベルでグループ分けする(UnionFind)。
3. 2のグループ数と登場していないものの数が答え

正しいが、回答を見るとそもそも一回UnionFindすればいいらしい...
*/

fn get_cards_set(xyz: &Vec<(usize, usize, usize)>) -> HashSet<usize> {
    let mut set = HashSet::new();

    for &(x, y, _) in xyz {
        set.insert(x - 1);
        set.insert(y - 1);
    }

    set
}

fn get_same_group_labels(n: usize, same: &Vec<(usize, usize, usize)>) -> Vec<usize> {
    let mut un: UnionFind<usize> = UnionFind::new(n);

    for &(x, y, _) in same.iter() {
        un.union(x - 1, y - 1);
    }

    un.into_labeling()
}

fn make_n_map(s: &HashSet<(usize, usize)>) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    let mut cnt = 0;
    for &(x, y) in s.iter() {
        if !map.contains_key(&x) {
            map.insert(x, cnt);
            cnt += 1;
        }

        if !map.contains_key(&y) {
            map.insert(y, cnt);
            cnt += 1;
        }
    }
    map
}

fn get_union_number(n: usize, xyz: &Vec<(usize, usize, usize)>) -> usize {
    let mut un: UnionFind<usize> = UnionFind::new(n);

    for &(x, y, _) in xyz.iter() {
        un.union(x - 1, y - 1);
    }

    un.group_numbers()
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    const MOD: usize = 1_000_000_007;
    const UINF: usize = std::usize::MAX;
    const IINF: isize = std::isize::MAX;

    input! {
        n: usize, m: usize,
        xyz: [(usize, usize, usize); m]
    }

    let mut ans = 0;
    ans += n - get_cards_set(&xyz).len();

    let mut same = vec![];
    let mut diff = vec![];
    for &(x, y, z) in xyz.iter() {
        if z % 2 == 0 {
            same.push((x, y, z))
        } else {
            diff.push((x, y, z));
        }
    }

    let same_group_label = get_same_group_labels(n, &same);
    let mut n_xy = HashSet::new();
    for &(x, y, _) in xyz.iter() {
        n_xy.insert((same_group_label[x - 1], same_group_label[y - 1]));
    }
    debug!(same_group_label);
    debug!(n_xy);
    let nmap = make_n_map(&n_xy);
    let mut nnxy = HashSet::new();

    for &(x, y) in n_xy.iter() {
        nnxy.insert((nmap.get(&x).unwrap(), nmap.get(&y).unwrap()));
    }
    debug!(nmap);
    debug!(nnxy);

    let mut un: UnionFind<usize> = UnionFind::new(nmap.len());

    for &(x, y) in nnxy.iter() {
        un.union(*x, *y);
    }
    debug!(un);
    ans += un.group_numbers();

    get_union_number(n, xyz)
}

fn main() {
    println!("{}", solve().format());
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
    pub mod data_structures {
        pub mod union_find {
            // from petgraph to customize UnionFind. rank -> size

            use num_traits::PrimInt;
            use std::collections::*;
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

                pub fn size(&self, x: K) -> usize {
                    let xrep = self.find(x);
                    let xrepu = xrep
                        .to_usize()
                        .expect("Cannot convert to usize. maybe negative number!");

                    self.size[xrepu]
                }

                pub fn group_numbers(&self) -> usize {
                    self.group_num
                }

                pub fn member(&self, x: K) -> HashSet<K> {
                    // O(n)
                    let xrep = self.find(x);
                    let mut set: HashSet<K> = HashSet::new();

                    for i in 0..self.parent.len() {
                        let i_k = K::from(i).unwrap();
                        if self.find(i_k) == xrep {
                            set.insert(i_k);
                        }
                    }

                    set
                }

                pub fn member_map(&self) -> HashMap<K, HashSet<K>> {
                    // O(n^2)
                    let mut map: HashMap<K, HashSet<K>> = HashMap::new();
                    for i in 0..self.parent.len() {
                        map.entry(K::from(i).unwrap())
                            .or_insert(self.member(K::from(i).unwrap()));
                    }
                    map
                }
            }
        }
    }
}
