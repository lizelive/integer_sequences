/// a(n) = 2*n^2 + 1*n + 1
/// https://oeis.org/A000730

pub struct A000730;

impl crate::traits::IntegerSequence for A000730 {
    const NAME: &str = "a(n) = 2*n^2 + 1*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 4, 11, 22, 37, 56, 79, 106, 137, 172, 211, 254, 301, 352, 407, 466, 529, 596, 667, 742, 821, 904, 991, 1082, 1177, 1276, 1379, 1486, 1597, 1712
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000730";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_730(n)
    }
}

const fn quad_730(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 1 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000730>();
}
