#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive::format::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*, source::auto::AutoSource};
use std::{
    collections::*,
    io::{BufRead, BufReader},
    ops::*, hash::Hash,
};
use superslice::*;
use utils::*;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

#[derive(Debug, Clone)]
struct Cylinders {
    content: Vec<Vec<usize>>,
    map: HashMap<usize, usize>,
    index: usize,
}

impl Cylinders {
    fn new(mut content: Vec<Vec<usize>>) -> Self {
        let mut map = HashMap::new();
        let index = 0;
        map.entry(content[index].pop().unwrap()).or_insert(0);

        Self { content, map, index }
    }

    fn rec(&mut self) {
        self.index += 1;

        if self.index >= self.content.len() {
            
        }

        if let Some(val) = self.content[self.index].pop() {
            
        }
    }
}


#[fastout]
fn run<R: BufRead>(mut source: AutoSource<R>) -> impl AtCoderFormat {
    input! {
        from &mut source,
        n: usize, m: usize,
    }

    let mut cylinders = vec![];

    for _ in 0..m {
        input! {
            from &mut source,
            k: usize,
            a: [usize; k]
        }

        cylinders.push(a);
    }

    let mut cylinders = Cylinders::new(cylinders);
    debug!(cylinders);
    false
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
