/// a(n) = 2*n^2 + 3*n + 2
/// https://oeis.org/A000737

pub struct A000737;

impl crate::traits::IntegerSequence for A000737 {
    const NAME: &str = "a(n) = 2*n^2 + 3*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 7, 16, 29, 46, 67, 92, 121, 154, 191, 232, 277, 326, 379, 436, 497, 562, 631, 704, 781, 862, 947, 1036, 1129, 1226, 1327, 1432, 1541, 1654, 1771
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000737";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_737(n)
    }
}

const fn quad_737(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 3 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000737>();
}
