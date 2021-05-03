#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::ops::*;
use std::collections::*;

#[derive(Debug, Clone)]
pub struct Graph2D<T> {
    graph: Vec<Vec<T>>,
    width: isize,
    height: isize
}

impl<K> Graph2D<K>
where K: Eq + Copy
{
    pub fn new(graph: Vec<Vec<K>>) -> Self {
        let width = graph[0].len() as isize;
        let height = graph.len() as isize;
        Self { graph, width, height }
    }

    pub fn is_in(&self, x: isize, y: isize) -> bool {
        x < 0 || y < 0 || x >= self.width || y >= self.height
    }

    pub fn is_go(&self, x: isize, y: isize, obs: &Option<K>) -> Option<(usize, usize)> {
        if self.is_in(x, y) {
            return None
        }
        match obs {
            Some(obs) => { if &self[(x as usize, y as usize)] == obs { None } else { Some((x as usize, y as usize))}},
            None => Some((x as usize, y as usize)),
        }
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }
}

impl<K> Index<(usize, usize)> for Graph2D<K>
{
    type Output = K;

    fn index<'a>(&'a self, index: (usize, usize)) -> &'a K {
        let x: usize = index.0;
        let y: usize = index.1;

        &self.graph[y][x]
    }
}

impl<K> IndexMut<(usize, usize)> for Graph2D<K>
{
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut K {
        let x: usize = index.0;
        let y: usize = index.1;

        &mut self.graph[y][x]
    }
}

pub fn dfs2d<T: Eq + Copy>(graph: &Graph2D<T>, seen: &mut Vec<Vec<bool>>, start: (usize, usize), obs: Option<T>) {
    fn dfs<T: Eq + Copy>(
        graph: &Graph2D<T>, 
        start: (usize, usize), 
        obs: &Option<T>,
        directions: &Vec<(isize, isize)>, 
        seen: &mut Vec<Vec<bool>>,) {
            // dbg!(&seen);
            // if seen[start.1][start.0] { return; }
            seen[start.1][start.0] = true;

            for &direction in directions.iter() {
                let next_x = start.0 as isize + direction.0;
                let next_y = start.1 as isize + direction.1;

                match graph.is_go(next_x, next_y, obs) {
                    Some(next) => {
                        if !seen[next.1][next.0] {
                            dfs(graph, next, obs, directions, seen)
                        }
                    },
                    None => { continue; }
                }
            }
    }
    
    // let mut seen = vec![vec![false; graph.width()]; graph.height()];
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];

    dfs(graph, start, &obs, &directions, seen);
}


fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    loop {
        let w: usize = sc.read();
        let h: usize = sc.read();

        if w == 0 && h == 0 { break; }

        let g: Vec<Vec<usize>> = sc.vec2d(h, w);
        let graph = Graph2D::new(g);
        // dbg!(&graph);
        let mut seen = vec![vec![false; graph.width()]; graph.height()];
        let mut ans = 0;
        for x in 0..w {
            for y in 0..h {
                // dbg!(&seen);
                if seen[y][x] { continue; }
                if graph[(x, y)] == 0 { continue; }
                dfs2d(&graph, &mut seen, (x, y), Some(0));
                ans += 1;
            }
        }
        sc.write(ans);
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