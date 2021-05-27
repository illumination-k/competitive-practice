#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::vec;
use std::{collections::*, ops::*};
use superslice::*;
use whiteread::parse_line;

use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;

use competitive_internal_mod::format::*;
use competitive_internal_mod::graph::*;
use utils::debug;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

/*
強連結成分分解
*/

fn dfs(
    start: usize,
    g: &ListGraph<usize>,
    last_ptr: &mut usize,
    seen: &mut Vec<bool>,
    last_ord: &mut Vec<usize>,
) {
    seen[start] = true;

    for &next in g.neighbors_unweighted(start) {
        if seen[next] {
            continue;
        }
        dfs(next, g, last_ptr, seen, last_ord);
    }

    *last_ptr += 1;
    last_ord[start] = *last_ptr;
}

fn rev_dfs(start: usize, cur_id: i32, g: &ListGraph<usize>, scc_labels: &mut Vec<i32>) {
    scc_labels[start] = cur_id;

    for &next in g.neighbors_unweighted(start) {
        if scc_labels[next] != -1 {
            continue;
        }
        rev_dfs(next, cur_id, g, scc_labels);
    }
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m]
    }

    let g: ListGraph<usize> = ListGraph::unweighted_from(ab, n, 1, Direction::DiGraph);
    let rg = g.t();

    let mut seen = vec![false; n];
    let mut last_ptr = 0;
    let mut last_ord = vec![0; n];

    for i in 0..n {
        if seen[i] {
            continue;
        }
        dfs(i, &g, &mut last_ptr, &mut seen, &mut last_ord);
    }

    debug!(seen, last_ord);
    let mut sorted_last_ord = last_ord
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect_vec();
    sorted_last_ord.sort();
    sorted_last_ord.reverse();
    debug!(sorted_last_ord);
    let mut scc_labels = vec![-1; n];
    let mut cur_id = 0;
    for &(_, i) in sorted_last_ord.iter() {
        if scc_labels[i] != -1 {
            continue;
        }
        rev_dfs(i, cur_id, &rg, &mut scc_labels);
        cur_id += 1;
    }

    let mut counter = vec![0; cur_id as usize];

    for &scc_label in scc_labels.iter() {
        counter[scc_label as usize] += 1;
    }

    debug!(counter);

    let mut ans = 0;
    for cnt in counter {
        ans += ((cnt) * (cnt - 1)) / 2;
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
    pub mod graph {
        use num_traits::{Bounded, NumCast, One, Zero};
        use std::cmp::Reverse;
        use std::{
            collections::{BinaryHeap, HashSet, VecDeque},
            fmt,
            ops::{Index, IndexMut},
            slice::Iter,
            writeln,
        };
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Edge<W> {
            target: usize,
            weight: W,
        }

        impl<W> Edge<W>
        where
            W: One + Copy,
        {
            pub fn new(target: usize, weight: W) -> Self {
                Self { target, weight }
            }

            pub fn new_unweighted(target: usize) -> Self {
                Self {
                    target,
                    weight: W::one(),
                }
            }

            pub fn target(&self) -> usize {
                self.target
            }

            pub fn weight(&self) -> W {
                self.weight
            }
        }

        impl<W> ListGraph<W>
        where
            W: ToString + One + PartialEq + Clone + Copy,
        {
            /// create dot format for graphviz
            /// ```rust
            /// use competitive::graph::*;
            /// let vec = vec![
            ///     (0, 1, 1),
            ///     (0, 2, 4),
            ///     (2, 0, 1),
            ///     (1, 2, 2),
            ///     (3, 1, 1),
            ///     (3, 2, 5),
            /// ];
            /// let graph: ListGraph<isize> = ListGraph::weighted_from(vec, 4, 0, Direction::DiGraph);
            /// let dot = graph.to_dot(true, &Direction::DiGraph);
            /// assert_eq!(
            ///     dot,
            ///     vec![
            ///         "digraph digraph_example {",
            ///         "  0 -> 1 [ label = 1 ]",
            ///         "  0 -> 2 [ label = 4 ]",
            ///         "  1 -> 2 [ label = 2 ]",
            ///         "  2 -> 0 [ label = 1 ]",
            ///         "  3 -> 1 [ label = 1 ]",
            ///         "  3 -> 2 [ label = 5 ]",
            ///         "}"
            ///     ].iter().map(|x| x.to_string()).collect::<Vec<String>>()
            /// )
            /// ```
            pub fn to_dot(&self, weighted: bool, graph_type: &Direction) -> Vec<String> {
                fn make_dot_edge<W>(
                    source: usize,
                    target: usize,
                    weight: W,
                    weighted: bool,
                    graph_type: &Direction,
                ) -> String
                where
                    W: One + ToString + PartialEq,
                {
                    match graph_type {
                        Direction::DiGraph => {
                            if weighted {
                                format!(
                                    "  {} -> {} [ label = {} ]",
                                    source,
                                    target,
                                    weight.to_string()
                                )
                            } else {
                                format!("  {} -> {}", source, target)
                            }
                        }
                        Direction::UnGraph => {
                            if weighted {
                                format!(
                                    "  {} -> {} [ label = {}, dir = both ]",
                                    source,
                                    target,
                                    weight.to_string()
                                )
                            } else {
                                format!("  {} -> {} [ dir = both ]", source, target)
                            }
                        }
                    }
                }

                let mut seen_edge = HashSet::new();
                let mut dot = match graph_type {
                    Direction::DiGraph => vec!["digraph digraph_example {".to_string()],
                    Direction::UnGraph => vec!["digraph ungraph_example {".to_string()],
                };
                for source in 0..self.len() {
                    for e in self.neighbors(source) {
                        let (target, weight) = (e.target(), e.weight());
                        let dot_edge = make_dot_edge(source, target, weight, weighted, &graph_type);
                        if seen_edge.contains(&dot_edge) {
                            continue;
                        }
                        dot.push(dot_edge.clone());
                        seen_edge.insert(dot_edge);
                    }
                }

                dot.push("}".to_string());
                dot
            }
        }

        impl<W> PartialOrd for Edge<W>
        where
            W: PartialEq + PartialOrd,
        {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.weight.partial_cmp(&other.weight)
            }
        }

        impl<W> Eq for Edge<W> where W: PartialEq + PartialOrd {}

        impl<W> Ord for Edge<W>
        where
            W: PartialOrd + PartialEq + PartialOrd,
        {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.weight.partial_cmp(&other.weight).expect("No Nan") // if dist[position] < edge.weight() { continue; }
            }
        }

        #[derive(Clone)]
        pub struct ListGraph<W> {
            graph: Vec<Vec<Edge<W>>>,
        }

        impl<W> fmt::Debug for ListGraph<W>
        where
            W: Copy + One + Clone + ToString,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                writeln!(f, "")?;
                for source in 0..self.len() {
                    let targets = &self[source];
                    let mut write_str = String::new();
                    write_str += &(source.to_string() + " -> ");
                    write_str += &targets
                        .iter()
                        .map(|x| x.target().to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                    writeln!(f, "{}", write_str)?
                }
                Ok(())
            }
        }

        #[derive(Debug, Clone)]
        pub enum Direction {
            DiGraph,
            UnGraph,
        }

        fn add_target<W: One + Copy>(
            source: usize,
            target: usize,
            weight: W,
            graph_type: &Direction,
            v: &mut Vec<Vec<Edge<W>>>,
        ) {
            match graph_type {
                Direction::DiGraph => v[source].push(Edge::new(target, weight)),
                Direction::UnGraph => {
                    v[source].push(Edge::new(target, weight));
                    v[target].push(Edge::new(source, weight))
                }
            }
        }

        impl<W> ListGraph<W>
        where
            W: Clone + One + Copy,
        {
            pub fn new(n: usize) -> Self {
                Self {
                    graph: vec![vec![]; n],
                }
            }

            /// create unweighted ListGraph.  
            /// offset: index = val - offset  
            /// graph_type: Undirect or Direct  
            pub fn unweighted_from(
                edges: Vec<(usize, usize)>,
                size: usize,
                offset: usize,
                graph_type: Direction,
            ) -> Self {
                let mut graph = vec![vec![]; size];
                for &(a, b) in edges.iter() {
                    add_target(a - offset, b - offset, W::one(), &graph_type, &mut graph)
                }

                Self { graph }
            }

            /// create weighted ListGraph<W>.
            /// offset: index = val - offset
            /// graph_type: Undirect or Direct
            pub fn weighted_from(
                edges: Vec<(usize, usize, W)>,
                size: usize,
                offset: usize,
                graph_type: Direction,
            ) -> Self {
                let mut graph = vec![vec![]; size];
                for &(a, b, w) in edges.iter() {
                    add_target(a - offset, b - offset, w, &graph_type, &mut graph)
                }

                Self { graph }
            }

            /// Reverse Direction of Graph
            pub fn t(&self) -> Self {
                let mut vec = vec![];
                for source in 0..self.len() {
                    for e in self.neighbors(source) {
                        vec.push((e.target(), source, e.weight()))
                    }
                }

                ListGraph::<W>::weighted_from(vec, self.len(), 0, Direction::DiGraph)
            }

            pub fn len(&self) -> usize {
                self.graph.len()
            }

            pub fn neighbors(&self, source: usize) -> Iter<Edge<W>> {
                self[source].iter()
            }

            pub fn neighbors_unweighted<'a>(
                &'a self,
                source: usize,
            ) -> impl Iterator<Item = &'a usize> + 'a {
                self[source].iter().map(|x| &x.target)
            }
        }

        impl<W> Index<usize> for ListGraph<W> {
            type Output = Vec<Edge<W>>;

            fn index(&self, index: usize) -> &Self::Output {
                &self.graph[index]
            }
        }

        impl<W> IndexMut<usize> for ListGraph<W> {
            fn index_mut(&mut self, index: usize) -> &mut Vec<Edge<W>> {
                &mut self.graph[index]
            }
        }

        pub type UnweightedListGraph = ListGraph<usize>;

        /// **Diktstra**   
        /// O(|E+V|log(|V|))   
        /// let E be edge number, let V be vertex number.
        /// return distance from start and prev nodes information.
        /// restore_path function can create shortest path from start to goal from prev nodes.
        /// ```rust
        /// use competitive::graph::*;
        /// let vec = vec![
        ///     (0, 1, 1),
        ///     (0, 2, 4),
        ///     (2, 0, 1),
        ///     (1, 2, 2),
        ///     (3, 1, 1),
        ///     (3, 2, 5),
        /// ];
        /// let graph: ListGraph<isize> = ListGraph::weighted_from(vec, 4, 0, Direction::DiGraph);
        ///
        /// let (w, prev_nodes) = diktstra(&graph, 1);
        /// assert_eq!(w, vec![3, 0, 2, std::isize::MAX]);
        /// assert_eq!(restore_path(1, 0, &prev_nodes), vec![1, 2, 0]);
        /// ```
        pub fn diktstra<W>(graph: &ListGraph<W>, start: usize) -> (Vec<W>, Vec<usize>)
        where
            W: Copy + One + Zero + PartialEq + PartialOrd + NumCast + Bounded + fmt::Debug,
        {
            // initialize dist
            let mut dist: Vec<W> = vec![W::max_value(); graph.len()];
            dist[start] = W::zero();

            // initialize BinaryHeap
            let mut bq = BinaryHeap::new();
            // add dumy edge and start position.
            // edge weight must be smaller than zero()
            bq.push((Reverse(Edge::new(0, W::zero())), start));

            // initialize prev nodes
            let mut prev_nodes = vec![std::usize::MAX; graph.len()];

            while let Some((Reverse(edge), position)) = bq.pop() {
                if dist[position] < edge.weight() {
                    continue;
                }
                for &e in graph.neighbors(position) {
                    if dist[e.target()] <= dist[position] + e.weight() {
                        continue;
                    }
                    bq.push((Reverse(e), e.target()));
                    dist[e.target()] = dist[position] + e.weight();
                    prev_nodes[e.target()] = position;
                }
            }

            (dist, prev_nodes)
        }

        /// **BFS**  
        /// O(|V+E|)  
        /// let E be edge numbers, Let V be vertex numbers
        /// return distance from start and prev nodes information
        /// restore_path function can create shortest path from start to goal from prev nodes.
        /// ```rust
        /// use competitive::graph::*;
        /// let vec = vec![
        ///     (1, 2),
        ///     (1, 4),
        ///     (2, 4),
        ///     (4, 3)    
        /// ];
        /// let graph: UnweightedListGraph = ListGraph::unweighted_from(vec, 4, 1, Direction::DiGraph);
        /// let (dist, prev_nodes) = bfs(&graph, 0);
        /// assert_eq!(dist, vec![0, 1, 2, 1]);
        /// assert_eq!(restore_path(0, 2, &prev_nodes), vec![0, 3, 2]);
        /// ```
        pub fn bfs<W>(graph: &ListGraph<W>, start: usize) -> (Vec<isize>, Vec<usize>)
        where
            W: Copy + One + Clone,
        {
            let mut dist = vec![-1; graph.len()];
            dist[start] = 0;
            let mut prev_nodes = vec![std::usize::MAX; graph.len()];

            let mut vq = VecDeque::new();
            vq.push_back(start);

            while let Some(position) = vq.pop_front() {
                for &next in graph.neighbors_unweighted(position) {
                    if dist[next] != -1 {
                        continue;
                    }
                    dist[next] = dist[position] + 1;
                    prev_nodes[next] = position;
                    vq.push_back(next)
                }
            }

            (dist, prev_nodes)
        }

        /// restore shortest path from start to goal
        pub fn restore_path(start: usize, goal: usize, prev_nodes: &Vec<usize>) -> Vec<usize> {
            let mut res = vec![];
            let mut pos = goal;
            while pos != start {
                res.push(pos);
                pos = prev_nodes[pos];
            }
            res.push(start);
            res.reverse();
            res
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum DfsResultType {
            FirstAndLastOrd,
            TimeStamp,
            NoOrd,
        }

        #[derive(Debug, Clone)]
        pub struct DfsResults {
            pub seen: Vec<bool>,
            pub first_order: Vec<usize>,
            pub last_order: Vec<usize>,
            pub result_type: DfsResultType,
            ptrs: Vec<usize>,
        }

        impl DfsResults {
            pub fn new(size: usize, result_type: DfsResultType) -> Self {
                let ptrs = match &result_type {
                    DfsResultType::FirstAndLastOrd => vec![0; 2],
                    DfsResultType::TimeStamp => vec![0; 1],
                    DfsResultType::NoOrd => vec![],
                };
                Self {
                    seen: vec![false; size],
                    first_order: vec![0; size],
                    last_order: vec![0; size],
                    result_type: result_type,
                    ptrs: ptrs,
                }
            }

            /// update first ord  
            pub fn update_first_order(&mut self, pos: usize) {
                match self.result_type {
                    DfsResultType::FirstAndLastOrd => {
                        self.first_order[pos] = self.ptrs[0];
                        self.ptrs[0] += 1;
                    }
                    DfsResultType::TimeStamp => {
                        self.first_order[pos] = self.ptrs[0];
                        self.ptrs[0] += 1;
                    }
                    DfsResultType::NoOrd => {}
                }
            }

            /// update last ord  
            pub fn update_last_order(&mut self, pos: usize) {
                match self.result_type {
                    DfsResultType::FirstAndLastOrd => {
                        self.last_order[pos] = self.ptrs[1];
                        self.ptrs[1] += 1;
                    }
                    DfsResultType::TimeStamp => {
                        self.last_order[pos] = self.ptrs[0];
                        self.ptrs[0] += 1;
                    }
                    DfsResultType::NoOrd => {}
                }
            }
        }

        /// **DFS**  
        /// O(|V+E|)  
        /// let E be edge numbers, Let V be vertex numbers.
        /// return the DfsResults struct, which has seen results and first and last ord or time stamps.
        pub fn dfs<W>(start: usize, graph: &ListGraph<W>, result_type: DfsResultType) -> DfsResults
        where
            W: Copy + Clone + One,
        {
            /// internal dfs implementation
            fn _dfs<W>(start: usize, graph: &ListGraph<W>, dfs_result: &mut DfsResults)
            where
                W: Copy + Clone + One,
            {
                // memo of first ord
                dfs_result.update_first_order(start);

                dfs_result.seen[start] = true;
                for &next in graph.neighbors_unweighted(start) {
                    if dfs_result.seen[next] {
                        continue;
                    }
                    _dfs(next, graph, dfs_result)
                }

                // memo of last ord
                dfs_result.update_last_order(start);
            }

            // initialize dfs results
            let mut dfs_result = DfsResults::new(graph.len(), result_type);
            dfs_result.seen[start] = true;

            // rec dfs
            _dfs(start, graph, &mut dfs_result);
            dfs_result
        }
    }
}
