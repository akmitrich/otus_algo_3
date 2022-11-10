use num::{BigUint, FromPrimitive, One, ToPrimitive, Zero, bigint::ToBigUint};
use std::{collections::HashMap, ops::MulAssign};

#[derive(Debug)]
pub struct Fibo {
    a: [BigUint; 4],
}

impl Default for Fibo {
    fn default() -> Self {
        Self {
            a: [
                BigUint::one(),
                BigUint::one(),
                BigUint::one(),
                BigUint::zero(),
            ],
        }
    }
}

impl Clone for Fibo {
    fn clone(&self) -> Self {
        Self { a: self.a.clone() }
    }
}

impl MulAssign<Fibo> for Fibo {
    fn mul_assign(&mut self, rhs: Fibo) {
        let a = self.a.clone();
        self.a[0] = &a[0] * &rhs.a[0] + &a[1] * &rhs.a[2];
        self.a[1] = &a[0] * &rhs.a[1] + &a[1] * &rhs.a[3];
        self.a[2] = &a[2] * &rhs.a[0] + &a[3] * &rhs.a[2];
        self.a[3] = &a[2] * &rhs.a[1] + &a[3] * &rhs.a[3];
    }
}

impl Fibo {
    pub fn unit() -> Self {
        Self {
            a: [
                BigUint::one(),
                BigUint::zero(),
                BigUint::zero(),
                BigUint::one(),
            ],
        }
    }

    pub fn number(&self) -> BigUint {
        self.a[0].clone()
    }
}

pub fn fib_rec(n: u8) -> u128 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fib_rec(n - 1) + fib_rec(n - 2)
}

pub fn fib_memo(n: u64) -> BigUint {
    let mut memo = HashMap::new();
    memo.insert(0, BigUint::zero());
    memo.insert(1, BigUint::one());
    fib_memo_impl(n, &mut memo)
}

fn fib_memo_impl(n: u64, memo: &mut HashMap<u64, BigUint>) -> BigUint {
    if let Some(result) = memo.get(&n) {
        return result.clone();
    }
    let result = fib_memo_impl(n - 1, memo) + fib_memo_impl(n - 2, memo);
    memo.insert(n, result.clone());
    result
}

pub fn fib_iter(n: u64) -> BigUint {
    if n == 1 {
        return BigUint::from_u8(1).unwrap();
    }
    let mut result = BigUint::from_u128(0).unwrap();
    let mut prev = BigUint::from_u128(1).unwrap();
    let mut prevprev = BigUint::from_u128(0).unwrap();
    for _ in 1..n {
        result = &prevprev + &prev;
        prevprev = prev.clone();
        prev = result.clone();
    }
    result
}

pub fn fibo_gold(n: u64) -> u128 {
    let sqrt5 = 5_f64.sqrt();
    let phi: f64 = (1. + sqrt5) / 2.;
    let fibo = crate::pow::binary_pow(phi, n) / sqrt5 + 0.5;
    println!("fibo = {fibo}");
    fibo.floor().to_u128().unwrap()
}

pub fn fibo_matrix(n: u64) -> BigUint {
    match n {
        0 | 1 => n.to_biguint().unwrap(),
        _ => crate::pow::fast_pow(Fibo::default(), n - 1, Fibo::unit()).number()
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_rec() {
        assert_eq!(0, fib_rec(0));
        assert_eq!(1, fib_rec(1));
        assert_eq!(55, dbg!(fib_rec(10)));
        //        assert_eq!(12586269025, dbg!(fib_rec(50))); // about 100 s
        //        assert_eq!(354224848179261915075, dbg!(fib_rec(100))) // almost impossible to calculate
    }

    #[test]
    fn test_fib_iter() {
        assert_eq!("0", fib_iter(0).to_string());
        assert_eq!("1", fib_iter(1).to_string());
        assert_eq!("55", dbg!(fib_iter(10).to_string()));
        assert_eq!("12586269025", dbg!(fib_iter(50).to_string()));
        assert_eq!("354224848179261915075", dbg!(fib_iter(100).to_string()));
        assert_eq!(
            "332825110087067562321196029789634457848",
            dbg!(fib_iter(186).to_string())
        );
    }
}
