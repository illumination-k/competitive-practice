#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive::format::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;
use permutohedron::LexicalPermutation;
use proconio::{fastout, input, marker::*};
use std::{collections::*, ops::*};
use superslice::*;
use utils::*;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

type CharMap = Vec<Vec<Vec<usize>>>;

fn update_charmap(c: &char, i: usize, s: &[char], p: usize, char_map: &mut CharMap) {
    for j in 0..s.len() {
        if &s[j] == c {
            char_map[i][p].push(j);
        }
    }
}

/// s1, s2, s3のどの位置が同じであればいいかを判定  
/// element -> ([], [], [])でs1 -> e.0, s2 -> e.1, s3 -> e.2
fn chars_map(s1: &[char], s2: &[char], s3: &[char], chars: &[char]) -> CharMap {
    let mut char_map = vec![vec![vec![]; 3]; chars.len()];

    for (i, c) in chars.iter().enumerate() {
        update_charmap(c, i, s1, 0, &mut char_map);
        update_charmap(c, i, s2, 1, &mut char_map);
        update_charmap(c, i, s3, 2, &mut char_map);
    }

    char_map
}

fn get_index(cm: &[Vec<usize>]) -> (usize, usize) {
    let n = cm
        .iter()
        .enumerate()
        .filter(|(_i, x)| x.len() != 0)
        .next()
        .unwrap();
    (n.0, n.1[0])
}

fn check_cm(
    cm: &[Vec<usize>],
    n: &[usize],
    exc: &mut Vec<usize>,
    p: usize,
    val: usize,
    count: &mut usize,
) -> bool {
    for i in 0..n.len() {
        debug!(i, n[i]);
        if cm[p].contains(&i) {
            if n[i] != val {
                return false;
            } else {
                *count += 1;
            }
        } else {
            if n[i] == val {
                return false;
            } else {
                exc.push(n[i]);
            }
        }
    }

    true
}

fn check(n1: &[usize], n2: &[usize], n3: &[usize], char_map: &CharMap) -> bool {
    for cm in char_map.iter() {
        let (t, i) = get_index(cm);
        let val = match t {
            0 => n1[i],
            1 => n2[i],
            2 => n3[i],
            _ => unreachable!(),
        };
        debug!(val);

        let mut exc = vec![];
        let val_num = cm.iter().flatten().count();
        let mut count = 0;
        let f1 = check_cm(cm, n1, &mut exc, 0, val, &mut count);
        let f2 = check_cm(cm, n2, &mut exc, 1, val, &mut count);
        let f3 = check_cm(cm, n3, &mut exc, 2, val, &mut count);
        let f4 = exc.len() == exc.iter().unique().count();
        let f5 = count == val_num;
        debug!(count, val_num, exc);

        if f1 && f2 && f3 && f4 && f5 {
            return true;
        }
    }

    true
}

fn add_n1_n2(n1: &[usize], n2: &[usize]) -> Vec<usize> {
    let n1u = n1.iter().join("").parse::<usize>().unwrap();

    let n2u = n2.iter().join("").parse::<usize>().unwrap();

    let n3u = n1u + n2u;
    n3u.to_string()
        .chars()
        .map(|c| c as usize - 48)
        .collect_vec()
}

#[fastout]
fn run() -> impl AtCoderFormat {
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
    }

    // n1, n2を作る
    let unr = "UNRESOLVE".to_string();
    let n = s1.len() + s2.len();
    let unique_chars = s1.iter().chain(s2.iter()).unique().cloned().collect_vec();
    let unique_char_num = unique_chars.len();
    if unique_char_num > 10 {
        return unr;
    }

    let char_map = chars_map(&s1, &s2, &s3, &unique_chars);
    debug!(char_map);

    for perm in (0..=9).permutations(n) {
        let n1 = &perm[..s1.len()];
        let n2 = &perm[s1.len()..];
        if n1[0] == 0 || n2[0] == 0 {
            continue;
        }
        let n3 = add_n1_n2(n1, n2);
        if n3.len() != s3.len() {
            continue;
        }
        debug!(n1, n2, n3);
        let flag = check(n1, n2, &n3, &char_map);
        if flag {
            let ans = [
                n1.iter().join(" "),
                n2.iter().join(" "),
                n3.iter().join(" "),
            ]
            .iter()
            .join("\n");
            return ans;
        }
    }

    unr
}

fn main() {
    println!("{}", run().format());
}

#[cfg(test)]
mod test {
    use super::*;
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
