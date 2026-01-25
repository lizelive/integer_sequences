/// a(n) = (n+1)! = A000142(n+1)
/// https://oeis.org/A000110

pub struct A000110;

impl crate::traits::IntegerSequence for A000110 {
    const NAME: &str = "a(n) = (n+1)!";

    const HEAD: &[crate::Value] = &[
        1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000110";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        factorial_n_plus_1(n)
    }
}

const fn factorial(n: isize) -> isize {
    if n <= 1 { return 1; }
    let mut result = 1isize;
    let mut i = 2isize;
    while i <= n {
        result = result.saturating_mul(i);
        i += 1;
    }
    result
}

const fn factorial_n_plus_1(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    factorial(n + 1)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000110>();
}
