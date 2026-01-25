/// a(n) = n^3 + 1*n^2 + 2*n + 0
/// https://oeis.org/A000211

pub struct A000211;

impl crate::traits::IntegerSequence for A000211 {
    const NAME: &str = "a(n) = n^3 + 1*n^2 + 2*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 4, 16, 42, 88, 160, 264, 406, 592, 828, 1120, 1474, 1896, 2392, 2968, 3630, 4384, 5236, 6192, 7258, 8440, 9744, 11176, 12742, 14448
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000211";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_211(n)
    }
}

const fn poly_211(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n * n + 2 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000211>();
}
