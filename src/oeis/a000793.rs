/// a(n) = 4*n^2 + 4*n + 3
/// https://oeis.org/A000793

pub struct A000793;

impl crate::traits::IntegerSequence for A000793 {
    const NAME: &str = "a(n) = 4*n^2 + 4*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 11, 27, 51, 83, 123, 171, 227, 291, 363, 443, 531, 627, 731, 843, 963, 1091, 1227, 1371, 1523, 1683, 1851, 2027, 2211, 2403, 2603, 2811, 3027, 3251, 3483
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000793";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_793(n)
    }
}

const fn quad_793(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 4 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000793>();
}
