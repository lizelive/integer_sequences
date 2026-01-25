/// a(n) = n^2 + 5*n + 4
/// https://oeis.org/A000195

pub struct A000195;

impl crate::traits::IntegerSequence for A000195 {
    const NAME: &str = "a(n) = n^2 + 5*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 10, 18, 28, 40, 54, 70, 88, 108, 130, 154, 180, 208, 238, 270, 304, 340, 378, 418, 460, 504, 550, 598, 648, 700
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000195";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_195(n)
    }
}

const fn poly_195(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 5 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000195>();
}
