/// a(n) = n^2 + 9*n + 0
/// https://oeis.org/A000159

pub struct A000159;

impl crate::traits::IntegerSequence for A000159 {
    const NAME: &str = "a(n) = n^2 + 9*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 10, 22, 36, 52, 70, 90, 112, 136, 162, 190, 220, 252, 286, 322, 360, 400, 442, 486, 532, 580, 630, 682, 736, 792
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000159";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_159(n)
    }
}

const fn poly_159(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 9 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000159>();
}
