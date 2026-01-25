/// 2^(2^n): a(n) = 2^a(n-1) with a(0) = 2.
/// https://oeis.org/A000033

pub struct A000033;

impl crate::traits::IntegerSequence for A000033 {
    const NAME: &str = "2^(2^n): Number of Boolean functions of n variables";

    const HEAD: &[crate::Value] = &[
        2, 4, 16, 256, 65536,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000033";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if n < 0 {
            return 0;
        }
        // 2^(2^n)
        let exp = 1isize << n;
        1isize << exp
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000033>();
}
