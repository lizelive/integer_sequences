/// a(n) = 4*n^2 + 5*n + 1
/// https://oeis.org/A000784

pub struct A000784;

impl crate::traits::IntegerSequence for A000784 {
    const NAME: &str = "a(n) = 4*n^2 + 5*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 10, 27, 52, 85, 126, 175, 232, 297, 370, 451, 540, 637, 742, 855, 976, 1105, 1242, 1387, 1540, 1701, 1870, 2047, 2232, 2425, 2626, 2835, 3052, 3277, 3510
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000784";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_784(n)
    }
}

const fn quad_784(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 5 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000784>();
}
