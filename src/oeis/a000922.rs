/// a(n) = 3*n^2 + 2
/// https://oeis.org/A000922

pub struct A000922;

impl crate::traits::IntegerSequence for A000922 {
    const NAME: &str = "a(n) = 3*n^2 + 2";

    const HEAD: &[crate::Value] = &[
        2, 5, 14, 29, 50, 77, 110, 149, 194, 245, 302, 365, 434, 509, 590, 677, 770, 869, 974, 1085, 1202, 1325, 1454, 1589, 1730, 1877, 2030, 2189, 2354, 2525
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000922";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_922(n)
    }
}

const fn sq_922(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 2 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000922>();
}
