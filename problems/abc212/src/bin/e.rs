#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use competitive_internal_mod::format::*;
use competitive_internal_mod::mod_int::*;
use itertools::{iproduct, Itertools};
use itertools_num::ItertoolsNum;
use num::*;
use num_traits::*;
use proconio::{fastout, input, marker::*};
use std::{collections::*, ops::*};
use superslice::*;
use utils::*;

const MOD: usize = 998244353;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

#[fastout]
fn run() -> impl AtCoderFormat {
    input! {
        n: usize, m: usize, k: usize,
        g: [(usize, usize); m]
    }
    let mb = ModIntBuilder::new(MOD);
    let mut e = vec![vec![]; n];

    // 通れなくなった部分の経路を保存
    for (u, v) in g.into_iter().map(|(u, v)| (u - 1, v - 1)) {
        e[u].push(v);
        e[v].push(u);
    }

    // dp[day][pos] dayの段階でposにいるときの通り数
    let mut dp = vec![vec![mb.build(0); n + 1]; k + 1];
    dp[0][0] = mb.build(1);

    for day in 0..k {
        // すべての道が完全だった場合の通り数
        let mut tot = mb.build(0);
        for p in 0..n {
            tot += dp[day][p];
        }

        debug!(tot);

        // 次の日の通り数
        for p in 0..n {
            // 通れない場所の数
            let mut etot = mb.build(0);
            for &next in e[p].iter() {
                etot += dp[day][next];
            }
            debug!(etot);
            dp[day + 1][p] = dp[day + 1][p] + tot - dp[day][p] - etot;
        }
    }

    dp[k][0].value()
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
    pub mod mod_int {
        use std::{
            borrow::Borrow,
            iter::Sum,
            ops::{
                Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, RemAssign, ShrAssign, Sub,
                SubAssign,
            },
        };

        use itertools::Itertools;
        use num_traits::Zero;

        #[derive(Debug)]
        pub struct ModIntBuilder<T> {
            m: T,
        }

        impl<T: Copy> ModIntBuilder<T> {
            pub fn new(modulo: T) -> Self {
                Self { m: modulo }
            }

            pub fn build(&self, v: T) -> ModInt<T> {
                ModInt::new_unchecked(v, self.m)
            }
        }

        #[derive(Debug)]
        pub struct ModInt<T> {
            v: T,
            m: T,
        }

        impl<T> ModInt<T>
        where
            T: Copy,
        {
            pub fn value(&self) -> T {
                self.v
            }
            pub fn modulo(&self) -> T {
                self.m
            }
        }

        impl<T> ModInt<T> {
            fn new_unchecked(v: T, modulo: T) -> Self {
                Self { v, m: modulo }
            }
        }

        impl<T> ModInt<T>
        where
            T: Copy + RemAssign + PartialOrd,
        {
            pub fn new(mut v: T, modulo: T) -> Self {
                if v >= modulo {
                    v %= modulo;
                }
                Self::new_unchecked(v, modulo)
            }
        }

        impl<T> ModInt<T>
        where
            T: Copy
                + Sub<Output = T>
                + ShrAssign
                + BitAnd<Output = T>
                + PartialEq
                + PartialOrd
                + Div<Output = T>
                + RemAssign,
            ModInt<T>: MulAssign,
        {
            pub fn pow(self, e: T) -> Self {
                let zero = self.modulo() - self.modulo();
                let one = self.modulo() / self.modulo();
                let mut e = e;
                let mut result = Self::new_unchecked(one, self.modulo());
                let mut cur = self;
                while e > zero {
                    if e & one == one {
                        result *= cur;
                    }
                    e >>= one;
                    cur *= cur;
                }
                result
            }
        }

        impl<T> Copy for ModInt<T> where T: Copy {}
        impl<T> Clone for ModInt<T>
        where
            T: Copy,
        {
            fn clone(&self) -> Self {
                Self::new_unchecked(self.value(), self.modulo())
            }
        }

        impl<T> Add<T> for ModInt<T>
        where
            T: AddAssign + SubAssign + RemAssign + Copy + PartialOrd,
        {
            type Output = Self;
            fn add(self, mut rhs: T) -> Self::Output {
                if rhs >= self.modulo() {
                    rhs %= self.modulo();
                }
                rhs += self.value();
                if rhs >= self.modulo() {
                    rhs -= self.modulo();
                }
                Self::new_unchecked(rhs, self.modulo())
            }
        }

        impl<T> Sub<T> for ModInt<T>
        where
            T: AddAssign + SubAssign + RemAssign + Copy + PartialOrd,
        {
            type Output = Self;
            fn sub(self, mut rhs: T) -> Self::Output {
                if rhs >= self.modulo() {
                    rhs %= self.modulo();
                }

                let mut result = self.value();
                result += self.modulo();
                result -= rhs;

                if result >= self.modulo() {
                    result -= self.modulo();
                }
                Self::new_unchecked(result, self.modulo())
            }
        }

        impl<T> Mul<T> for ModInt<T>
        where
            T: MulAssign + RemAssign + Copy + PartialOrd,
        {
            type Output = Self;
            fn mul(self, mut rhs: T) -> Self::Output {
                if rhs >= self.modulo() {
                    rhs %= self.modulo();
                }
                rhs *= self.value();
                rhs %= self.modulo();
                Self::new_unchecked(rhs, self.modulo())
            }
        }

<<<<<<< HEAD
        impl<T> Add<ModInt<T>> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Add<T, Output = ModInt<T>>,
        {
            type Output = Self;
            fn add(self, rhs: ModInt<T>) -> Self::Output {
                self + rhs.value()
            }
        }
        impl<T> Sub<ModInt<T>> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Sub<T, Output = ModInt<T>>,
        {
            type Output = Self;
            fn sub(self, rhs: ModInt<T>) -> Self::Output {
                self - rhs.value()
            }
        }
        impl<T> Mul<ModInt<T>> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Mul<T, Output = ModInt<T>>,
        {
            type Output = Self;
            fn mul(self, rhs: ModInt<T>) -> Self::Output {
                self * rhs.value()
            }
        }
        impl<T> Div<ModInt<T>> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Div<T, Output = ModInt<T>>,
        {
            type Output = Self;
            fn div(self, rhs: ModInt<T>) -> Self::Output {
                self / rhs.value()
            }
        }

        impl<T> AddAssign<T> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Add<T, Output = ModInt<T>>,
        {
            fn add_assign(&mut self, other: T) {
                *self = *self + other;
            }
        }
        impl<T> AddAssign<ModInt<T>> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Add<ModInt<T>, Output = ModInt<T>>,
        {
            fn add_assign(&mut self, other: ModInt<T>) {
                *self = *self + other;
            }
        }

        impl<T> SubAssign<T> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Sub<T, Output = ModInt<T>>,
        {
            fn sub_assign(&mut self, other: T) {
                *self = *self - other;
            }
        }

        impl<T> SubAssign<ModInt<T>> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Sub<ModInt<T>, Output = ModInt<T>>,
        {
            fn sub_assign(&mut self, other: ModInt<T>) {
                *self = *self - other;
            }
        }

        impl<T> DivAssign<T> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Div<T, Output = ModInt<T>>,
        {
            fn div_assign(&mut self, rhs: T) {
                *self = *self / rhs
            }
        }
        impl<T> DivAssign<ModInt<T>> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Div<ModInt<T>, Output = ModInt<T>>,
        {
            fn div_assign(&mut self, rhs: ModInt<T>) {
                *self = *self / rhs
            }
        }

        impl<T> MulAssign<T> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Mul<T, Output = ModInt<T>>,
        {
            fn mul_assign(&mut self, rhs: T) {
                *self = *self * rhs;
            }
        }

        impl<T> MulAssign<ModInt<T>> for ModInt<T>
        where
            T: Copy,
            ModInt<T>: Mul<ModInt<T>, Output = ModInt<T>>,
        {
            fn mul_assign(&mut self, rhs: ModInt<T>) {
                *self = *self * rhs;
            }
        }
=======
        macro_rules! impl_mod_op {
            ($trait: ident, $function: ident, $op: tt) => {
                impl<T> $trait<ModInt<T>> for ModInt<T>
                where
                    T: Copy,
                    ModInt<T>: $trait<T, Output = ModInt<T>>,
                {
                    type Output = Self;
                    fn $function(self, rhs: ModInt<T>) -> Self::Output {
                        self $op rhs.value()
                    }
                }
            };
        }

        impl_mod_op!(Add, add, +);
        impl_mod_op!(Sub, sub, -);
        impl_mod_op!(Mul, mul, *);
        impl_mod_op!(Div, div, /);

        macro_rules! impl_mod_assign {
            ($trait: ident, $output_trait: ident, $function: ident, $op: tt) => {
                impl<T> $trait<T> for ModInt<T>
                where
                    T: Copy,
                    ModInt<T>: $output_trait<T, Output = ModInt<T>>
                {
                    fn $function(&mut self, other: T) {
                        *self = *self $op other;
                    }
                }

                impl<T> $trait<ModInt<T>> for ModInt<T>
                where
                    T: Copy,
                    ModInt<T>: $output_trait<ModInt<T>, Output = ModInt<T>>
                {
                    fn $function(&mut self, other: ModInt<T>) {
                        *self = *self $op other;
                    }
                }
            };
        }

        impl_mod_assign!(AddAssign, Add, add_assign, +);
        impl_mod_assign!(SubAssign, Sub, sub_assign, -);
        impl_mod_assign!(DivAssign, Div, div_assign, /);
        impl_mod_assign!(MulAssign, Mul, mul_assign, *);
>>>>>>> 5aa6ccac2630f952ad5abc5ea07713b50951a4f4

        impl<T> Div<T> for ModInt<T>
        where
            T: Copy
                + Add<Output = T>
                + Sub<Output = T>
                + Div<Output = T>
                + BitAnd<Output = T>
                + PartialEq
                + PartialOrd
                + SubAssign
                + ShrAssign
                + RemAssign
                + MulAssign,
        {
            type Output = Self;
            fn div(self, mut rhs: T) -> Self::Output {
                if rhs >= self.modulo() {
                    rhs %= self.modulo();
                }
                let one = self.modulo() / self.modulo();
                let two = one + one;
                self * Self::new_unchecked(rhs, self.modulo()).pow(self.modulo() - two)
            }
        }
    }
}
