#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::{collections::*, ops::*};
use superslice::*;

use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;

use competitive_internal_mod::combinations::*;
use competitive_internal_mod::format::*;
use utils::debug;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

fn dfs(v: &mut Vec<usize>, set: &mut HashSet<Vec<usize>>) {
    if v.len() == 4 {
        set.insert(v.clone());
        return;
    }

    for i in 0..10 {
        v.push(i);
        dfs(v, set);
        v.pop();
    }
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    input! {
        s: Chars,
    }
    let mut mem = vec![0; 10];

    for i in 0..10 {
        match s[i] {
            'o' => mem[i] = 1,
            '?' => mem[i] = 2,
            _ => {}
        }
    }

    let mut ans: usize = 0;

    let mut set = HashSet::new();
    let mut v = vec![];

    dfs(&mut v, &mut set);
    for vv in set.iter() {
        let mut ok = true;
        for i in 0..10 {
            match mem[i] {
                0 => {
                    if vv.contains(&i) {
                        ok = false;
                        break;
                    }
                }
                1 => {
                    if !vv.contains(&i) {
                        ok = false;
                        break;
                    }
                }
                _ => {}
            }
        }

        if ok {
            ans += 1;
        }
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

#[cfg(test)]
mod test {
    use super::*;
}

mod competitive_internal_mod {
    pub mod combinations {
        #[derive(Debug, Clone)]
        pub struct Combination {
            fact_inv: Vec<usize>,
            inv: Vec<usize>,
            com: Option<Vec<usize>>,
            modulo: usize,
        }

        impl Combination {
            pub fn new(upper: usize, modulo: usize) -> Self {
                let mut fact_inv = vec![0; upper + 1];
                let mut inv = vec![0; upper + 1];
                fact_inv[0] = 1;
                fact_inv[1] = 1;
                inv[1] = 1;

                for i in 2..=upper {
                    inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
                    fact_inv[i] = fact_inv[i - 1] * inv[i] % modulo;
                }
                Self {
                    fact_inv: fact_inv,
                    inv: inv,
                    com: None,
                    modulo: modulo,
                }
            }

            pub fn fix_n(&mut self, n: usize) {
                let mut com = vec![0; n + 1];
                com[0] = 1;
                for i in 1..=n {
                    com[i] = com[i - 1] * ((n - i + 1) * self.inv[i] % self.modulo) % self.modulo;
                }
                self.com = Some(com)
            }

            fn _calc_nck(&self, n: usize, k: usize) -> usize {
                if n < k {
                    return 0;
                }
                let mut ans: usize = 1;
                let mut i = n;

                while n - k < i {
                    ans *= i;
                    ans %= self.modulo;
                    i -= 1;
                }
                ans * self.fact_inv[k] % self.modulo
            }

            pub fn nck(&self, n: usize, k: usize) -> usize {
                match &self.com {
                    None => self._calc_nck(n, k),
                    Some(x) => x[k],
                }
            }

            pub fn nhk(&self, n: usize, k: usize) -> usize {
                assert!(self.fact_inv.len() >= n + k - 1);
                self.nck(n + k - 1, k)
            }
        }

        /// simple calculation of combinations without modulo
        /// ```
        /// use competitive::combinations::combination;
        /// let res = combination(16);
        /// // get 16C11
        /// assert_eq!(res[16][11], 4368);
        /// // get 5C2
        /// assert_eq!(res[5][2], 10);
        /// ```
        pub fn combination(n: usize) -> Vec<Vec<usize>> {
            let mut v = vec![vec![0; n + 1]; n + 1];

            for i in 0..n + 1 {
                v[i][0] = 1;
                v[i][i] = 1;
            }

            for j in 1..n + 1 {
                for k in 1..j {
                    v[j][k] = v[j - 1][k - 1] + v[j - 1][k]
                }
            }

            v
        }

        /// Simple wrapper of combinaiton  
        /// ```
        /// use competitive::combinations::*;
        /// assert_eq!(nck(16, 11), 4368)
        /// ```
        pub fn nck(n: usize, k: usize) -> usize {
            combination(n)[n][k]
        }

        /// Simple wrapper of combinations for multi choises
        pub fn nhk(n: usize, k: usize) -> usize {
            combination(n + k - 1)[n + k - 1][k]
        }
    }
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

        impl AtCoderFormat for char {
            fn format(&self) -> String {
                self.to_string()
            }
        }

        impl AtCoderFormat for Vec<char> {
            fn format(&self) -> String {
                self.iter().collect::<String>()
            }
        }

        impl AtCoderFormat for Vec<Vec<char>> {
            fn format(&self) -> String {
                self.iter()
                    .map(|v| v.format())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        }

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
}
