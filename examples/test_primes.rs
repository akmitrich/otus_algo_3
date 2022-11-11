use otus_algo_3::tester;

fn main() {
    println!("{}", include_str!("5.Primes/problem.txt"));
    println!("Solve with simple search.");
    tester::run_test_lim(
        "examples/5.Primes", 
        |data| {
            let n = data.first().unwrap().parse::<u64>().unwrap();
            let number_of_primes = otus_algo_3::prime::count_primes_naive(n);
            number_of_primes.to_string()
        }, 
        9,
    );
    println!("\nSolve with optimized search.");
    tester::run_test_lim(
        "examples/5.Primes", 
        |data| {
            let n = data.first().unwrap().parse::<u64>().unwrap();
            let number_of_primes = otus_algo_3::prime::get_prime_count(n);
            number_of_primes.to_string()
        }, 
        13,
    );
    println!("\nSolve with optimized memory Erathos sieve.");
    tester::run_test(
        "examples/5.Primes",
        |data| {
            let n = data.first().unwrap().parse::<usize>().unwrap();
            let number_of_primes = otus_algo_3::prime::effective_sieve(n);
            number_of_primes.to_string()
        }
    );
    println!("\nSolve with Erathos sieve.");
    tester::run_test(
        "examples/5.Primes",
        |data| {
            let n = data.first().unwrap().parse::<usize>().unwrap();
            let number_of_primes = otus_algo_3::prime::prime_sieve(n);
            number_of_primes.to_string()
        }
    );
    println!("\nSolve with linear sieve algo O(N).");
    tester::run_test_lim(
        "examples/5.Primes", 
        |data| {
            let n = data.first().unwrap().parse::<usize>().unwrap();
            let number_of_primes = otus_algo_3::prime::linear_sieve(n);
            number_of_primes.to_string()
        },
        14, 
    );
}