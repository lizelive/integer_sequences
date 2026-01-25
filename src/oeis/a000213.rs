/// a(n) = n^3 + 3*n^2 + 2*n + 0
/// https://oeis.org/A000213

pub struct A000213;

impl crate::traits::IntegerSequence for A000213 {
    const NAME: &str = "a(n) = n^3 + 3*n^2 + 2*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 6, 24, 60, 120, 210, 336, 504, 720, 990, 1320, 1716, 2184, 2730, 3360, 4080, 4896, 5814, 6840, 7980, 9240, 10626, 12144, 13800, 15600
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000213";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_213(n)
    }
}

const fn poly_213(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n * n + 2 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000213>();
}
