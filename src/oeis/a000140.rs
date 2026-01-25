/// a(n) = (n-1)!
/// https://oeis.org/A000140

pub struct A000140;

impl crate::traits::IntegerSequence for A000140 {
    const NAME: &str = "a(n) = (n-1)!";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600, 6227020800, 87178291200, 1307674368000, 20922789888000, 355687428096000, 6402373705728000, 121645100408832000
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000140";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        factorial_n_minus_1(n)
    }
}

fn factorial_n_minus_1(n: crate::Index) -> crate::Value {
    if n <= 0 { return 0; }
    if n == 1 { return 1; }
    
    let mut result = 1isize;
    let mut i = 2isize;
    while i < n {
        result = result.saturating_mul(i);
        i += 1;
    }
    result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000140>();
}
