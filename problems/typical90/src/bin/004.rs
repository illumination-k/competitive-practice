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
use competitive_internal_mod::graph2d::*;

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
        h: usize, w: usize,
        a: [[usize; w]; h]
    }
    let g = Graph2D::new(a);
    let mut col_sum = vec![];
    let mut row_sum = vec![];
    for x in 0..w {
        let y_sum = g.col(x).sum::<usize>();
        col_sum.push(y_sum)
    }

    for y in 0..h {
        let x_sum = g.row(y).sum::<usize>();
        row_sum.push(x_sum)
    }

    let mut ans = vec![vec![]; h];

    for x in 0..w {
        for y in 0..h {
            let sum = col_sum[x] + row_sum[y] - g[(x, y)];
            ans[y].push(sum);
        }
    }

    debug!(ans);

    ans
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
