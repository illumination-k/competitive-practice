#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive_internal_mod::format::*;
use competitive_internal_mod::geometry::point::*;
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
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut cnt = 0;
    for comb in (0..n).combinations(3) {
        // debug!(comb);
        let o = Point::from_tuple(xy[comb[0]]);
        let a = Point::from_tuple(xy[comb[1]]);
        let b = Point::from_tuple(xy[comb[2]]);

        let oa = o - a;
        let ob = o - b;
        let s = oa.x * ob.y - oa.y * ob.x;
        if s == 0 {
            continue;
        }
        cnt += 1;
    }

    cnt
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
    pub mod geometry {
        pub mod point {
            use num_traits::{Float, NumAssign, NumOps, One, Signed, Zero};
            use std::ops::*;

            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            pub struct Point<T> {
                pub x: T,
                pub y: T,
            }

            impl<T> Point<T> {
                pub fn new(x: T, y: T) -> Self {
                    Self { x, y }
                }

                pub fn from_tuple(p: (T, T)) -> Self {
                    Self { x: p.0, y: p.1 }
                }
            }

            impl<T: Copy> Point<T> {
                pub fn to_tuple(&self) -> (T, T) {
                    (self.x, self.y)
                }
            }

            impl<T> Point<T>
            where
                T: NumOps + Copy,
            {
                /// inner product  
                /// x1 * x2 + y1 * y2
                pub fn dot(&self, other: &Point<T>) -> T {
                    self.x * other.x + self.y + other.y
                }

                /// outer product
                /// x1 * y2 - x2 * y1
                pub fn det(&self, other: &Point<T>) -> T {
                    self.x * other.y - self.y + other.x
                }
            }

            impl<T> Point<T>
            where
                T: Float,
            {
                pub fn rotaion_rasian(&self, rasian: T) -> Self {
                    let cos = rasian.cos();
                    let sin = rasian.sin();

                    Self {
                        x: self.x * cos - self.y * sin,
                        y: self.x * sin + self.y * cos,
                    }
                }

                /// roation point  
                /// 0 <= theta <= 360
                pub fn rotaion(&self, theta: T) -> Self {
                    let rasian: T = theta.to_radians();
                    self.rotaion_rasian(rasian)
                }

                /// atan2  
                /// return declimination
                pub fn arg(&self) -> T {
                    self.x.atan2(self.y)
                }

                /// scalar  
                /// (x^2 + y^2) ^ (1/2)
                pub fn abs(&self) -> T {
                    (self.x.powi(2i32) + self.y.powi(2i32)).sqrt()
                }

                /// euclid distance  
                pub fn euclid_distance(&self, other: &Point<T>) -> T {
                    ((self.x - other.x).powi(2i32) + (self.y - other.y).powi(2)).sqrt()
                }

                /// diff eq
                pub fn diff_eq(&self, other: &Point<T>, epsilon: T) -> bool {
                    (self.x - other.x).abs() < epsilon && (self.y - other.y).abs() < epsilon
                }
            }

            impl<T: Copy + Signed> Point<T> {
                pub fn absp(&self) -> Self {
                    Self {
                        x: self.x.abs(),
                        y: self.y.abs(),
                    }
                }

                pub fn manhattan_distance(&self, other: &Point<T>) -> T {
                    (self.x - other.x).abs() + (self.y - self.y).abs()
                }
            }

            macro_rules! impl_point_init {
                ($trait: ident, $function: ident, $fill: path) => {
                    impl<T: $trait> Point<T> {
                        pub fn $function() -> Self {
                            Self {
                                x: $fill(),
                                y: $fill(),
                            }
                        }
                    }
                };
            }

            impl_point_init!(Zero, zeros, T::zero);
            impl_point_init!(One, ones, T::one);
            impl_point_init!(Default, defaults, T::default);

            /// calculate space of triangle
            pub fn triangle_space<T>(o: Point<T>, a: Point<T>, b: Point<T>) -> T
            where
                T: Float,
            {
                let oa = a - o;
                let ob = b - o;
                (oa.x * ob.y - oa.y * ob.x).abs() / T::from(2).unwrap()
            }

            /// calculate gradient  
            /// if zero division occur, return None
            pub fn gradient<T>(a: Point<T>, b: Point<T>) -> Option<T>
            where
                T: Float,
            {
                let x = b - a;
                if x.x == T::zero() {
                    return None;
                }

                Some(x.y / x.x)
            }

            impl<T> Neg for Point<T>
            where
                T: Neg<Output = T>,
            {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    Self {
                        x: -self.x,
                        y: -self.y,
                    }
                }
            }

            macro_rules! impl_point_op {
                ($trait: ident, $function: ident, $op: tt) => {
                    impl<T> $trait<Point<T>> for Point<T>
                    where
                        T: $trait<Output = T>,
                    {
                        type Output = Self;
                        fn $function(self, rhs: Self) -> Self {
                            Self {
                                x: self.x $op rhs.x,
                                y: self.y $op rhs.y,
                            }
                        }
                    }

                    impl<T> $trait<T> for Point<T>
                    where
                        T: $trait<Output = T> + Copy,
                    {
                        type Output = Self;
                        fn $function(self, rhs: T) -> Self {
                            Self {
                                x: self.x $op rhs,
                                y: self.y $op rhs,
                            }
                        }
                    }
                };
            }

            macro_rules! impl_point_assign_op {
                ($trait: ident, $function: ident, $op: tt) => {
                    impl<T> $trait<Point<T>> for Point<T>
                    where
                        T: Copy + NumAssign,
                    {
                        fn $function(&mut self, rhs: Self) {
                            *self = Self {
                                x: self.x $op rhs.x,
                                y: self.y $op rhs.y,
                            }
                        }
                    }

                    impl<T> $trait<T> for Point<T>
                    where
                        T: Copy + NumAssign,
                    {
                        fn $function(&mut self, rhs: T) {
                            *self = Self {
                                x: self.x $op rhs,
                                y: self.y $op rhs,
                            }
                        }
                    }
                };
            }

            impl_point_op!(Add, add, +);
            impl_point_assign_op!(AddAssign, add_assign, +);
            impl_point_op!(Sub, sub, -);
            impl_point_assign_op!(SubAssign, sub_assign, -);
            impl_point_op!(Mul, mul, *);
            impl_point_assign_op!(MulAssign, mul_assign, *);
            impl_point_op!(Div, div, /);
            impl_point_assign_op!(DivAssign, div_assign, /);
        }
    }
}
