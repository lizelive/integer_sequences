/// a(n) = 2^n - n - 1
/// https://oeis.org/A000118

pub struct A000118;

impl crate::traits::IntegerSequence for A000118 {
    const NAME: &str = "a(n) = 2^n - n - 1";

    const HEAD: &[crate::Value] = &[
        0, 0, 1, 4, 11, 26, 57, 120, 247, 502, 1013, 2036, 4083, 8178, 16369, 32752, 65519
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000118";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        two_pow_minus_n_minus_1(n)
    }
}

const fn two_pow_minus_n_minus_1(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    if n > 60 { return 0; }
    (1isize << n) - n - 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000118>();
}
