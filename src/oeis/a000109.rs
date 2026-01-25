/// a(n) = n!*n
/// https://oeis.org/A000109

pub struct A000109;

impl crate::traits::IntegerSequence for A000109 {
    const NAME: &str = "a(n) = n!*n";

    const HEAD: &[crate::Value] = &[
        0, 1, 4, 18, 96, 600, 4320, 35280, 322560, 3265920, 36288000, 439084800
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000109";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        factorial_times_n(n)
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

const fn factorial_times_n(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * factorial(n)
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000109>();
}
