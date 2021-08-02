#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive_internal_mod::format::*;
use competitive_internal_mod::math::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::{collections::*, ops::*};
use superslice::*;
use utils::*;

const MOD: usize = 1_000_000_007;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

#[fastout]
fn run() -> impl AtCoderFormat {
    input! {
        n: usize, t: [usize; n]
    }

    if t.len() == 1 {
        return t[0];
    }

    let lcm = lcm_list(t);

    lcm
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
    pub mod math {
        use std::fmt::Debug;

        use num_traits::{one, zero, NumCast, PrimInt};

        /// GCD
        pub fn gcd<T: PrimInt>(a: T, b: T) -> T {
            if b == zero() {
                a
            } else {
                gcd(b, a % b)
            }
        }

        /// LCM
        pub fn lcm<T: PrimInt>(a: T, b: T) -> T {
            a / gcd(a, b) * b
        }

        /// GCD of vec
        pub fn gcd_list<T: PrimInt>(vec: Vec<T>) -> T {
            assert!(vec.len() > 1);
            vec.iter().fold(vec[0], |acc, x| gcd(*x, acc))
        }

        /// LCM of vec
        pub fn lcm_list<T: PrimInt>(vec: Vec<T>) -> T {
            assert!(vec.len() > 1);
            vec.iter().fold(vec[0], |acc, x| lcm(*x, acc))
        }

        /// ans of quadratic formula ax^2 + bx + c = 0
        pub fn quadratic_formula<T: NumCast>(a: T, b: T, c: T) -> Option<(f64, f64)> {
            let a = a.to_f64().unwrap();
            let b = b.to_f64().unwrap();
            let c = c.to_f64().unwrap();

            let descriminant = b * b - 4.0 * a * c;

            if descriminant > 0.0 {
                let ans_1 = (-b + descriminant.sqrt()) / (2.0 * a);
                let ans_2 = (-b - descriminant.sqrt()) / (2.0 * a);
                return Some((ans_1, ans_2));
            } else if descriminant == 0.0 {
                let ans = -b / (2.0 * a);
                return Some((ans, ans));
            } else {
                return None;
            }
        }

        fn safe_mod(mut x: i64, modulo: i64) -> i64 {
            x %= modulo;
            if x < 0 {
                x += modulo;
            }
            x
        }

        pub fn ext_gcd<T: NumCast + PrimInt>(a: T, b: T) -> (T, T) {
            let a = a.to_i64().expect("a can not convert to i64");
            let b = b.to_i64().expect("b cannot convert to i64");
            let a = safe_mod(a, b);
            if a == 0 {
                return (T::from(b).unwrap(), T::from(0).unwrap());
            }

            // Contracts:
            // [1] s - m0 * a = 0 (mod b)
            // [2] t - m1 * a = 0 (mod b)
            // [3] s * |m1| + t * |m0| <= b
            let mut s = b;
            let mut t = a;
            let mut m0 = 0;
            let mut m1 = 1;

            while t != 0 {
                let u = s / t;
                s -= t * u;
                m0 -= m1 * u; // |m1 * u| <= |m1| * s <= b

                // [3]:
                // (s - t * u) * |m1| + t * |m0 - m1 * u|
                // <= s * |m1| - t * u * |m1| + t * (|m0| + |m1| * u)
                // = s * |m1| + t * |m0| <= b

                std::mem::swap(&mut s, &mut t);
                std::mem::swap(&mut m0, &mut m1);
            }
            // by [3]: |m0| <= b/g
            // by g != b: |m0| < b/g
            if m0 < 0 {
                m0 += b / s;
            }
            (T::from(s).unwrap(), T::from(m0).unwrap())
        }

        /// n!
        pub fn frac(mut n: usize) -> usize {
            if n == 0 {
                return 1;
            }
            if n <= 2 {
                return n;
            }

            let mut prod = 1;
            while n != 0 {
                prod *= n;
                n -= 1;
            }
            prod
        }

        pub fn inv_mod<T: NumCast + PrimInt>(x: T, m: T) -> T {
            assert!(one::<T>() <= m);
            let z = ext_gcd(x, m);
            assert!(z.0 == one::<T>());
            z.1
        }

        /// sum of Arithmetic progression  
        /// **a0**: the first term of serires    
        /// **d**: common difference  
        /// **n**: number of terms  
        pub fn arithmetic_progression_sum<T: PrimInt>(a0: T, d: T, n: T) -> T {
            n * ((T::one() + T::one()) * a0 + (n - T::one()) * d) / (T::one() + T::one())
        }

        /// sum of geometric progression  
        /// **a0**: the first term of serires    
        /// **r**: geometric progression    
        /// **n**: number of terms
        pub fn geometric_progression_sum<T: PrimInt + Debug>(a0: T, r: T, n: T) -> T {
            assert_ne!(r, T::one());

            a0 * (T::one() - r.pow(n.to_u32().unwrap())) / (T::one() - r)
        }
    }
}
