/// a(n) = n^3 + 1*n^2 + 0*n + 0
/// https://oeis.org/A000201

pub struct A000201;

impl crate::traits::IntegerSequence for A000201 {
    const NAME: &str = "a(n) = n^3 + 1*n^2 + 0*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 2, 12, 36, 80, 150, 252, 392, 576, 810, 1100, 1452, 1872, 2366, 2940, 3600, 4352, 5202, 6156, 7220, 8400, 9702, 11132, 12696, 14400
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000201";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_201(n)
    }
}

const fn poly_201(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n * n + 0 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000201>();
}
