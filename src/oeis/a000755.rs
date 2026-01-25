/// a(n) = 3*n^2 + 1*n + 1
/// https://oeis.org/A000755

pub struct A000755;

impl crate::traits::IntegerSequence for A000755 {
    const NAME: &str = "a(n) = 3*n^2 + 1*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 15, 31, 53, 81, 115, 155, 201, 253, 311, 375, 445, 521, 603, 691, 785, 885, 991, 1103, 1221, 1345, 1475, 1611, 1753, 1901, 2055, 2215, 2381, 2553
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000755";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_755(n)
    }
}

const fn quad_755(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 1 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000755>();
}
