/// a(n) = 3*n^2 + 4
/// https://oeis.org/A000942

pub struct A000942;

impl crate::traits::IntegerSequence for A000942 {
    const NAME: &str = "a(n) = 3*n^2 + 4";

    const HEAD: &[crate::Value] = &[
        4, 7, 16, 31, 52, 79, 112, 151, 196, 247, 304, 367, 436, 511, 592, 679, 772, 871, 976, 1087, 1204, 1327, 1456, 1591, 1732, 1879, 2032, 2191, 2356, 2527
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000942";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_942(n)
    }
}

const fn sq_942(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 4 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000942>();
}
