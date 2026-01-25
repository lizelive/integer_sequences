/// a(n) = 2*n^2 + 3*n + 1
/// https://oeis.org/A000732

pub struct A000732;

impl crate::traits::IntegerSequence for A000732 {
    const NAME: &str = "a(n) = 2*n^2 + 3*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 15, 28, 45, 66, 91, 120, 153, 190, 231, 276, 325, 378, 435, 496, 561, 630, 703, 780, 861, 946, 1035, 1128, 1225, 1326, 1431, 1540, 1653, 1770
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000732";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_732(n)
    }
}

const fn quad_732(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 3 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000732>();
}
