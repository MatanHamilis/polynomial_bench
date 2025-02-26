use std::num::Wrapping;

// Polynomial in Z_{2^64}.
#[derive(Clone)]
pub struct Polynomial {
    coefficients: Vec<Wrapping<u64>>,
}

fn eval_naive(x: Wrapping<u64>, coeff: impl Iterator<Item = Wrapping<u64>>) -> Wrapping<u64> {
    let mut result = Wrapping(0);
    let mut pow = Wrapping(1);
    for coeff in coeff {
        result += coeff * pow;
        pow *= x;
    }
    result
}

// pass reversed coefficients
fn eval_horner(x: Wrapping<u64>, coeff: impl Iterator<Item = Wrapping<u64>>) -> Wrapping<u64> {
    let mut result = Wrapping(0);
    for coeff in coeff {
        result = result * x + coeff;
    }
    result
}

impl Polynomial {
    pub fn eval(&self, x: u64) -> u64 {
        eval_naive(Wrapping(x), self.coefficients.iter().copied()).0
    }
    pub fn eval_horner(&self, x: u64) -> u64 {
        eval_horner(Wrapping(x), self.coefficients.iter().rev().copied()).0
    }
    pub fn random(degree: usize) -> Polynomial {
        let mut coefficients = Vec::with_capacity(degree);
        for _ in 0..degree {
            coefficients.push(rand::random());
        }
        Polynomial { coefficients }
    }

    pub fn reverse(mut self) -> ReversedPolynomial {
        self.coefficients.reverse();
        ReversedPolynomial {
            coefficients: self.coefficients,
        }
    }
}

#[derive(Clone)]
pub struct ReversedPolynomial {
    coefficients: Vec<Wrapping<u64>>,
}

impl ReversedPolynomial {
    pub fn eval(&self, x: u64) -> u64 {
        eval_horner(Wrapping(x), self.coefficients.iter().rev().copied()).0
    }
    pub fn eval_horner(&self, x: u64) -> u64 {
        eval_naive(Wrapping(x), self.coefficients.iter().copied()).0
    }
    pub fn random(degree: usize) -> Polynomial {
        let mut coefficients = Vec::with_capacity(degree);
        for _ in 0..degree {
            coefficients.push(rand::random());
        }
        Polynomial { coefficients }
    }

    pub fn reverse(mut self) -> Polynomial {
        self.coefficients.reverse();
        Polynomial {
            coefficients: self.coefficients,
        }
    }
}

#[test]
fn test_poly_eval() {
    use rand::Rng;
    let mut rng = rand::rng();
    for _ in 0..100 {
        let degree: u16 = rng.random();
        let poly = Polynomial::random(degree as usize);
        for _ in 0..100 {
            let x: u64 = rng.random();
            assert_eq!(poly.eval(x), poly.eval_horner(x));
        }
    }
}
