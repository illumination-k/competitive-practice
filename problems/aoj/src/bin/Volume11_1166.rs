#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{io::StdinLock, ops::*, usize};
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

    pub fn is_not_in(&self, x: isize, y: isize) -> bool {
        x < 0 || y < 0 || x >= self.width || y >= self.height
    }

    pub fn is_go(&self, x: isize, y: isize, obs: &Option<K>) -> Option<(usize, usize)> {
        if self.is_not_in(x, y) {
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
        let (x, y): (usize, usize) = index;

        &self.graph[y][x]
    }
}

impl<K> IndexMut<(usize, usize)> for Graph2D<K>
{
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut K {
        let (x, y): (usize, usize) = index;

        &mut self.graph[y][x]
    }
}


fn solve(h: usize, w: usize, walls: HashMap<(usize, usize), HashSet<(usize, usize)>>) {
    let mut dq: VecDeque<(usize, usize)> = VecDeque::new();
    let start = (0, 0); let goal = (w-1, h-1);

    dq.push_back(start);
    
    let graph = Graph2D::new(vec![vec![0; w]; h]);
    let mut dist = Graph2D::new(vec![vec![-1; w]; h]);
    dist[start] = 0;
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(s) = dq.pop_front() {
        for &(dx, dy) in directions.iter() {
            let next_x = s.0 as isize + dx;
            let next_y = s.1 as isize + dy;

            let next = (next_x as usize, next_y as usize);

            if graph.is_not_in(next_x, next_y) { continue; }
            if dist[next] != -1 { continue; }
            
            if let Some(set) = walls.get(&s) {
                if set.contains(&(next)) { continue; }
            }

            dist[next] = dist[s] + 1;
            dq.push_back(next);
        }
    }

    println!("{}", dist[goal] + 1)
}

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    loop {
        let w: usize = sc.read();
        let h: usize = sc.read();

        if h == 0 && w == 0 { break; }

        let mut walls: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

        let mut y = 0;
        for i in 0..h*2 - 1 {
            if i % 2 == 0 {
                let row: Vec<usize> = sc.vec(w-1);
                // dbg!(&row);
                for (i, &x) in row.iter().enumerate() {
                    if x == 0 { continue; }
                    walls.entry((i, y)).or_insert(HashSet::new()).insert((i+1, y));
                    walls.entry((i+1, y)).or_insert(HashSet::new()).insert((i, y));
                }
                y += 1;
            } else {
                let col: Vec<usize> = sc.vec(w);
                // dbg!(&col);
                for (i, &x) in col.iter().enumerate() {
                    if x == 0 { continue; }
                    walls.entry((i, y)).or_insert(HashSet::new()).insert((i, y-1));
                    walls.entry((i, y-1)).or_insert(HashSet::new()).insert((i, y));
                }
            }
        }
        // dbg!(&walls);
        solve(h, w, walls);
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