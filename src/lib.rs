use std::num::Wrapping;

// Polynomial in Z_{2^64}.
pub struct Polynomial {
    coefficients: Vec<Wrapping<u64>>,
}

impl Polynomial {
    pub fn eval(&self, x: u64) -> u64 {
        let mut result = Wrapping(0);
        let mut pow = Wrapping(1);
        for &coeff in &self.coefficients {
            result += coeff * pow;
            pow *= x;
        }
        result.0
    }
    pub fn eval_horner(&self, x: u64) -> u64 {
        let mut result = Wrapping(0);
        let x = Wrapping(x);
        for &coeff in self.coefficients.iter().rev() {
            result = result * x + coeff;
        }
        result.0
    }
    pub fn random(degree: usize) -> Polynomial {
        let mut coefficients = Vec::with_capacity(degree);
        for _ in 0..degree {
            coefficients.push(rand::random());
        }
        Polynomial { coefficients }
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
