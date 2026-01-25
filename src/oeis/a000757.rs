/// a(n) = 3*n^2 + 3*n + 1
/// https://oeis.org/A000757

pub struct A000757;

impl crate::traits::IntegerSequence for A000757 {
    const NAME: &str = "a(n) = 3*n^2 + 3*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 7, 19, 37, 61, 91, 127, 169, 217, 271, 331, 397, 469, 547, 631, 721, 817, 919, 1027, 1141, 1261, 1387, 1519, 1657, 1801, 1951, 2107, 2269, 2437, 2611
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000757";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_757(n)
    }
}

const fn quad_757(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 3 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000757>();
}
