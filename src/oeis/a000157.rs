/// a(n) = n^2 + 7*n + 0
/// https://oeis.org/A000157

pub struct A000157;

impl crate::traits::IntegerSequence for A000157 {
    const NAME: &str = "a(n) = n^2 + 7*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 8, 18, 30, 44, 60, 78, 98, 120, 144, 170, 198, 228, 260, 294, 330, 368, 408, 450, 494, 540, 588, 638, 690, 744
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000157";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_157(n)
    }
}

const fn poly_157(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 7 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000157>();
}
