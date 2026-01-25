/// a(n) = 2*n^2 + 5*n + 4
/// https://oeis.org/A000749

pub struct A000749;

impl crate::traits::IntegerSequence for A000749 {
    const NAME: &str = "a(n) = 2*n^2 + 5*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 11, 22, 37, 56, 79, 106, 137, 172, 211, 254, 301, 352, 407, 466, 529, 596, 667, 742, 821, 904, 991, 1082, 1177, 1276, 1379, 1486, 1597, 1712, 1831
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000749";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_749(n)
    }
}

const fn quad_749(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 5 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000749>();
}
