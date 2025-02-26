// Polynomial in Z_{2^64}.
pub struct Polynomial {
    coefficients: Vec<u64>,
}

impl Polynomial {
    pub fn eval(&self, x: u64) -> u64 {
        let mut result = 0;
        let mut pow = 1;
        for &coeff in &self.coefficients {
            result += coeff * pow;
            pow = unsafe { pow.unchecked_mul(x) };
        }
        result
    }
    pub fn eval_horner(&self, x: u64) -> u64 {
        let mut result = 0;
        for &coeff in self.coefficients.iter().rev() {
            result = result * x + coeff;
        }
        result
    }
    pub fn random(degree: usize) -> Polynomial {
        let mut coefficients = Vec::with_capacity(degree);
        for _ in 0..degree {
            coefficients.push(rand::random());
        }
        Polynomial { coefficients }
    }
}
