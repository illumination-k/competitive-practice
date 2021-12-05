#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive_internal_mod::format::*;
use competitive_internal_mod::matrix2d::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*, source::auto::AutoSource};
use std::cmp::Reverse;
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

fn manhattan(x: &isize, y: &isize) -> isize {
    x.abs() + y.abs()
}

#[fastout]
fn run<R: BufRead>(mut source: AutoSource<R>) -> impl AtCoderFormat {
    input! {
        from &mut source,
        h: usize, w: usize,
        s: [Chars; h]
    }

    let mat = Matrix2D::new(s);
    let normal_directions = default_directions();
    let destroy_directions = iproduct!(-2..=2, -2..=2)
        .filter(|(x, y)| manhattan(x, y) != 0 && manhattan(x, y) != 4)
        .collect_vec();
    // debug!(directions, directions.len());

    let mut dist = Matrix2D::fill(-1, mat.shape());
    dist[(0, 0)] = 0;
    let mut bq = BinaryHeap::new();
    bq.push((Reverse(0), (0, 0)));

    while let Some((Reverse(val), (x, y))) = bq.pop() {
        debug!(bq, dist);
        for next in mat.next_index((x as usize, y as usize), &normal_directions) {
            if mat[next] == '#' {
                continue;
            }
            if dist[next] >= 0 {
                continue;
            }
            dist[next] = val;
            bq.push((Reverse(val), next))
        }

        for next in mat.next_index((x, y), &destroy_directions) {
            if dist[next] >= 0 {
                continue;
            }
            if mat[next] != '#' {
                continue;
            }

            dist[next] = val + 1;
            bq.push((Reverse(val + 1), next))
        }
    }
    debug!(dist);
    dist[(w - 1, h - 1)]
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

        macro_rules! impl_formats {
            ($($t: ty), *) => {
                $(impl_format!{$t})*
            };
        }

        impl_formats!(
            usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8, f32, f64, &str, String
        );

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
    pub mod matrix2d {
        use itertools::{iproduct, Product};
        use num_traits::{NumCast, Zero};
        use std::collections::*;
        use std::fmt;
        use std::{ops::*, writeln};

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Matrix2D<T> {
            pub matrix: Vec<Vec<T>>,
            pub width: usize,
            pub height: usize,
        }

        impl<T> fmt::Debug for Matrix2D<T>
        where
            T: Eq + Copy + std::fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                writeln!(f)?;
                writeln!(f, "width = {}, height = {}", self.width, self.height)?;
                writeln!(f, "--------------")?;
                for y in 0..self.height() {
                    for (i, x) in self.matrix[y].iter().enumerate() {
                        write!(f, "{:?}", x)?;
                        if i == self.width() - 1 {
                            writeln!(f, "")?;
                        } else {
                            write!(f, " ")?;
                        }
                    }
                }

                Ok(())
            }
        }

        impl<K> Matrix2D<K>
        where
            K: Eq + Copy,
        {
            pub fn new(matrix: Vec<Vec<K>>) -> Self {
                let width = matrix[0].len();
                let height = matrix.len();
                Self {
                    matrix,
                    width,
                    height,
                }
            }

            pub fn fill(value: K, shape: (usize, usize)) -> Self {
                let (width, height) = shape;
                let matrix = vec![vec![value; width]; height];
                Matrix2D::new(matrix)
            }

            pub fn is_not_in<I>(&self, x: I, y: I) -> bool
            where
                I: PartialEq + PartialOrd + NumCast + Zero,
            {
                if x < I::zero() || y < I::zero() {
                    return true;
                }

                let (xu, yu) = (
                    x.to_usize().unwrap_or_else(|| {
                        panic!("cannot convert usize from x in is_not_in function")
                    }),
                    y.to_usize().unwrap_or_else(|| {
                        panic!("cannot convert usize from y in is_not_in function")
                    }),
                );
                xu >= self.width || yu >= self.height
            }

            pub fn is_in(&self, x: isize, y: isize) -> bool {
                !(self.is_not_in(x, y))
            }

            pub fn width(&self) -> usize {
                self.width as usize
            }

            pub fn height(&self) -> usize {
                self.height as usize
            }

            pub fn shape(&self) -> (usize, usize) {
                (self.width, self.height)
            }

            /// get Iterator of a row
            pub fn row(&self, y: usize) -> std::slice::Iter<K> {
                self.matrix[y].iter()
            }

            pub fn row_mut(&mut self, y: usize) -> std::slice::IterMut<K> {
                self.matrix[y].iter_mut()
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
                Matrix2D::new(vec)
            }

            /// return iterator which represents all combination of index
            pub fn index_iter(&self) -> Product<Range<usize>, Range<usize>> {
                iproduct!(0..self.width, 0..self.height)
            }

            pub fn next_index(
                &self,
                source: (usize, usize),
                directions: &[(isize, isize)],
            ) -> impl Iterator<Item = (usize, usize)> + '_ {
                let mut v = vec![];
                for &(dx, dy) in directions.iter() {
                    let next_x = source.0 as isize + dx;
                    let next_y = source.1 as isize + dy;

                    if self.is_in(next_x, next_y) {
                        v.push((next_x as usize, next_y as usize))
                    }
                }

                v.into_iter()
            }

            pub fn diag(&self) {
                todo!()
            }
            pub fn diag_rev(&self) {
                todo!()
            }
        }

        impl Matrix2D<char> {
            pub fn from_str_slice<S: AsRef<str>>(v: &[S]) -> Self {
                let matrix: Vec<Vec<char>> =
                    v.iter().map(|x| x.as_ref().chars().collect()).collect();

                Matrix2D::new(matrix)
            }
        }

        impl<K> Index<(usize, usize)> for Matrix2D<K> {
            type Output = K;

            fn index<'a>(&'a self, index: (usize, usize)) -> &'a K {
                let (x, y): (usize, usize) = index;

                &self.matrix[y][x]
            }
        }

        impl<K> IndexMut<(usize, usize)> for Matrix2D<K> {
            fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut K {
                let (x, y): (usize, usize) = index;

                &mut self.matrix[y][x]
            }
        }

        /// bfs of 2d graph
        pub fn bfs2d<T: Eq + Copy>(
            graph: &Matrix2D<T>,
            start: (usize, usize),
            obs: Option<T>,
        ) -> Matrix2D<isize> {
            let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            let mut dist: Matrix2D<isize> =
                Matrix2D::new(vec![vec![-1; graph.width()]; graph.height()]);
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

            dist[start] = 0;
            queue.push_back(start);

            while let Some(s) = queue.pop_front() {
                for next in graph.next_index(s, &directions) {
                    if let Some(o) = obs {
                        if graph[next] == o {
                            continue;
                        }
                    }

                    if dist[next] == -1 {
                        dist[next] = dist[s] + 1;
                        queue.push_back(next)
                    }
                }
            }

            dist
        }

        pub fn default_directions() -> Vec<(isize, isize)> {
            vec![(0, 1), (1, 0), (-1, 0), (0, -1)]
        }

        /// dfs of 2d graph
        pub fn dfs2d<T: Eq + Copy>(
            graph: &Matrix2D<T>,
            seen: &mut Matrix2D<bool>,
            start: (usize, usize),
            obs: Option<T>,
        ) {
            fn dfs<T: Eq + Copy>(
                graph: &Matrix2D<T>,
                start: (usize, usize),
                obs: &Option<T>,
                directions: &[(isize, isize)],
                seen: &mut Matrix2D<bool>,
            ) {
                seen[start] = true;

                for next in graph.next_index(start, directions) {
                    if let Some(o) = obs {
                        if graph[next] == *o {
                            continue;
                        }
                    }

                    if !seen[next] {
                        dfs(graph, next, obs, directions, seen)
                    }
                }
            }
            let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

            dfs(graph, start, &obs, &directions, seen);
        }
    }
}
