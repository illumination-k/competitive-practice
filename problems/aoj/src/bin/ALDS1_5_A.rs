#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{ops::*, vec};
use std::collections::*;


fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let q: usize = sc.read();
    let m: Vec<usize> = sc.vec(q);

    let mut ans = vec![];
    for &mm in m.iter() {
        let mut flag = false;
        for bit in 0..1<<n {
            let mut sum = 0;
            for i in 0..n {
                if bit & 1<<i != 0 { sum += a[i] }
            }

            if sum == mm { flag = true; break; }
        }

        if flag {
            ans.push("yes");
        } else {
            ans.push("no");
        }
    }
    sc.write(format!("{}\n", ans.join("\n")))
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
        self.1.write_all(s.to_string().as_bytes()).unwrap();
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