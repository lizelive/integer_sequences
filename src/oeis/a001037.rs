/// a(n) = 2*n^3 + 1*n^2 + 2*n
/// https://oeis.org/A001037

pub struct A001037;

impl crate::traits::IntegerSequence for A001037 {
    const NAME: &str = "a(n) = 2*n^3 + 1*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 5, 24, 69, 152, 285, 480, 749, 1104, 1557, 2120, 2805, 3624, 4589, 5712, 7005, 8480, 10149, 12024, 14117, 16440, 19005, 21824, 24909, 28272, 31925, 35880, 40149, 44744, 49677
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001037";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1037(n)
    }
}

const fn cubic_1037(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n * n + 1 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001037>();
}
