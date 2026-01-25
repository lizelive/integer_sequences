/// a(n) = n^3 + 3*n^2 + 0*n + 1
/// https://oeis.org/A000228

pub struct A000228;

impl crate::traits::IntegerSequence for A000228 {
    const NAME: &str = "a(n) = n^3 + 3*n^2 + 0*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 21, 55, 113, 201, 325, 491, 705, 973, 1301, 1695, 2161, 2705, 3333, 4051, 4865, 5781, 6805, 7943, 9201, 10585, 12101, 13755, 15553
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000228";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_228(n)
    }
}

const fn poly_228(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n * n + 0 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000228>();
}
