/// a(n) = 2*n^2 + 4*n + 1
/// https://oeis.org/A000733

pub struct A000733;

impl crate::traits::IntegerSequence for A000733 {
    const NAME: &str = "a(n) = 2*n^2 + 4*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 17, 31, 49, 71, 97, 127, 161, 199, 241, 287, 337, 391, 449, 511, 577, 647, 721, 799, 881, 967, 1057, 1151, 1249, 1351, 1457, 1567, 1681, 1799
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000733";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_733(n)
    }
}

const fn quad_733(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 4 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000733>();
}
