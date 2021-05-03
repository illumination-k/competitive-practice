#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::ops::*;
use std::collections::*;

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());


    let v: usize = sc.read();
    let e: usize = sc.read();
    const INF: usize = 1 << 50;
    let mut cost = vec![vec![INF; v]; v];
    for _ in 0..e {
        let input: Vec<usize> = sc.vec(3);
        cost[input[0]][input[1]] = input[2];
    }
    
    let mut dp = vec![vec![INF; v]; 1<<v];

    dp[0][0] = 0;
    for bit in 1..1<<v {
        for t in 0..v {
            if bit & 1<<t > 0 {
                for k in 0..v {
                    dp[bit][t] = std::cmp::min(dp[bit][t], dp[bit ^ (1<<t)][k] + cost[k][t]);
                }
            }
        }
    }

    if dp[(1<<v)-1][0] == INF {
        sc.write(-1)
    } else {
        sc.write(dp[(1<<v)-1][0])
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