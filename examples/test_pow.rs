use otus_algo_3::tester;
fn main() {
    println!("{}", include_str!("3.Power/problem.txt"));
    println!("Solve with binary exp algo O(log N).");
    tester::run_test("examples/3.Power", |data| {
        let base = data.first().unwrap().parse::<f64>().unwrap();
        let exp = data.last().unwrap().parse::<u64>().unwrap();
        format!("{}", otus_algo_3::pow::binary_pow(base, exp))
    });
    println!("\nSolve with advanced iterative algo O(N/2+log N) = O(N).");
    tester::run_test("examples/3.Power", |data| {
        let base = data.first().unwrap().parse::<f64>().unwrap();
        let exp = data.last().unwrap().parse::<u64>().unwrap();
        format!("{}", otus_algo_3::pow::pow2(base, exp))
    });
    println!("\nSolve with iterative algo O(N).");
    tester::run_test("examples/3.Power", |data| {
        let base = data.first().unwrap().parse::<f64>().unwrap();
        let exp = data.last().unwrap().parse::<u64>().unwrap();
        format!("{}", otus_algo_3::pow::pow(base, exp))
    });
}
