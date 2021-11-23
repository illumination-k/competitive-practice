#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive_internal_mod::format::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*, source::auto::AutoSource};
use std::{
    collections::*,
    io::{BufRead, BufReader},
    ops::*,
};
use superslice::*;
use utils::*;

use competitive_internal_mod::prime::*;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

fn solve(n: usize, m: usize, a: &[usize]) -> Vec<usize> {
    let osak = OsaK::new(200000 as usize);

    let mut prime_set: HashSet<usize> = HashSet::new();
    for &elem in a.iter() {
        let primes = osak.prime_factorize(elem);
        prime_set.extend(primes.keys());
    }

    let mut ans = vec![1];
    for i in 2..=m {
        let primes = osak.prime_factorize(i);
        let mut flag = true;
        for k in primes.keys() {
            if prime_set.contains(k) {
                flag = false;
                continue;
            }
        }
        if flag {
            ans.push(i)
        }
    }
    ans.sort_unstable();
    ans
}

#[fastout]
fn run<R: BufRead>(mut source: AutoSource<R>) -> impl AtCoderFormat {
    input! {
        from &mut source,
        n: usize, m: usize,
        a: [usize; n]
    }

    let ans = solve(n, m, &a);
    println!("{}", ans.len());
    ans
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

    #[test]
    fn test() {
        for _ in 0..10000 {
            let n = gen_number(2, 10000);
            let m = gen_number(2, 10000);
            let a = make_random_vec(n, (0, 10000));
            solve(n, m, &a);
        }
    }
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

        macro_rules! impl_formats {
            ($($t: ty), *) => {
                $(impl_format!{$t})*
            };
        }

        impl_formats!(
            usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8, f32, f64, &str, String
        );

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

        pub fn is_prime<T>(n: T) -> bool
        where
            T: PrimInt + NumAssign,
        {
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

        pub fn enum_divisors<T>(n: T) -> Vec<T>
        where
            T: PrimInt + NumAssign,
        {
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

        pub fn prime_factorize<T>(mut n: T) -> Vec<(T, T)>
        where
            T: PrimInt + NumAssign,
        {
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
            let n = n
                .to_usize()
                .expect("cannot convert n to usize in sieve_of_eratosthenes");
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
                assert!(!vec.is_empty());
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
