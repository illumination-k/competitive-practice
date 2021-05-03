#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{ops::*, vec};
use std::collections::*;


fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let k: usize = sc.read();
    let mut b = Board::new();

    for _ in 0..k {
        let rc: Vec<usize> = sc.vec(2);
        b.put_queen(rc[1], rc[0]);
        b.mark_cannot_put(rc[1], rc[0]);
    }

    // b.dbg();

    let mut v = vec![];

    for x in 0..8 {
        for y in 0..8 {
            if b.board[y][x] == '.' { v.push((x, y)) } 
        }
    }
    
    for com in combinations(v.iter(), 8-k) {
        let mut tb = b.clone();
        
        for (x, y) in com.iter() {
            if !tb.is_put(*x, *y) { break }
            tb.put_queen(*x, *y);
            tb.mark_cannot_put(*x, *y);
        }

        if tb.count_queen() == 8 { tb.ans(); break; }
    }
    
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<char>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: vec![vec!['.'; 8]; 8]
        }
    }

    pub fn dbg(&self) {
        println!("{}", self.board.iter()
                            .map(|x| x.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "))
                            .collect::<Vec<String>>().join("\n"))
    }

    pub fn ans(&self) {
        println!("{}", self.board.iter().map(|x| x.iter().map(|x| x.to_string().replace("-", ".")).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"))
    }

    pub fn put_queen(&mut self, x: usize, y: usize) {
        self.board[y][x] = 'Q'
    }

    pub fn mark_cannot_put(&mut self, x: usize, y:usize) {
        let directions = vec![(0, 1), (1, 0), (-1, 0), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

        for &(dx, dy) in directions.iter() {
            let mut cx = x as isize;
            let mut cy = y as isize;

            loop {
                cx += dx;
                cy += dy;
                if cx < 0 || cy < 0 || cx >= 8 || cy >= 8 { break; }
                if self.board[cy as usize][cx as usize] == 'Q' { continue; }

                self.board[cy as usize][cx as usize] = '-'
            }
        }
    }

    pub fn is_put(&self, x: usize, y: usize) -> bool {
        let mut flag = true;

        let directions = vec![(0, 1), (1, 0), (-1, 0), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

        for &(dx, dy) in directions.iter() {
            let mut cx = x as isize;
            let mut cy = y as isize;

            loop {
                cx += dx;
                cy += dy;
                if cx < 0 || cy < 0 || cx >= 8 || cy >= 8 { break; }
                if self.board[cy as usize][cx as usize] == 'Q' { flag = false; continue; }
            }
        }
        flag
    }

    pub fn count_queen(&self) -> usize {
        self.board.iter().flatten().filter(|x| **x == 'Q').count()
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


macro_rules! debug_fmt_fields {
    ($tyname:ident, $($($field:ident).+),*) => {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(stringify!($tyname))
                $(
              .field(stringify!($($field).+), &self.$($field).+)
              )*
              .finish()
        }
    }
}

macro_rules! clone_fields {
    ($($field:ident),*) => {
        fn clone(&self) -> Self {
            Self {
                $($field: self.$field.clone(),)*
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LazyBuffer<I: Iterator> {
    pub it: I,
    done: bool,
    buffer: Vec<I::Item>,
}

impl<I> LazyBuffer<I>
where
    I: Iterator,
{
    pub fn new(it: I) -> LazyBuffer<I> {
        LazyBuffer {
            it,
            done: false,
            buffer: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn get_next(&mut self) -> bool {
        if self.done {
            return false;
        }
        let next_item = self.it.next();
        match next_item {
            Some(x) => {
                self.buffer.push(x);
                true
            }
            None => {
                self.done = true;
                false
            }
        }
    }
}

impl<I, J> Index<J> for LazyBuffer<I>
where
    I: Iterator,
    I::Item: Sized,
    Vec<I::Item>: Index<J>
{
    type Output = <Vec<I::Item> as Index<J>>::Output;

    fn index(&self, _index: J) -> &Self::Output {
        self.buffer.index(_index)
    }
}


pub struct Combinations<I: Iterator> {
    indices: Vec<usize>,
    pool: LazyBuffer<I>,
    first: bool,
}

impl<I> Clone for Combinations<I>
    where I: Clone + Iterator,
          I::Item: Clone,
{
    clone_fields!(indices, pool, first);
}

impl<I> std::fmt::Debug for Combinations<I>
    where I: Iterator + std::fmt::Debug,
          I::Item: std::fmt::Debug,
{
    debug_fmt_fields!(Combinations, indices, pool, first);
}

/// Create a new `Combinations` from a clonable iterator.
pub fn combinations<I>(iter: I, k: usize) -> Combinations<I>
    where I: Iterator
{
    let mut pool: LazyBuffer<I> = LazyBuffer::new(iter);

    for _ in 0..k {
        if !pool.get_next() {
            break;
        }
    }

    Combinations {
        indices: (0..k).collect(),
        pool,
        first: true,
    }
}

impl<I> Iterator for Combinations<I>
    where I: Iterator,
          I::Item: Clone
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            if self.pool.is_done() {
                return None;
            }
            self.first = false;
        } else if self.indices.is_empty() {
            return None;
        } else {
            // Scan from the end, looking for an index to increment
            let mut i: usize = self.indices.len() - 1;

            // Check if we need to consume more from the iterator
            if self.indices[i] == self.pool.len() - 1 {
                self.pool.get_next(); // may change pool size
            }

            while self.indices[i] == i + self.pool.len() - self.indices.len() {
                if i > 0 {
                    i -= 1;
                } else {
                    // Reached the last combination
                    return None;
                }
            }

            // Increment index, and reset the ones to its right
            self.indices[i] += 1;
            for j in i+1..self.indices.len() {
                self.indices[j] = self.indices[j - 1] + 1;
            }
        }

        // Create result vector based on the indices
        Some(self.indices.iter().map(|i| self.pool[*i].clone()).collect())
    }
}