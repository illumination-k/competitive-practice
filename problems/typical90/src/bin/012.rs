#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::{collections::*, ops::*};
use superslice::*;
use whiteread::parse_line;

use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;

use competitive_internal_mod::data_structures::union_find::*;
use competitive_internal_mod::format::*;
use competitive_internal_mod::graph2d::*;
use utils::debug;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

fn get_1dim_idx(two_dim: &(usize, usize), map: &BTreeMap<(usize, usize), usize>) -> usize {
    match map.get(two_dim) {
        Some(x) => *x,
        None => 0,
    }
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    let (h, w): (usize, usize) = parse_line().unwrap();
    let q: usize = parse_line().unwrap();

    let mut queries = vec![];
    for _ in 0..q {
        let query: Vec<usize> = parse_line().unwrap();
        queries.push(query);
    }

    /*
    UnionFindでつながったところを探していけばよさそう
    二次元配列を一次元につなげられるようにする。
    */

    let two2one_dim = iproduct!(0..h, 0..w)
        .enumerate()
        .map(|(i, t)| (t, i))
        .collect::<BTreeMap<(usize, usize), usize>>();

    assert_eq!(two2one_dim.iter().count(), h * w);
    let mut g = Graph2D::new(vec![vec![false; 2009]; 2009]);
    let mut un: UnionFind<usize> = UnionFind::new(h * w);

    let mut ans: Vec<bool> = vec![];

    for query in queries.into_iter() {
        let tt = query[0];

        if tt == 1 {
            // query 1
            let p = (query[1] - 1, query[2] - 1);
            let query_idx = get_1dim_idx(&p, &two2one_dim);
            g[p] = true;

            let directions = [(0, 1), (1, 0), (-1, 0), (0, -1)];
            for &(dx, dy) in directions.iter() {
                let (nx, ny) = (p.0 as isize + dx, p.1 as isize + dy);
                if g.is_not_in(nx, ny) {
                    continue;
                }
                let next_idx = get_1dim_idx(&(nx as usize, ny as usize), &two2one_dim);
                if g[(nx as usize, ny as usize)] {
                    un.union(next_idx, query_idx);
                }
            }
        } else {
            // query 2
            let p1: (usize, usize) = (query[1] - 1, query[2] - 1);
            let p2: (usize, usize) = (query[3] - 1, query[4] - 1);
            let p1_idx = get_1dim_idx(&p1, &two2one_dim);
            let p2_idx = get_1dim_idx(&p2, &two2one_dim);
            if g[p1] && g[p2] {
                if un.equiv(p1_idx, p2_idx) {
                    ans.push(true)
                } else {
                    ans.push(false)
                }
            } else {
                ans.push(false)
            }
        }
    }

    ans
}

fn main() {
    println!("{}", solve().format());
}

pub mod utils {
    #[allow(unused_macros)]
    macro_rules! debug {
        ($($a:expr),* $(,)*) => {
            #[cfg(debug_assertions)]
            eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
        };
    }
    pub(crate) use debug;
}

mod competitive_internal_mod {
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
    pub mod graph2d {
        #[allow(unused_imports)]
        use std::collections::*;
        use std::fmt;
        use std::{ops::*, writeln};

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Graph2D<T> {
            graph: Vec<Vec<T>>,
            width: isize,
            height: isize,
        }

        impl<T> fmt::Debug for Graph2D<T>
        where
            T: ToString + Eq + Copy,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                writeln!(f, "")?;
                writeln!(f, "width = {}, height = {}", self.width, self.height)?;
                for y in 0..self.height() {
                    let s: String = self.graph[y]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                    writeln!(f, "{}", s)?;
                }

                Ok(())
            }
        }

        impl<K> Graph2D<K>
        where
            K: Eq + Copy,
        {
            pub fn new(graph: Vec<Vec<K>>) -> Self {
                let width = graph[0].len() as isize;
                let height = graph.len() as isize;
                Self {
                    graph,
                    width,
                    height,
                }
            }

            pub fn is_not_in(&self, x: isize, y: isize) -> bool {
                x < 0 || y < 0 || x >= self.width || y >= self.height
            }

            pub fn is_in(&self, x: isize, y: isize) -> bool {
                !(self.is_not_in(x, y))
            }

            pub fn is_go(&self, x: isize, y: isize, obs: &Option<K>) -> Option<(usize, usize)> {
                if self.is_not_in(x, y) {
                    return None;
                }
                match obs {
                    Some(obs) => {
                        if &self[(x as usize, y as usize)] == obs {
                            None
                        } else {
                            Some((x as usize, y as usize))
                        }
                    }
                    None => Some((x as usize, y as usize)),
                }
            }

            pub fn width(&self) -> usize {
                self.width as usize
            }

            pub fn height(&self) -> usize {
                self.height as usize
            }

            /// get Iterator of a row
            pub fn row(&self, y: usize) -> std::slice::Iter<K> {
                self.graph[y].iter()
            }

            /// get Iterator of a column
            pub fn col(&self, x: usize) -> impl Iterator<Item = K> + '_ {
                let height = self.height();
                (0..height).map(move |y| self[(x, y)])
            }

            // transpose
            pub fn t(&self) -> Self {
                let mut vec = vec![vec![]; self.width()];

                for i in 0..self.width() {
                    for x in self.col(i) {
                        vec[i].push(x);
                    }
                }
                Graph2D::new(vec)
            }
        }

        impl<K> Index<(usize, usize)> for Graph2D<K> {
            type Output = K;

            fn index<'a>(&'a self, index: (usize, usize)) -> &'a K {
                let (x, y): (usize, usize) = index;

                &self.graph[y][x]
            }
        }

        impl<K> IndexMut<(usize, usize)> for Graph2D<K> {
            fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut K {
                let (x, y): (usize, usize) = index;

                &mut self.graph[y][x]
            }
        }

        /// bfs of 2d graph
        pub fn bfs2d<T: Eq + Copy>(
            graph: &Graph2D<T>,
            start: (usize, usize),
            obs: Option<T>,
        ) -> Graph2D<isize> {
            let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            let mut dist: Graph2D<isize> =
                Graph2D::new(vec![vec![-1; graph.width()]; graph.height()]);
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

            dist[start] = 0;
            queue.push_back(start);

            while let Some((cx, cy)) = queue.pop_front() {
                for direction in directions.iter() {
                    let next_x = cx as isize + direction.0;
                    let next_y = cy as isize + direction.1;

                    match graph.is_go(next_x, next_y, &obs) {
                        Some(next) => {
                            if dist[next] == -1 {
                                dist[next] = dist[(cx, cy)] + 1;
                                queue.push_back(next)
                            }
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }

            dist
        }

        /// dfs of 2d graph
        pub fn dfs2d<T: Eq + Copy>(
            graph: &Graph2D<T>,
            seen: &mut Vec<Vec<bool>>,
            start: (usize, usize),
            obs: Option<T>,
        ) {
            fn dfs<T: Eq + Copy>(
                graph: &Graph2D<T>,
                start: (usize, usize),
                obs: &Option<T>,
                directions: &Vec<(isize, isize)>,
                seen: &mut Vec<Vec<bool>>,
            ) {
                seen[start.1][start.0] = true;

                for &direction in directions.iter() {
                    let next_x = start.0 as isize + direction.0;
                    let next_y = start.1 as isize + direction.1;

                    match graph.is_go(next_x, next_y, obs) {
                        Some(next) => {
                            if !seen[next.1][next.0] {
                                dfs(graph, next, obs, directions, seen)
                            }
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }

            // let mut seen = vec![vec![false; graph.width()]; graph.height()];
            let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

            dfs(graph, start, &obs, &directions, seen);
        }
    }
}
