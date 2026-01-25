/// a(n) = n^3 + 3*n + 2
/// https://oeis.org/A000523

pub struct A000523;

impl crate::traits::IntegerSequence for A000523 {
    const NAME: &str = "a(n) = n^3 + 3*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 6, 16, 38, 78, 142, 236, 366, 538, 758, 1032, 1366, 1766, 2238, 2788, 3422, 4146, 4966, 5888, 6918, 8062, 9326, 10716, 12238, 13898, 15702, 17656, 19766, 22038, 24478
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000523";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_523(n)
    }
}

const fn poly_523(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000523>();
}
