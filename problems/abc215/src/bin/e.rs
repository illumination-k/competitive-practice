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

use competitive_internal_mod::modint::*;

const MOD: usize = 998244353;
const UINF: usize = std::usize::MAX;
const IINF: isize = std::isize::MAX;

#[fastout]
fn run<R: BufRead>(source: AutoSource<R>) -> impl AtCoderFormat {
    input! {
        from &mut source,
        n: usize, s: Chars,
    }

    // mod数え上げ -> dpの可能性がある
    // dp + bit dp
    // 今まで出たコンテストをbitで表す
    // dp[i][bit][x] i: コンテストi回目, bit: 今まで出たコンテンストの種類, x: 最後のコンテスト
    let mb = ModIntBuilder::new(MOD);
    let mut dp = vec![vec![vec![mb.build(0); 10]; 1 << 10]; 1 << 10];

    for i in 1..=n {
        let contest = s[i - 1] as usize - 'A' as usize;

        for bit in 0..1 << 10 {
            for x in 0..10 {
                dp[i][bit][x] = dp[i - 1][bit][x]; // そのまま
                if x == contest {
                    dp[i][bit][x] = dp[i - 1][bit][x] + dp[i - 1][bit][x];
                }
            }
        }

        for bit in 0..1 << 10 {
            if bit & 1 << contest != 0 {
                continue;
            }
            for x in 0..10 {
                // debug!(dp[i][bit|1<<contest][contest]);
                dp[i][bit | 1 << contest][contest] =
                    dp[i][bit | (1 << contest)][contest] + dp[i - 1][bit][x];
            }
        }

        dp[i][1 << contest][contest] += mb.build(1);
        // debug!(dp[i][1<<contest][contest]);
    }

    let mut ans = mb.build(0);
    for bit in 0..1 << 10 {
        for x in 0..10 {
            ans += dp[n][bit][x];
        }
    }

    ans.value()
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
        let s = AutoSource::from("4\nBBGH\n");
        run(s);
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
    pub mod modint {
        use std::{
            iter::{Product, Sum},
            ops::{
                Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, RemAssign, ShrAssign, Sub,
                SubAssign,
            },
        };

        use num_traits::{One, Zero};

        #[derive(Debug)]
        pub struct ModIntBuilder<T> {
            m: T,
        }

        impl<T> ModIntBuilder<T> {
            pub fn new(modulo: T) -> Self {
                Self { m: modulo }
            }
        }

        impl<T> ModIntBuilder<T>
        where
            T: RemAssign + Copy + PartialOrd,
        {
            pub fn build(&self, v: T) -> ModInt<T> {
                ModInt::new(v, Some(self.m))
            }
        }

        #[derive(Debug)]
        pub struct ModInt<T> {
            value: T,
            modulo: Option<T>,
        }

        impl<T> ModInt<T>
        where
            T: Copy,
        {
            pub fn value(&self) -> T {
                self.value
            }
            pub fn modulo(&self) -> Option<T> {
                self.modulo
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

        impl<T> PartialEq for ModInt<T>
        where
            T: PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        macro_rules! impl_mod_partialeq {
            ($t: ty) => {
                impl PartialEq<$t> for ModInt<$t> {
                    fn eq(&self, other: &$t) -> bool {
                        &self.value == other
                    }
                }
            };
        }

        impl_mod_partialeq!(usize);
        impl_mod_partialeq!(u128);
        impl_mod_partialeq!(u64);
        impl_mod_partialeq!(u32);

        impl<T> ModInt<T> {
            fn new_unchecked(value: T, modulo: Option<T>) -> Self {
                Self { value, modulo }
            }
        }

        impl<T> ModInt<T>
        where
            T: Copy + RemAssign + PartialOrd,
        {
            pub fn new(mut value: T, modulo: Option<T>) -> Self {
                if let Some(m) = modulo {
                    if value >= m {
                        value %= m
                    }
                }
                Self::new_unchecked(value, modulo)
            }
        }

        macro_rules! impl_mod_init {
            ($trait: ident, $function: ident, $fill: path) => {
                impl<T> ModInt<T>
                where
                    T: $trait,
                {
                    pub fn $function() -> Self {
                        Self::new_unchecked($fill(), None)
                    }
                }
            };
        }

        impl_mod_init!(Zero, zeros, T::zero);
        impl_mod_init!(One, ones, T::one);
        impl_mod_init!(Default, defaults, T::default);

        impl<T> ModInt<T>
        where
            T: Copy
                + Sub<Output = T>
                + ShrAssign
                + BitAnd<Output = T>
                + PartialEq
                + PartialOrd
                + Div<Output = T>
                + RemAssign
                + Zero
                + One,
            ModInt<T>: MulAssign,
        {
            pub fn pow(self, e: T) -> Self {
                let zero = T::zero();
                let one = T::one();
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

        impl<T> Add<T> for ModInt<T>
        where
            T: AddAssign + SubAssign + RemAssign + Copy + PartialOrd,
        {
            type Output = Self;
            fn add(self, mut rhs: T) -> Self::Output {
                if let Some(modulo) = self.modulo() {
                    if rhs >= modulo {
                        rhs %= modulo;
                    }
                    rhs += self.value();
                    if rhs >= modulo {
                        rhs -= modulo;
                    }
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
                let mut result = self.value();
                if let Some(modulo) = self.modulo() {
                    if rhs >= modulo {
                        rhs %= modulo;
                    }
                    result += modulo;
                    result -= rhs;

                    if result >= modulo {
                        result -= modulo;
                    }
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
                if let Some(modulo) = self.modulo() {
                    if rhs >= modulo {
                        rhs %= modulo;
                    }
                    rhs *= self.value();
                    rhs %= modulo;
                }
                Self::new_unchecked(rhs, self.modulo())
            }
        }

        macro_rules! impl_mod_op {
            ($trait: ident, $function: ident, $op: tt) => {
                impl<T> $trait<ModInt<T>> for ModInt<T>
                where
                    T: Copy,
                    ModInt<T>: $trait<T, Output = ModInt<T>>,
                {
                    type Output = Self;
                    fn $function(self, rhs: ModInt<T>) -> Self::Output {
                        if self.modulo.is_none() && rhs.modulo.is_some() {
                            let mut s = self;
                            s.modulo = rhs.modulo();
                            s $op rhs.value()
                        } else {
                            self $op rhs.value()
                        }
                    }
                }
            };
        }

        impl_mod_op!(Add, add, +);
        impl_mod_op!(Sub, sub, -);
        impl_mod_op!(Mul, mul, *);

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
        impl_mod_assign!(MulAssign, Mul, mul_assign, *);

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
                + MulAssign
                + Zero
                + One,
        {
            type Output = Self;
            fn div(self, mut rhs: T) -> Self::Output {
                if let Some(modulo) = self.modulo() {
                    if rhs >= modulo {
                        rhs %= modulo;
                    }
                    let one = T::one();
                    let two = one + one;
                    self * Self::new_unchecked(rhs, self.modulo()).pow(modulo - two)
                } else {
                    Self::new_unchecked(self.value() / rhs, self.modulo())
                }
            }
        }

        impl_mod_op!(Div, div, /);
        impl_mod_assign!(DivAssign, Div, div_assign, /);

        macro_rules! impl_mod_forward_ref_binop {
            ($imp: ident, $method: ident) => {
                impl<'a, T> $imp<ModInt<T>> for &'a ModInt<T>
                where
                    T: Copy,
                    ModInt<T>: $imp,
                {
                    type Output = <ModInt<T> as $imp>::Output;

                    #[inline]
                    fn $method(self, rhs: ModInt<T>) -> <ModInt<T> as $imp>::Output {
                        $imp::$method(*self, rhs)
                    }
                }

                impl<T> $imp<&ModInt<T>> for ModInt<T>
                where
                    T: Copy,
                    ModInt<T>: $imp,
                {
                    type Output = <ModInt<T> as $imp>::Output;

                    #[inline]
                    fn $method(self, rhs: &ModInt<T>) -> <ModInt<T> as $imp>::Output {
                        $imp::$method(self, *rhs)
                    }
                }

                impl<'a, T> $imp<&'a ModInt<T>> for &'a ModInt<T>
                where
                    T: Copy,
                    ModInt<T>: $imp,
                {
                    type Output = <ModInt<T> as $imp>::Output;

                    #[inline]
                    fn $method(self, rhs: &ModInt<T>) -> <ModInt<T> as $imp>::Output {
                        $imp::$method(*self, *rhs)
                    }
                }
            };
        }

        impl_mod_forward_ref_binop!(Add, add);
        impl_mod_forward_ref_binop!(Sub, sub);
        impl_mod_forward_ref_binop!(Mul, mul);
        impl_mod_forward_ref_binop!(Div, div);

        impl<T> Sum for ModInt<T>
        where
            T: Copy + Zero + Add<Output = T>,
            ModInt<T>: Add<Output = Self>,
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(ModInt::zeros(), |a, b| a + b)
            }
        }

        impl<'a, T> Sum<&'a ModInt<T>> for ModInt<T>
        where
            T: Copy + Zero + Add<Output = T>,
            ModInt<T>: Add<Output = Self>,
        {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(ModInt::zeros(), |a, &b| a + b)
            }
        }

        impl<T> Product for ModInt<T>
        where
            T: Copy + One + Add<Output = T>,
            ModInt<T>: Mul<Output = Self>,
        {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(ModInt::ones(), |a, b| a * b)
            }
        }

        impl<'a, T> Product<&'a ModInt<T>> for ModInt<T>
        where
            T: Copy + One + Add<Output = T>,
            ModInt<T>: Mul<Output = Self>,
        {
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(ModInt::ones(), |a, &b| a * b)
            }
        }

        impl<T> From<T> for ModInt<T>
        where
            T: Copy + RemAssign + PartialOrd,
        {
            fn from(from: T) -> Self {
                ModInt::new(from, None)
            }
        }
    }
}
