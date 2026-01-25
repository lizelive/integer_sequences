/// a(n) = n!.
/// https://oeis.org/A000142

pub struct A000142;

impl crate::traits::IntegerSequence for A000142 {
    const NAME: &str = "Factorial numbers: n!";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600, 6227020800,
        87178291200, 1307674368000, 20922789888000, 355687428096000, 6402373705728000,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000142";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        factorial(n)
    }
}

const fn factorial(n: crate::Index) -> crate::Value {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 1;
    }
    let mut result = 1isize;
    let mut i = 1;
    while i <= n {
        result *= i;
        i += 1;
    }
    result
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000142>();
}
