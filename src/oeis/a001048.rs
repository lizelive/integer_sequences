/// a(n) = 1*n^3 + 3*n^2 + 2*n
/// https://oeis.org/A001048

pub struct A001048;

impl crate::traits::IntegerSequence for A001048 {
    const NAME: &str = "a(n) = 1*n^3 + 3*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 6, 24, 60, 120, 210, 336, 504, 720, 990, 1320, 1716, 2184, 2730, 3360, 4080, 4896, 5814, 6840, 7980, 9240, 10626, 12144, 13800, 15600, 17550, 19656, 21924, 24360, 26970
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001048";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1048(n)
    }
}

const fn cubic_1048(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n * n + 3 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001048>();
}
