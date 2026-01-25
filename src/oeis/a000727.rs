/// a(n) = 2*n^2 + 3*n + 0
/// https://oeis.org/A000727

pub struct A000727;

impl crate::traits::IntegerSequence for A000727 {
    const NAME: &str = "a(n) = 2*n^2 + 3*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 5, 14, 27, 44, 65, 90, 119, 152, 189, 230, 275, 324, 377, 434, 495, 560, 629, 702, 779, 860, 945, 1034, 1127, 1224, 1325, 1430, 1539, 1652, 1769
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000727";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_727(n)
    }
}

const fn quad_727(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 3 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000727>();
}
