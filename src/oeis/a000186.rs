/// a(n) = n^2 + 6*n + 3
/// https://oeis.org/A000186

pub struct A000186;

impl crate::traits::IntegerSequence for A000186 {
    const NAME: &str = "a(n) = n^2 + 6*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 10, 19, 30, 43, 58, 75, 94, 115, 138, 163, 190, 219, 250, 283, 318, 355, 394, 435, 478, 523, 570, 619, 670, 723
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000186";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_186(n)
    }
}

const fn poly_186(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 6 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000186>();
}
