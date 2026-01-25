/// Number of representations of n as sum of distinct Fibonacci numbers
/// https://oeis.org/A000119

pub struct A000119;

impl crate::traits::IntegerSequence for A000119 {
    const NAME: &str = "Representations as sums of distinct Fibonacci numbers";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 2, 1, 2, 2, 1, 3, 2, 2, 3, 1, 3, 3, 2, 4, 2, 3, 3, 1, 4, 3, 3, 5, 2, 4, 4, 2, 5,
        3, 3, 4, 1, 4, 4, 3, 6, 3, 5,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000119";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        fib_representations(n)
    }
}

fn fib_representations(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    if n == 0 { return 1; }
    
    const MAX: usize = 1000;
    let n_usize = n as usize;
    if n_usize >= MAX { return 0; }
    
    // Generate Fibonacci numbers starting from F_1=1, F_2=2, F_3=3, F_4=5, ...
    let mut fibs = [0isize; 50];
    fibs[0] = 1;
    fibs[1] = 2;
    let mut fib_count = 2usize;
    while fib_count < 50 && fibs[fib_count - 1] + fibs[fib_count - 2] <= n as isize {
        fibs[fib_count] = fibs[fib_count - 1] + fibs[fib_count - 2];
        fib_count += 1;
    }
    
    // DP: count ways to represent each value
    let mut dp = [0isize; MAX];
    dp[0] = 1;
    
    // Process each Fibonacci number (go backwards to ensure distinctness)
    let mut f = 0;
    while f < fib_count {
        let fib = fibs[f] as usize;
        let mut i = n_usize;
        while i >= fib {
            dp[i] += dp[i - fib];
            i -= 1;
        }
        f += 1;
    }
    
    dp[n_usize]
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000119>();
}
