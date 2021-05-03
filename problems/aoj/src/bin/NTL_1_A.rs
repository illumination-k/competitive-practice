#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::ops::*;
use std::collections::*;


fn prime_factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            let mut ex = 0;
            while n % i == 0 {
                ex += 1;
                n = n / i;
            }
            res.push((i, ex));
        }
        i += 1;
    }
    if n != 1 {
        res.push((n, 1))
    }
    res
}

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let mut ans = vec![n.to_string(), ":".to_string()];
    let factors = prime_factorize(n);

    for &(f, ex) in factors.iter() {
        for _ in 0..ex {
            ans.push(f.to_string())
        }
    }
    println!("{}\n", ans.join(" "));
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