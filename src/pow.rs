use std::{ops::MulAssign, fmt::Debug};

pub fn pow(base: f64, exp: u64) -> f64 {
    (0..exp).fold(1., |pow, _| pow * base )
}

pub fn pow2(base: f64, exp: u64) -> f64 {
    match exp {
        0 => 1.,
        1 => base,
        _ => {
            let mut current_exp = 2;
            let mut result = base;
            loop {
                result *= result;
                let next = 2 * current_exp;
                if next > exp {
                    break;
                } else {
                    current_exp = next;
                }
            }
            result * pow(base, exp - current_exp)
        }
    }
}

pub fn binary_pow(base: f64, exp: u64) -> f64 {
    let mut d = base;
    let mut p = 1.;
    let mut n = exp;
    while n > 0 {
        if n % 2 > 0 {
            p *= d;
        }
        n /= 2;
        d *= d;
    }
    p
}

pub fn fast_pow<T>(base: T, exp: u64, one: T) -> T
where
    T: MulAssign<T> + Clone + Debug,
{
    let mut d = base;
    let mut p = one;
    let mut n = exp;
    while n > 0 {
        if n % 2 > 0 {
            p *= d.clone();
        }
        n /= 2;
        d *= d.clone();
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1., dbg!(binary_pow(100., 0)));
        assert_eq!(2., dbg!(binary_pow(2., 1)));
        assert_eq!(-8., dbg!(binary_pow(-2., 3)));
        assert_eq!(16., dbg!(binary_pow(-2., 4)));
        assert_eq!(4096., dbg!(binary_pow(-2., 12)));
        assert_eq!(3.1197348228454238e35, dbg!(binary_pow(7., 42)));
        assert_eq!(5.70899077082384e45, dbg!(binary_pow(4., 76)));
    }

    #[test]
    fn test_pow() {
        assert_eq!(1., dbg!(pow(100., 0)));
        assert_eq!(2., dbg!(pow(2., 1)));
        assert_eq!(-8., dbg!(pow(-2., 3)));
        assert_eq!(16., dbg!(pow(-2., 4)));
        assert_eq!(4096., dbg!(pow(-2., 12)));
        assert_eq!(3.119734822845424e35, dbg!(pow(7., 42)));
        assert_eq!(5.70899077082384e45, dbg!(pow(4., 76)));
    }

    #[test]
    fn test_pow2() {
        assert_eq!(1., dbg!(pow2(100., 0)));
        assert_eq!(2., dbg!(pow2(2., 1)));
        assert_eq!(-8., dbg!(pow2(-2., 3)));
        assert_eq!(16., dbg!(pow2(-2., 4)));
        assert_eq!(4096., dbg!(pow2(-2., 12)));
        assert_eq!(3.1197348228454238e35, dbg!(pow2(7., 42)));
        assert_eq!(5.70899077082384e45, dbg!(pow2(4., 76)));
    }
}
