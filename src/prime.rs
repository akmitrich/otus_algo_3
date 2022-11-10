pub fn count_primes_naive(n: u64) -> u64 {
    let mut count = 0;
    for i in 1..=n {
        if count_divisors(i) == 2 {
            count += 1;
        }
    }
    count
}

fn count_divisors(number: u64) -> u64 {
    let mut count = 0;
    for i in 1..=number {
        if number % i == 0 {
            count += 1;
        }
    }
    count
}

pub fn get_prime_count(n: u64) -> usize {
    let mut primes = Vec::new();
    for p in 2..=n {
        if is_prime(p, &primes) {
            primes.push(p);
        }
    }
    primes.len()
}

fn is_prime(p: u64, primes: &[u64]) -> bool {
    match p {
        0 => panic!("Unexpected p = {p}"),
        1 => false,
        2 => true,
        p if p % 2 == 0 => false,
        _ => {
            let sqrt = (p as f64).sqrt().floor() as u64;
            for divisor in primes.iter() {
                if divisor > &sqrt {
                    break;
                } else if p % divisor == 0 {
                    return false;
                } else {
                    continue;
                }
            }
            true
        }
    }
}

pub fn prime_sieve(n: usize) -> usize {
    let mut sieve: Vec<bool> = vec![true; n];
    sieve[0] = false;
    for p in 2..n {
        if sieve[p - 1] {
            expunge(p, &mut sieve);
        }
    }
    sieve.iter().filter(|p| **p).count()
}

fn expunge(p: usize, sieve: &mut[bool]) {
    let mut mult = p + p;
    while mult <= sieve.len() {
        sieve[mult - 1] = false;
        mult += p;
    }
}

pub fn linear_sieve(n: usize) -> usize {
    let mut pr = vec![];
    let mut lp = vec![0; n + 1];
    for i in 2..=n {
        if lp[i] == 0 {
            lp[i] = i;
            pr.push(i);
        }
        for p in pr.iter() {
            if p > &lp[i] || p * i > n { break; }
            lp[p * i] = *p;
        }
    }
    pr.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, count_primes_naive(10));
    }

    #[test]
    fn test_get_count() {
        assert_eq!(4, get_prime_count(10));
        assert_eq!(25, get_prime_count(100));
        assert_eq!(168, get_prime_count(1000));
        assert_eq!(1229, get_prime_count(10_000));
        assert_eq!(9592, get_prime_count(100_000));
        assert_eq!(78_498, get_prime_count(1_000_000));
        assert_eq!(664_579, get_prime_count(10_000_000));
    }

    #[test]
    fn test_sieve() {
        assert_eq!(4, prime_sieve(10));
        assert_eq!(25, prime_sieve(100));
        assert_eq!(168, prime_sieve(1000));
        assert_eq!(1229, prime_sieve(10_000));
        assert_eq!(9592, prime_sieve(100_000));
        assert_eq!(78_498, prime_sieve(1_000_000));
        assert_eq!(664_579, prime_sieve(10_000_000));
        assert_eq!(5_761_455, prime_sieve(100_000_000));
        assert_eq!(50_847_534, prime_sieve(1_000_000_000));
    }

    #[test]
    fn test_linear_sieve() {
        assert_eq!(4, linear_sieve(10));
        assert_eq!(25, linear_sieve(100));
        assert_eq!(168, linear_sieve(1000));
        assert_eq!(1229, linear_sieve(10_000));
        assert_eq!(9592, linear_sieve(100_000));
        assert_eq!(78_498, linear_sieve(1_000_000));
        assert_eq!(664_579, linear_sieve(10_000_000));
        assert_eq!(5_761_455, linear_sieve(100_000_000));
//        assert_eq!(50_847_534, linear_sieve(1_000_000_000));
    }
}
