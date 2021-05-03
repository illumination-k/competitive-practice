#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::ops::*;
use std::collections::*;

#[derive(Debug)]
struct Dfs {
    graph: Vec<Vec<usize>>,
    pub first_order: Vec<usize>,
    pub last_order: Vec<usize>,
    cnt: usize,
}

impl Dfs {
    fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            graph,
            first_order: vec![0; n],
            last_order: vec![0; n],
            cnt: 1,
        }
    }

    fn run(&mut self) {
        let mut seen = vec![false; self.graph.len()];
        let graph = self.graph.clone();
        for start in 0..seen.len() {
            if seen[start] { continue; }
            self.dfs(start, &graph, &mut seen)
        }
    }

    fn dfs(&mut self, start: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
        self.first_order[start] = self.cnt;
        self.cnt += 1;

        seen[start] = true;
        for &next in graph[start].iter() {
            if seen[next] { continue; }
            self.dfs(next, graph, seen)
        }

        self.last_order[start] = self.cnt;
        self.cnt += 1;
    }
}

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    
    for _ in 0..n {
        let u: usize = sc.read();
        let k: usize = sc.read();

        let v = if k != 0 { sc.vec(k).iter().map(|x: &usize| x - 1).collect() } else { vec![] };

        graph[u-1] = v;
    }

    let mut d = Dfs::new(graph);
    d.run();

    // println!("-----------");
    for i in 0..n {
        sc.write(format!("{} {} {}", i+1, d.first_order[i], d.last_order[i]));
    }
}


// from https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/utils/scanner.rs
pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

// let (r, w) = (std::io::stdin(), std::io::stdout());
// let mut sc = IO::new(r.lock(), w.lock())
impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        let w = format!("{}\n", s.to_string());
        self.1.write_all(w.to_string().as_bytes()).unwrap();
    }

    pub fn write_vec<S: ToString>(&mut self, v: Vec<S>) {
        use std::io::Write;
        let s = v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n");
        self.write(s);
    }

    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }

    pub fn vec2d<T: std::str::FromStr>(&mut self, h: usize, w: usize) -> Vec<Vec<T>> {
        (0..h).map(|_| self.vec(w)).collect()
    }

    pub fn set<T: std::str::FromStr + Eq + std::hash::Hash>(&mut self, n: usize) -> std::collections::HashSet<T> {
        (0..n).map(|_| self.read()).collect()
    }

    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}