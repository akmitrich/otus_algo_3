use otus_algo_3::tester;

fn main() {
    println!("{}", include_str!("4.Fibo/problem.txt"));
    println!("Solve with matrix algo O(log N).");
    tester::run_silently(
        "examples/4.Fibo",
        |data| {
            let n = data.first().unwrap().parse::<u64>().unwrap();
            let fibo = otus_algo_3::fibonacci::fibo_matrix(n);
            fibo.to_string()
        },
    );
    println!("\nSolve with golden ratio (Failed at N = 100).");
    tester::run_test_lim(
        "examples/4.Fibo", 
        |data| {
            let n = data.first().unwrap().parse::<u64>().unwrap();
            let fibo = otus_algo_3::fibonacci::fibo_gold(n);
            fibo.to_string()
        }, 
        7,
    );
    println!("\nSolve with iterative algo O(N).");
    tester::run_silently(
        "examples/4.Fibo",
        |data| {
            let n = data.first().unwrap().parse::<u64>().unwrap();
            let fibo = otus_algo_3::fibonacci::fib_iter(n);
            fibo.to_string()
        },
    );
    println!("\nSolve recursively (Failed at N = 100)");
    tester::run_test_lim(
        "examples/4.Fibo", 
        |data| {
            let n = data.first().unwrap().parse::<u8>().unwrap();
            let fibo = otus_algo_3::fibonacci::fib_rec(n);
            fibo.to_string()
        }, 
        6,
    );
    println!("\nSolve recursively with memoization O(N).");
    tester::run_silently(
        "examples/4.Fibo",
        |data| {
            let n = data.first().unwrap().parse::<u64>().unwrap();
            let fibo = otus_algo_3::fibonacci::fib_memo(n);
            fibo.to_string()
        },
    );
}
