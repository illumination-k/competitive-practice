#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::hash::Hash;
use std::{collections::*, ops::*};
use superslice::*;

use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;

use competitive_internal_mod::format::*;
use competitive_internal_mod::prime::*;
use utils::debug;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

fn euler_func(r: usize, prime_fact: &HashMap<usize, usize>) -> usize {
    let mut n = r;
    for &k in prime_fact.keys() {
        n *= k - 1;
        n /= k;
    }

    n
}

#[fastout]
fn solve() -> impl AtCoderFormat {
    input! {
        l: isize, r: isize,
    }

    let osak = OsaK::new(r + 1);

    let mut ans = 0;
    let mut max_fact_num = 0;
    for i in l..=r {
        if osak.is_prime(i) || i == 1 {
            continue;
        }

        let fact = osak.prime_factorize(i);
        max_fact_num = std::cmp::max(fact.keys().count(), max_fact_num);
        let fact_keys = fact.keys().collect_vec();
        // i -> rまでの間にある素因数の倍数の数
        let mut fact_multiple_count: isize = 0;

        // 互いに素でないものの数え上げ
        // 包除原理
        let mut r_not_coprime = 0;
        let mut i_not_coprime = 0;
        for bit in 0..1 << fact_keys.len() {
            let popcnt = bit.count_ones();
            let mut mul = 1;
            for i in 0..fact_keys.len() {
                if bit & 1 << i != 0 {
                    mul *= fact_keys[i];
                }
            }

            // debug!(popcnt, mul);
            if mul == 1 {
                continue;
            }
            if popcnt % 2 == 1 {
                r_not_coprime += r / mul;
                i_not_coprime += i / mul;
            } else {
                r_not_coprime -= r / mul;
                i_not_coprime -= i / mul;
            }
        }

        fact_multiple_count += r_not_coprime - i_not_coprime;

        // 自分自身で割れるものは除く。
        fact_multiple_count -= r / i - 1;
        // debug!(i, r_not_coprime, i_not_coprime, fact_multiple_count);
        ans += fact_multiple_count * 2;
    }

    debug!(max_fact_num);

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

    #[test]
    fn test_euler() {
        let osak = OsaK::new(20);
        let prime_fact = osak.prime_factorize(12);
        assert_eq!(euler_func(12, &prime_fact), 4);
    }
}

mod competitive_internal_mod {
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
    pub mod prime {
        use num_traits::*;

        pub fn is_prime<T: PrimInt + NumAssign>(n: T) -> bool {
            // O(sqrt(n))
            let mut flag: bool = true;

            if n == one() {
                flag = false
            }
            let mut i: T = one::<T>().signed_shl(1);
            while i * i <= n {
                if n % i == zero() {
                    flag = false;
                    break;
                }
                i += one();
            }
            flag
        }

        pub fn enum_divisors<T: PrimInt + NumAssign>(n: T) -> Vec<T> {
            // O(sqrt(n))
            let mut res: Vec<T> = Vec::new();

            let mut i: T = one();

            while i * i <= n {
                if n % i == zero() {
                    res.push(i);
                    if n / i != i {
                        res.push(n / i)
                    }
                }
                i += one();
            }
            res.sort();
            res
        }

        pub fn prime_factorize<T: PrimInt + NumAssign>(mut n: T) -> Vec<(T, T)> {
            // O(sqrt(n))
            let mut res: Vec<(T, T)> = Vec::new();

            let mut i: T = one::<T>().signed_shl(1);

            while i * i <= n {
                if n % i == zero() {
                    let mut ex = zero::<T>();

                    while n % i == zero() {
                        ex += one();
                        n = n / i;
                    }
                    res.push((i, ex));
                }
                i += one();
            }

            if n != one() {
                res.push((n, one()))
            }

            res
        }

        pub fn sieve_of_eratosthenes<T: NumCast>(n: T) -> Vec<usize> {
            let n = n.to_usize().expect("cannot convert n to usize");
            if n < 2 {
                return vec![];
            }

            let mut flags = vec![true; n / 2];
            flags[0] = false;

            let sqrt_x = (((n as f64).sqrt() + 0.1).ceil() + 0.5) as usize;
            let sqrt_xi = sqrt_x / 2;

            for i in 1..sqrt_xi {
                if !flags[i] {
                    continue;
                }
                let p = 2 * i + 1;
                let start = 2 * i * (i + 1);
                for mult in (start..flags.len()).step_by(p) {
                    flags[mult] = false;
                }
            }

            std::iter::once(2)
                .chain(
                    flags
                        .iter()
                        .enumerate()
                        .filter(|(_i, flag)| **flag)
                        .map(|(i, _flag)| 2 * i + 1),
                )
                .collect()
        }

        #[derive(Debug, Clone)]
        pub struct OsaK<T: PrimInt + std::hash::Hash + NumAssign> {
            sieve: Vec<T>,
            max: T,
        }

        fn _make_sieve<T: PrimInt>(mut maxu: usize) -> Vec<T> {
            maxu += 1;
            let mut sieve: Vec<usize> = (0..maxu).collect();

            let mut i = 2;
            while i * i < maxu {
                if sieve[i] < i {
                    i += 1;
                    continue;
                }
                for j in (i * i..maxu).step_by(i) {
                    if sieve[j] == j {
                        sieve[j] = i
                    }
                }
                i += 1;
            }

            sieve.into_iter().filter_map(|x| T::from(x)).collect()
        }

        impl<T: PrimInt + std::hash::Hash + NumAssign> OsaK<T> {
            /// O(maxloglog(max))   
            /// construct osa-k from max size
            pub fn new(max: T) -> Self {
                let maxu = max.to_usize().expect("cannot convert to usize");
                let sieve = _make_sieve(maxu);

                Self { sieve, max }
            }

            /// O(max(vec)loglog(max(vec)))  
            /// construct osa-k from Vector
            pub fn from(vec: Vec<T>) -> Self {
                assert!(vec.len() > 0);
                let max = vec.iter().max().unwrap();
                let maxu = max.to_usize().unwrap();
                let sieve = _make_sieve(maxu);

                Self { sieve, max: *max }
            }

            /// O(1)
            /// test x is prime or not
            pub fn is_prime(&self, x: T) -> bool {
                assert!(x <= self.max);
                if x == one() || x == zero() {
                    return false;
                }
                self.sieve[x.to_usize().unwrap()] == x
            }

            /// O(log(n))  
            /// prime factoraize
            pub fn prime_factorize(&self, mut n: T) -> std::collections::HashMap<T, T> {
                assert!(n <= self.max);
                if n == zero() || n == one() {
                    return std::collections::HashMap::new();
                }

                let mut res: std::collections::HashMap<T, T> = std::collections::HashMap::new();
                while n > one() {
                    *res.entry(self.sieve[n.to_usize().unwrap()])
                        .or_insert(zero()) += one();
                    n /= self.sieve[n.to_usize().unwrap()]
                }
                res
            }
        }
    }
}
