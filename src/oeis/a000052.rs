/// Number of labeled graphs with n nodes.
/// https://oeis.org/A000052

pub struct A000052;

impl crate::traits::IntegerSequence for A000052 {
    const NAME: &str = "Number of labeled graphs with n nodes";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 8, 64, 1024, 32768, 2097152, 268435456,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000052";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        // 2^C(n,2)
        if n < 0 {
            return 0;
        }
        let edges = n * (n - 1) / 2;
        if edges < 0 {
            return 1;
        }
        1isize << edges
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000052>();
}
