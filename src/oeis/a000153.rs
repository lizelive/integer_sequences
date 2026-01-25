/// a(n) = n^2 + 3*n + 0
/// https://oeis.org/A000153

pub struct A000153;

impl crate::traits::IntegerSequence for A000153 {
    const NAME: &str = "a(n) = n^2 + 3*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 4, 10, 18, 28, 40, 54, 70, 88, 108, 130, 154, 180, 208, 238, 270, 304, 340, 378, 418, 460, 504, 550, 598, 648
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000153";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_153(n)
    }
}

const fn poly_153(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 3 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000153>();
}
