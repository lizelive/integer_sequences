/// a(n) = 3*n^2 + 5*n + 0
/// https://oeis.org/A000754

pub struct A000754;

impl crate::traits::IntegerSequence for A000754 {
    const NAME: &str = "a(n) = 3*n^2 + 5*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 8, 22, 42, 68, 100, 138, 182, 232, 288, 350, 418, 492, 572, 658, 750, 848, 952, 1062, 1178, 1300, 1428, 1562, 1702, 1848, 2000, 2158, 2322, 2492, 2668
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000754";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_754(n)
    }
}

const fn quad_754(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 5 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000754>();
}
