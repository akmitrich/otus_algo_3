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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, count_primes_naive(10));
    }
}
