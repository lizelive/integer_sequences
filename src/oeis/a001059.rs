/// a(n) = 6*n^3 + 4*n^2 + 2*n
/// https://oeis.org/A001059

pub struct A001059;

impl crate::traits::IntegerSequence for A001059 {
    const NAME: &str = "a(n) = 6*n^3 + 4*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 12, 68, 204, 456, 860, 1452, 2268, 3344, 4716, 6420, 8492, 10968, 13884, 17276, 21180, 25632, 30668, 36324, 42636, 49640, 57372, 65868, 75164, 85296, 96300, 108212, 121068, 134904, 149756
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001059";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1059(n)
    }
}

const fn cubic_1059(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n * n + 4 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001059>();
}
