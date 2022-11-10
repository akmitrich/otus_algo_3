use otus_algo_3::tester;

fn main() {
    println!("{}", include_str!("4.Fibo/problem.txt"));
    println!("Solve with golden ratio algo O(log N).");
    tester::run_silently(
        "examples/4.Fibo",
        |data| {
            let n = data.first().unwrap().parse::<u64>().unwrap();
            let fibo = otus_algo_3::fibonacci::fibo_matrix(n);
            fibo.to_string()
        },
    );
}
