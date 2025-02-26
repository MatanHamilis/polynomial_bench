use k256::elliptic_curve::PrimeField;
use rand::Rng;
use std::{
    num::Wrapping,
    ops::{Add, AddAssign, Mul, MulAssign},
};

pub trait ScalarTrait:
    Sized
    + Add<Output = Self>
    + AddAssign
    + Mul<Output = Self>
    + for<'a> MulAssign<&'a Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + MulAssign
    + Copy
{
    const ZERO: Self;
    const ONE: Self;
    fn rand(r: &mut impl rand::RngCore) -> Self;
}

// Polynomial in Z_{2^64}.
#[derive(Clone, Debug)]
pub struct Polynomial<S: ScalarTrait> {
    coefficients: Vec<S>,
}

fn eval_naive<S: ScalarTrait>(x: &S, coeff: impl Iterator<Item = S>) -> S {
    let mut result = S::ZERO;
    let mut pow = S::ONE;
    for coeff in coeff {
        result += coeff * pow;
        pow *= x;
    }
    result
}

// pass reversed coefficients
fn eval_horner<S: ScalarTrait>(x: &S, coeff: impl Iterator<Item = S>) -> S {
    let mut result = S::ZERO;
    for coeff in coeff {
        result = result * x + coeff;
    }
    result
}

impl<S: ScalarTrait> Polynomial<S> {
    pub fn eval(&self, x: &S) -> S {
        eval_naive(x.into(), self.coefficients.iter().copied())
    }
    pub fn eval_horner(&self, x: &S) -> S {
        eval_horner(x, self.coefficients.iter().rev().copied())
    }
    pub fn reverse_eval(&self, x: &S) -> S {
        eval_naive(x, self.coefficients.iter().rev().copied())
    }
    pub fn reverse_eval_horner(&self, x: &S) -> S {
        eval_horner(x, self.coefficients.iter().copied())
    }
    pub fn random(degree: usize) -> Self {
        let mut coefficients = Vec::with_capacity(degree);
        let mut rng = rand::rng();
        for _ in 0..degree {
            coefficients.push(S::rand(&mut rng));
        }
        Polynomial { coefficients }
    }

    pub fn reverse(mut self) -> Self {
        self.coefficients.reverse();
        self
    }
}

impl ScalarTrait for Wrapping<u64> {
    const ZERO: Self = Wrapping(0);
    const ONE: Self = Wrapping(1);
    fn rand(r: &mut impl rand::RngCore) -> Self {
        r.random()
    }
}

impl ScalarTrait for Wrapping<u128> {
    const ZERO: Self = Wrapping(0);
    const ONE: Self = Wrapping(1);
    fn rand(r: &mut impl rand::RngCore) -> Self {
        r.random()
    }
}

impl ScalarTrait for k256::Scalar {
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;
    fn rand(r: &mut impl rand::RngCore) -> Self {
        Self::from_repr_vartime(r.random::<[u8; 32]>().into()).unwrap()
    }
}

impl ScalarTrait for curve25519_dalek::Scalar {
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;
    fn rand(r: &mut impl rand::RngCore) -> Self {
        Self::from_bytes_mod_order_wide(&r.random::<[u8; 64]>())
    }
}

#[test]
fn test_poly_eval() {
    use rand::Rng;
    let mut rng = rand::rng();
    for _ in 0..100 {
        let degree: u16 = rng.random();
        let poly = Polynomial::<Wrapping<u64>>::random(degree as usize);
        for _ in 0..100 {
            let x = rng.random();
            assert_eq!(poly.eval(&x), poly.eval_horner(&x));
        }
    }
}
