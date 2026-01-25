/// a(n) = 3*n^2 + 3*n + 3
/// https://oeis.org/A000767

pub struct A000767;

impl crate::traits::IntegerSequence for A000767 {
    const NAME: &str = "a(n) = 3*n^2 + 3*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 9, 21, 39, 63, 93, 129, 171, 219, 273, 333, 399, 471, 549, 633, 723, 819, 921, 1029, 1143, 1263, 1389, 1521, 1659, 1803, 1953, 2109, 2271, 2439, 2613
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000767";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_767(n)
    }
}

const fn quad_767(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 3 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000767>();
}
